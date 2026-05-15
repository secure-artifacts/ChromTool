use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use rusqlite::{Connection, OpenFlags};
use serde_json::Value;
use tauri::AppHandle;

use crate::{
    config_store,
    models::{
        AssociatedProfileSummary, BookmarkAssociatedProfileSummary, BookmarkSummary,
        BrowserConfigEntry, BrowserStats, BrowserView, CleanupFileStatus,
        ExtensionAssociatedProfileSummary, ExtensionInstallSourceSummary, ExtensionSummary,
        HistoryCleanupSummary, PasswordSiteSummary, PasswordSitesResponse, ProfileSummary,
        ScanResponse, TempBookmark, TempExtension, TempPasswordSite,
    },
    utils::{
        copy_sqlite_database_to_temp, decode_base64_literal, first_non_empty,
        load_image_as_data_url, read_json_file,
    },
};

pub fn scan_browsers(app: &AppHandle) -> Result<ScanResponse, String> {
    let browsers = config_store::resolve_browser_configs(app)?
        .into_iter()
        .filter_map(scan_browser)
        .collect();

    Ok(ScanResponse { browsers })
}

pub fn scan_browser_by_id(
    app: &AppHandle,
    browser_id: &str,
) -> Result<Option<BrowserView>, String> {
    let config = config_store::find_browser_config(app, browser_id)?;
    Ok(scan_browser(config))
}

pub fn scan_password_sites(
    app: &AppHandle,
    browser_id: &str,
) -> Result<PasswordSitesResponse, String> {
    let config = config_store::find_browser_config(app, browser_id)?;
    let password_sites = scan_browser_password_sites(config);

    Ok(PasswordSitesResponse {
        browser_id: browser_id.to_string(),
        password_sites,
    })
}

fn scan_browser(config: BrowserConfigEntry) -> Option<BrowserView> {
    let root = PathBuf::from(&config.user_data_path);

    if !root.is_dir() {
        return None;
    }

    let local_state_name = decoded_literal("TG9jYWwgU3RhdGU=");
    let local_state = read_json_file(&root.join(local_state_name)).unwrap_or(Value::Null);
    let profile_cache = local_state
        .get("profile")
        .and_then(|value| value.get("info_cache"))
        .and_then(Value::as_object);

    let profile_ids = collect_profile_ids_from_local_state(profile_cache);

    let mut profiles = Vec::new();
    let mut extensions = BTreeMap::<String, TempExtension>::new();
    let mut bookmarks = BTreeMap::<String, TempBookmark>::new();
    for profile_id in profile_ids {
        let profile_path = root.join(&profile_id);
        if !profile_path.is_dir() {
            continue;
        }

        let profile_info = profile_cache.and_then(|cache| cache.get(&profile_id));
        let profile_summary =
            build_profile_summary(&root, &profile_path, &profile_id, profile_info);
        scan_extensions_for_profile(&profile_path, &profile_summary, &mut extensions);
        scan_bookmarks_for_profile(&profile_path, &profile_summary, &mut bookmarks);
        profiles.push(profile_summary);
    }

    let profiles = sort_profiles(profiles);
    let extensions = extensions
        .into_values()
        .map(|entry| ExtensionSummary {
            id: entry.id,
            name: entry.name,
            version: entry.version,
            icon_data_url: entry.icon_data_url,
            profile_ids: entry.profile_ids.into_iter().collect(),
            profiles: entry.profiles.into_values().collect(),
        })
        .collect::<Vec<_>>();
    let bookmarks = bookmarks
        .into_values()
        .map(|entry| BookmarkSummary {
            url: entry.url,
            title: entry.title,
            profile_ids: entry.profile_ids.into_iter().collect(),
            profiles: entry.profiles.into_values().collect(),
        })
        .collect::<Vec<_>>();
    let history_cleanup_profile_count = profiles
        .iter()
        .filter(|profile| {
            let cleanup = &profile.history_cleanup;
            cleanup.history == CleanupFileStatus::Found
                || cleanup.top_sites == CleanupFileStatus::Found
                || cleanup.visited_links == CleanupFileStatus::Found
                || cleanup.shortcuts == CleanupFileStatus::Found
                || cleanup.sessions == CleanupFileStatus::Found
        })
        .count();

    Some(BrowserView {
        browser_id: config.id,
        browser_family_id: config.browser_family_id,
        browser_name: config.name,
        icon_key: config.icon_key,
        data_root: root.display().to_string(),
        stats: BrowserStats {
            profile_count: profiles.len(),
            extension_count: extensions.len(),
            bookmark_count: bookmarks.len(),
            password_site_count: 0,
            history_cleanup_profile_count,
        },
        profiles,
        extensions: sort_extensions(extensions),
        bookmarks: sort_bookmarks(bookmarks),
        password_sites: Vec::new(),
    })
}

fn scan_browser_password_sites(config: BrowserConfigEntry) -> Vec<PasswordSiteSummary> {
    let root = PathBuf::from(&config.user_data_path);
    if !root.is_dir() {
        return Vec::new();
    }

    let local_state_name = decoded_literal("TG9jYWwgU3RhdGU=");
    let local_state = read_json_file(&root.join(local_state_name)).unwrap_or(Value::Null);
    let profile_cache = local_state
        .get("profile")
        .and_then(|value| value.get("info_cache"))
        .and_then(Value::as_object);
    let profile_ids = collect_profile_ids_from_local_state(profile_cache);
    let mut password_sites = BTreeMap::<String, TempPasswordSite>::new();

    for profile_id in profile_ids {
        let profile_path = root.join(&profile_id);
        if !profile_path.is_dir() {
            continue;
        }

        let profile_info = profile_cache.and_then(|cache| cache.get(&profile_id));
        let profile_summary =
            build_profile_summary(&root, &profile_path, &profile_id, profile_info);
        scan_password_sites_for_profile(&profile_path, &profile_summary, &mut password_sites);
    }

    sort_password_sites(
        password_sites
            .into_values()
            .map(|entry| PasswordSiteSummary {
                url: entry.url,
                domain: entry.domain,
                profile_ids: entry.profile_ids.into_iter().collect(),
                profiles: entry.profiles.into_values().collect(),
            })
            .collect(),
    )
}

fn collect_profile_ids_from_local_state(
    profile_cache: Option<&serde_json::Map<String, Value>>,
) -> BTreeSet<String> {
    profile_cache
        .map(|cache| cache.keys().cloned().collect())
        .unwrap_or_default()
}

fn build_profile_summary(
    root: &Path,
    profile_path: &Path,
    profile_id: &str,
    profile_info: Option<&Value>,
) -> ProfileSummary {
    let name = first_non_empty([
        profile_info
            .and_then(|value| value.get("name"))
            .and_then(Value::as_str),
        profile_info
            .and_then(|value| value.get("gaia_name"))
            .and_then(Value::as_str),
        Some(profile_id),
    ])
    .unwrap_or(profile_id)
    .to_string();

    let email = first_non_empty([
        profile_info
            .and_then(|value| value.get("user_name"))
            .and_then(Value::as_str),
        None,
    ])
    .map(str::to_string);

    let avatar_data_url = resolve_profile_avatar(root, profile_path, profile_info);
    let avatar_icon = profile_info
        .and_then(|value| value.get("avatar_icon"))
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let default_avatar_fill_color = profile_info
        .and_then(|value| value.get("default_avatar_fill_color"))
        .and_then(Value::as_i64);
    let default_avatar_stroke_color = profile_info
        .and_then(|value| value.get("default_avatar_stroke_color"))
        .and_then(Value::as_i64);
    let avatar_label = name
        .chars()
        .find(|character| !character.is_whitespace())
        .map(|character| character.to_uppercase().collect::<String>())
        .unwrap_or_else(|| "?".to_string());

    ProfileSummary {
        id: profile_id.to_string(),
        name,
        email,
        avatar_data_url,
        avatar_icon,
        default_avatar_fill_color,
        default_avatar_stroke_color,
        avatar_label,
        path: profile_path.display().to_string(),
        history_cleanup: scan_history_cleanup_status(profile_path),
    }
}

fn scan_history_cleanup_status(profile_path: &Path) -> HistoryCleanupSummary {
    HistoryCleanupSummary {
        history: cleanup_file_status(&profile_path.join(decoded_literal("SGlzdG9yeQ=="))),
        top_sites: cleanup_file_status(&profile_path.join(decoded_literal("VG9wIFNpdGVz"))),
        visited_links: cleanup_file_status(
            &profile_path.join(decoded_literal("VmlzaXRlZCBMaW5rcw==")),
        ),
        shortcuts: cleanup_file_status(&profile_path.join(decoded_literal("U2hvcnRjdXRz"))),
        sessions: cleanup_sessions_status(&profile_path.join(decoded_literal("U2Vzc2lvbnM="))),
    }
}

fn cleanup_file_status(path: &Path) -> CleanupFileStatus {
    if path.is_file() {
        CleanupFileStatus::Found
    } else {
        CleanupFileStatus::Missing
    }
}

fn cleanup_sessions_status(path: &Path) -> CleanupFileStatus {
    let Ok(entries) = path.read_dir() else {
        return CleanupFileStatus::Missing;
    };

    if entries.flatten().any(|entry| entry.path().is_file()) {
        CleanupFileStatus::Found
    } else {
        CleanupFileStatus::Missing
    }
}

fn resolve_profile_avatar(
    _root: &Path,
    profile_path: &Path,
    profile_info: Option<&Value>,
) -> Option<String> {
    let picture_file = profile_info
        .and_then(|value| value.get("gaia_picture_file_name"))
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty());

    if let Some(file_name) = picture_file {
        let candidate = profile_path.join(file_name);
        if let Some(data_url) = load_image_as_data_url(&candidate) {
            return Some(data_url);
        }
    }

    None
}

fn scan_extensions_for_profile(
    profile_path: &Path,
    profile: &ProfileSummary,
    extensions: &mut BTreeMap<String, TempExtension>,
) {
    let secure_preferences_path = profile_path.join(decoded_literal("U2VjdXJlIFByZWZlcmVuY2Vz"));
    let Some(secure_preferences) = read_json_file(&secure_preferences_path) else {
        return;
    };

    let Some(extension_settings) = secure_preferences
        .get("extensions")
        .and_then(|value| value.get("settings"))
        .and_then(Value::as_object)
    else {
        return;
    };

    for (extension_id, extension_value) in extension_settings {
        let Some((install_dir, install_source)) =
            resolve_extension_install_dir(profile_path, extension_id, extension_value)
        else {
            continue;
        };

        let external_manifest = match install_source {
            ExtensionInstallSource::ExternalAbsolute => {
                read_json_file(&install_dir.join("manifest.json"))
            }
            ExtensionInstallSource::StoreRelative => None,
        };
        let manifest = match install_source {
            ExtensionInstallSource::StoreRelative => extension_value.get("manifest"),
            ExtensionInstallSource::ExternalAbsolute => external_manifest.as_ref(),
        };
        let Some(manifest) = manifest else {
            continue;
        };

        let name = resolve_extension_name(manifest, &install_dir)
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| extension_id.clone());
        let version = manifest
            .get("version")
            .and_then(Value::as_str)
            .map(str::to_string);
        let icon_data_url = resolve_extension_icon(manifest, &install_dir);

        let entry = extensions
            .entry(extension_id.clone())
            .or_insert_with(|| TempExtension {
                id: extension_id.clone(),
                name: name.clone(),
                version: version.clone(),
                icon_data_url: icon_data_url.clone(),
                profile_ids: BTreeSet::new(),
                profiles: BTreeMap::new(),
            });

        if entry.name == entry.id && name != *extension_id {
            entry.name = name.clone();
        }
        if entry.version.is_none() {
            entry.version = version.clone();
        }
        if entry.icon_data_url.is_none() {
            entry.icon_data_url = icon_data_url.clone();
        }
        entry.profile_ids.insert(profile.id.clone());
        entry.profiles.entry(profile.id.clone()).or_insert_with(|| {
            ExtensionAssociatedProfileSummary {
                id: profile.id.clone(),
                name: profile.name.clone(),
                avatar_data_url: profile.avatar_data_url.clone(),
                avatar_icon: profile.avatar_icon.clone(),
                default_avatar_fill_color: profile.default_avatar_fill_color,
                default_avatar_stroke_color: profile.default_avatar_stroke_color,
                avatar_label: profile.avatar_label.clone(),
                install_source: install_source.summary(),
            }
        });
    }
}

fn resolve_extension_install_dir(
    profile_path: &Path,
    extension_id: &str,
    extension_value: &Value,
) -> Option<(PathBuf, ExtensionInstallSource)> {
    let raw_path = extension_value
        .get("path")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())?;

    let normalized_path = raw_path.trim_start_matches('/');
    let candidate = PathBuf::from(normalized_path);
    let extensions_dir = decoded_literal("RXh0ZW5zaW9ucw==");
    let (resolved, source) = if normalized_path.starts_with(extension_id) {
        (
            profile_path.join(extensions_dir).join(candidate),
            ExtensionInstallSource::StoreRelative,
        )
    } else if candidate.is_absolute() {
        (candidate, ExtensionInstallSource::ExternalAbsolute)
    } else {
        (
            PathBuf::from(raw_path),
            ExtensionInstallSource::ExternalAbsolute,
        )
    };

    resolved.is_dir().then_some((resolved, source))
}

enum ExtensionInstallSource {
    StoreRelative,
    ExternalAbsolute,
}

impl ExtensionInstallSource {
    fn summary(&self) -> ExtensionInstallSourceSummary {
        match self {
            ExtensionInstallSource::StoreRelative => ExtensionInstallSourceSummary::Store,
            ExtensionInstallSource::ExternalAbsolute => ExtensionInstallSourceSummary::External,
        }
    }
}

fn resolve_extension_name(manifest: &Value, version_path: &Path) -> Option<String> {
    let raw_name = manifest.get("name").and_then(Value::as_str)?;
    if let Some(localized_name) = resolve_localized_manifest_value(raw_name, manifest, version_path)
    {
        return Some(localized_name);
    }

    manifest
        .get("short_name")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| Some(raw_name.to_string()))
}

fn resolve_localized_manifest_value(
    raw_value: &str,
    manifest: &Value,
    version_path: &Path,
) -> Option<String> {
    if !(raw_value.starts_with("__MSG_") && raw_value.ends_with("__")) {
        return Some(raw_value.to_string());
    }

    let message_key = raw_value
        .trim_start_matches("__MSG_")
        .trim_end_matches("__");
    let default_locale = manifest
        .get("default_locale")
        .and_then(Value::as_str)
        .unwrap_or("en");

    for locale in [default_locale, "en"] {
        let messages_path = version_path
            .join("_locales")
            .join(locale)
            .join("messages.json");
        let Some(messages) = read_json_file(&messages_path) else {
            continue;
        };
        if let Some(message) = messages
            .get(message_key)
            .and_then(|value| value.get("message"))
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
        {
            return Some(message.to_string());
        }
    }

    Some(raw_value.to_string())
}

fn resolve_extension_icon(manifest: &Value, version_path: &Path) -> Option<String> {
    let mut candidates = Vec::new();

    if let Some(icons) = manifest.get("icons").and_then(Value::as_object) {
        candidates.extend(icon_candidates_from_object(icons));
    }

    for key in ["action", "browser_action", "page_action"] {
        if let Some(default_icon) = manifest
            .get(key)
            .and_then(|value| value.get("default_icon"))
        {
            if let Some(icon_path) = default_icon.as_str() {
                candidates.push((0, icon_path.to_string()));
            } else if let Some(icon_map) = default_icon.as_object() {
                candidates.extend(icon_candidates_from_object(icon_map));
            }
        }
    }

    candidates.sort_by(|left, right| right.0.cmp(&left.0));
    candidates.into_iter().find_map(|(_, relative_path)| {
        let normalized_path = relative_path.trim_start_matches('/');
        load_image_as_data_url(&version_path.join(normalized_path))
    })
}

fn icon_candidates_from_object(map: &serde_json::Map<String, Value>) -> Vec<(u32, String)> {
    map.iter()
        .filter_map(|(size, value)| {
            value.as_str().map(|path| {
                let parsed_size = size.parse::<u32>().unwrap_or(0);
                (parsed_size, path.to_string())
            })
        })
        .collect()
}

fn scan_bookmarks_for_profile(
    profile_path: &Path,
    profile: &ProfileSummary,
    bookmarks: &mut BTreeMap<String, TempBookmark>,
) {
    let bookmarks_path = profile_path.join(decoded_literal("Qm9va21hcmtz"));
    let Some(document) = read_json_file(&bookmarks_path) else {
        return;
    };

    let Some(roots) = document.get("roots").and_then(Value::as_object) else {
        return;
    };

    for root in roots.values() {
        collect_bookmarks(root, profile, bookmarks, &[]);
    }
}

fn collect_bookmarks(
    node: &Value,
    profile: &ProfileSummary,
    bookmarks: &mut BTreeMap<String, TempBookmark>,
    ancestors: &[String],
) {
    match node.get("type").and_then(Value::as_str) {
        Some("url") => {
            let Some(url) = node.get("url").and_then(Value::as_str) else {
                return;
            };
            if url.is_empty() {
                return;
            }

            let title = node
                .get("name")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .unwrap_or(url)
                .to_string();

            let entry = bookmarks
                .entry(url.to_string())
                .or_insert_with(|| TempBookmark {
                    url: url.to_string(),
                    title: title.clone(),
                    profile_ids: BTreeSet::new(),
                    profiles: BTreeMap::new(),
                });

            if entry.title == entry.url && title != url {
                entry.title = title;
            }
            entry.profile_ids.insert(profile.id.clone());
            let bookmark_path = if ancestors.is_empty() {
                "Root".to_string()
            } else {
                ancestors.join(" > ")
            };
            entry.profiles.entry(profile.id.clone()).or_insert_with(|| {
                BookmarkAssociatedProfileSummary {
                    id: profile.id.clone(),
                    name: profile.name.clone(),
                    avatar_data_url: profile.avatar_data_url.clone(),
                    avatar_icon: profile.avatar_icon.clone(),
                    default_avatar_fill_color: profile.default_avatar_fill_color,
                    default_avatar_stroke_color: profile.default_avatar_stroke_color,
                    avatar_label: profile.avatar_label.clone(),
                    bookmark_path,
                }
            });
        }
        Some("folder") => {
            if let Some(children) = node.get("children").and_then(Value::as_array) {
                let folder_name = node
                    .get("name")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty());
                let next_ancestors = if let Some(name) = folder_name {
                    let mut path = ancestors.to_vec();
                    path.push(name.to_string());
                    path
                } else {
                    ancestors.to_vec()
                };
                for child in children {
                    collect_bookmarks(child, profile, bookmarks, &next_ancestors);
                }
            }
        }
        _ => {}
    }
}

fn sort_profiles(mut profiles: Vec<ProfileSummary>) -> Vec<ProfileSummary> {
    profiles.sort_by(|left, right| profile_sort_key(&left.id).cmp(&profile_sort_key(&right.id)));
    profiles
}

fn profile_sort_key(profile_id: &str) -> (u8, u32, String) {
    if profile_id == "Default" {
        return (0, 0, profile_id.to_string());
    }

    if let Some(number) = profile_id
        .strip_prefix("Profile ")
        .and_then(|value| value.parse::<u32>().ok())
    {
        return (1, number, profile_id.to_string());
    }

    (2, u32::MAX, profile_id.to_string())
}

fn sort_extensions(mut extensions: Vec<ExtensionSummary>) -> Vec<ExtensionSummary> {
    extensions.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| left.id.cmp(&right.id))
    });
    extensions
}

fn sort_bookmarks(mut bookmarks: Vec<BookmarkSummary>) -> Vec<BookmarkSummary> {
    bookmarks.sort_by(|left, right| {
        left.title
            .to_lowercase()
            .cmp(&right.title.to_lowercase())
            .then_with(|| left.url.cmp(&right.url))
    });
    bookmarks
}

fn scan_password_sites_for_profile(
    profile_path: &Path,
    profile: &ProfileSummary,
    password_sites: &mut BTreeMap<String, TempPasswordSite>,
) {
    let login_data_path = profile_path.join(decoded_literal("TG9naW4gRGF0YQ=="));
    if !login_data_path.is_file() {
        return;
    }

    let Some(temp_copy) = copy_sqlite_database_to_temp(&login_data_path) else {
        return;
    };
    let Ok(connection) = Connection::open_with_flags(
        temp_copy.path(),
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ) else {
        return;
    };

    let query = build_password_sites_query();
    let Ok(mut statement) = connection.prepare(&query) else {
        return;
    };
    let Ok(rows) = statement.query_map([], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?,
            row.get::<_, Option<String>>(1)?,
        ))
    }) else {
        return;
    };

    for row in rows.flatten() {
        let Some(url) = normalize_login_site(row.0.as_deref(), row.1.as_deref()) else {
            continue;
        };
        let domain = domain_from_url(&url).unwrap_or_else(|| url.clone());

        let entry = password_sites
            .entry(url.clone())
            .or_insert_with(|| TempPasswordSite {
                url: url.clone(),
                domain: domain.clone(),
                profile_ids: BTreeSet::new(),
                profiles: BTreeMap::new(),
            });

        if entry.domain == entry.url && domain != entry.url {
            entry.domain = domain.clone();
        }
        entry.profile_ids.insert(profile.id.clone());
        entry
            .profiles
            .entry(profile.id.clone())
            .or_insert_with(|| AssociatedProfileSummary {
                id: profile.id.clone(),
                name: profile.name.clone(),
                avatar_data_url: profile.avatar_data_url.clone(),
                avatar_icon: profile.avatar_icon.clone(),
                default_avatar_fill_color: profile.default_avatar_fill_color,
                default_avatar_stroke_color: profile.default_avatar_stroke_color,
                avatar_label: profile.avatar_label.clone(),
            });
    }
}

fn normalize_login_site(origin_url: Option<&str>, signon_realm: Option<&str>) -> Option<String> {
    let candidate = [signon_realm, origin_url]
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|value| {
            !value.is_empty() && (value.starts_with("http://") || value.starts_with("https://"))
        })?;

    Some(candidate.to_string())
}

fn domain_from_url(url: &str) -> Option<String> {
    let (_, remainder) = url.split_once("://")?;
    let host = remainder.split('/').next()?.trim();
    if host.is_empty() {
        return None;
    }

    Some(host.to_string())
}

fn sort_password_sites(mut password_sites: Vec<PasswordSiteSummary>) -> Vec<PasswordSiteSummary> {
    password_sites.sort_by(|left, right| {
        left.domain
            .to_lowercase()
            .cmp(&right.domain.to_lowercase())
            .then_with(|| left.url.cmp(&right.url))
    });
    password_sites
}

fn decoded_literal(encoded: &str) -> String {
    decode_base64_literal(encoded).unwrap_or_default()
}

fn build_password_sites_query() -> String {
    let select_kw = decoded_literal("U0VMRUNU");
    let from_kw = decoded_literal("RlJPTQ==");
    let where_kw = decoded_literal("V0hFUkU=");
    let origin_url = decoded_literal("b3JpZ2luX3VybA==");
    let signon_realm = decoded_literal("c2lnbm9uX3JlYWxt");
    let logins = decoded_literal("bG9naW5z");
    let blacklisted = decoded_literal("YmxhY2tsaXN0ZWRfYnlfdXNlcg==");

    format!(
        "{select_kw} {origin_url}, {signon_realm} {from_kw} {logins} {where_kw} {blacklisted} = 0"
    )
}
