use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    config_store,
    models::{
        BookmarkRemovalRequest, BrowserConfigListResponse, CleanupHistoryInput,
        CleanupHistoryResponse, CleanupHistoryResult, CreateCustomBrowserConfigInput,
        ExtensionInstallSourceSummary, RemoveBookmarkResult, RemoveBookmarksInput,
        RemoveBookmarksResponse, RemoveExtensionResult, RemoveExtensionsInput,
        RemoveExtensionsResponse, PasswordSitesResponse, ScanResponse,
    },
    scanner,
    utils::decode_base64_literal,
};
use tauri::{async_runtime, AppHandle};
use serde_json::Value;

#[tauri::command]
pub async fn scan_browsers(app: AppHandle) -> Result<ScanResponse, String> {
    async_runtime::spawn_blocking(move || scanner::scan_browsers(&app))
        .await
        .map_err(|error| format!("Failed to join browser scan task: {error}"))?
}

#[tauri::command]
pub async fn scan_password_sites(
    app: AppHandle,
    browser_id: String,
) -> Result<PasswordSitesResponse, String> {
    async_runtime::spawn_blocking(move || scanner::scan_password_sites(&app, &browser_id))
        .await
        .map_err(|error| format!("Failed to join password site scan task: {error}"))?
}

#[tauri::command]
pub fn list_browser_configs(app: AppHandle) -> Result<BrowserConfigListResponse, String> {
    config_store::load_browser_config_list(&app)
}

#[tauri::command]
pub fn create_custom_browser_config(
    app: AppHandle,
    input: CreateCustomBrowserConfigInput,
) -> Result<BrowserConfigListResponse, String> {
    config_store::create_custom_browser_config(&app, input)
}

#[tauri::command]
pub fn delete_custom_browser_config(
    app: AppHandle,
    config_id: String,
) -> Result<BrowserConfigListResponse, String> {
    config_store::delete_custom_browser_config(&app, &config_id)
}

#[tauri::command]
pub fn open_browser_profile(
    app: AppHandle,
    browser_id: String,
    profile_id: String,
) -> Result<(), String> {
    let config = config_store::find_browser_config(&app, &browser_id)?;
    let executable_path = PathBuf::from(&config.executable_path);
    let user_data_dir = PathBuf::from(&config.user_data_path);
    let profile_directory = user_data_dir.join(&profile_id);

    if !user_data_dir.is_dir() {
        return Err(format!(
            "User data directory does not exist: {}",
            user_data_dir.display()
        ));
    }

    if !profile_directory.is_dir() {
        return Err(format!(
            "Profile directory does not exist: {}",
            profile_directory.display()
        ));
    }

    spawn_browser_process(executable_path, user_data_dir, profile_id)
}

#[tauri::command]
pub fn cleanup_history_files(
    app: AppHandle,
    input: CleanupHistoryInput,
) -> Result<CleanupHistoryResponse, String> {
    let config = config_store::find_browser_config(&app, &input.browser_id)?;
    let user_data_dir = PathBuf::from(&config.user_data_path);

    if !user_data_dir.is_dir() {
        return Err(format!(
            "User data directory does not exist: {}",
            user_data_dir.display()
        ));
    }

    let mut results = Vec::new();
    for profile_id in input.profile_ids {
        let profile_path = user_data_dir.join(&profile_id);
        let result = cleanup_profile_history_files(&profile_path, &profile_id);
        results.push(result);
    }

    Ok(CleanupHistoryResponse { results })
}

#[tauri::command]
pub fn remove_extensions(
    app: AppHandle,
    input: RemoveExtensionsInput,
) -> Result<RemoveExtensionsResponse, String> {
    let config = config_store::find_browser_config(&app, &input.browser_id)?;
    let user_data_dir = PathBuf::from(&config.user_data_path);

    if !user_data_dir.is_dir() {
        return Err(format!(
            "User data directory does not exist: {}",
            user_data_dir.display()
        ));
    }

    let mut results = Vec::new();
    for removal in input.removals {
        for profile_id in removal.profile_ids {
            results.push(remove_extension_from_profile(
                &user_data_dir.join(&profile_id),
                &removal.extension_id,
                &profile_id,
            ));
        }
    }

    Ok(RemoveExtensionsResponse { results })
}

#[tauri::command]
pub fn remove_bookmarks(
    app: AppHandle,
    input: RemoveBookmarksInput,
) -> Result<RemoveBookmarksResponse, String> {
    let config = config_store::find_browser_config(&app, &input.browser_id)?;
    let user_data_dir = PathBuf::from(&config.user_data_path);

    if !user_data_dir.is_dir() {
        return Err(format!(
            "User data directory does not exist: {}",
            user_data_dir.display()
        ));
    }

    let mut results = Vec::new();
    for removal in input.removals {
        for profile_id in &removal.profile_ids {
            results.push(remove_bookmark_from_profile(
                &user_data_dir.join(profile_id),
                &removal,
                profile_id,
            ));
        }
    }

    Ok(RemoveBookmarksResponse { results })
}

fn spawn_browser_process(
    executable_path: PathBuf,
    user_data_dir: PathBuf,
    profile_id: String,
) -> Result<(), String> {
    Command::new(&executable_path)
        .arg(format!("--user-data-dir={}", user_data_dir.display()))
        .arg(format!("--profile-directory={profile_id}"))
        .arg("https://www.google.com")
        .spawn()
        .map(|_| ())
        .map_err(|error| {
            format!(
                "Failed to open browser profile with executable {}: {error}",
                executable_path.display()
            )
        })
}

fn cleanup_profile_history_files(profile_path: &Path, profile_id: &str) -> CleanupHistoryResult {
    if !profile_path.is_dir() {
        return CleanupHistoryResult {
            profile_id: profile_id.to_string(),
            deleted_files: Vec::new(),
            skipped_files: Vec::new(),
            error: Some(format!(
                "Profile directory does not exist: {}",
                profile_path.display()
            )),
        };
    }

    let mut deleted_files = Vec::new();
    let mut skipped_files = Vec::new();

    for file_name in cleanup_file_names() {
        let file_path = profile_path.join(&file_name);
        if !file_path.exists() {
            skipped_files.push(file_name);
            continue;
        }

        if let Err(error) = fs::remove_file(&file_path) {
            return CleanupHistoryResult {
                profile_id: profile_id.to_string(),
                deleted_files,
                skipped_files,
                error: Some(format!("Failed to delete {}: {error}", file_path.display())),
            };
        }

        deleted_files.push(file_name);
        remove_sidecar_files(&file_path);
    }

    let sessions_name = decoded_literal("U2Vzc2lvbnM=");
    let sessions_directory = profile_path.join(&sessions_name);
    match cleanup_sessions_directory(&sessions_directory) {
        Ok(session_deleted) => {
            if session_deleted {
                deleted_files.push(sessions_name.clone());
            } else {
                skipped_files.push(sessions_name);
            }
        }
        Err(error) => {
            return CleanupHistoryResult {
                profile_id: profile_id.to_string(),
                deleted_files,
                skipped_files,
                error: Some(format!(
                    "Failed to clean {}: {error}",
                    sessions_directory.display()
                )),
            };
        }
    }

    CleanupHistoryResult {
        profile_id: profile_id.to_string(),
        deleted_files,
        skipped_files,
        error: None,
    }
}

fn remove_extension_from_profile(
    profile_path: &Path,
    extension_id: &str,
    profile_id: &str,
) -> RemoveExtensionResult {
    if !profile_path.is_dir() {
        return RemoveExtensionResult {
            extension_id: extension_id.to_string(),
            profile_id: profile_id.to_string(),
            removed_files: Vec::new(),
            skipped_files: Vec::new(),
            error: Some(format!(
                "Profile directory does not exist: {}",
                profile_path.display()
            )),
        };
    }

    let secure_preferences_path = profile_path.join(decoded_literal("U2VjdXJlIFByZWZlcmVuY2Vz"));
    let preferences_path = profile_path.join(decoded_literal("UHJlZmVyZW5jZXM="));
    let mut removed_files = Vec::new();
    let mut skipped_files = Vec::new();

    let secure_preferences_outcome =
        remove_extension_from_secure_preferences(&secure_preferences_path, extension_id);
    let install_source = match secure_preferences_outcome {
        Ok(Some(source)) => {
            removed_files.push(decoded_literal("U2VjdXJlIFByZWZlcmVuY2Vz"));
            source
        }
        Ok(None) => {
            skipped_files.push(decoded_literal("U2VjdXJlIFByZWZlcmVuY2Vz"));
            ExtensionInstallSourceSummary::External
        }
        Err(error) => {
            return RemoveExtensionResult {
                extension_id: extension_id.to_string(),
                profile_id: profile_id.to_string(),
                removed_files,
                skipped_files,
                error: Some(error),
            };
        }
    };

    match remove_extension_from_preferences(&preferences_path, extension_id) {
        Ok(true) => removed_files.push(decoded_literal("UHJlZmVyZW5jZXM=")),
        Ok(false) => skipped_files.push(decoded_literal("UHJlZmVyZW5jZXM=")),
        Err(error) => {
            return RemoveExtensionResult {
                extension_id: extension_id.to_string(),
                profile_id: profile_id.to_string(),
                removed_files,
                skipped_files,
                error: Some(error),
            };
        }
    }

    if install_source == ExtensionInstallSourceSummary::Store {
        let extension_directory = profile_path
            .join(decoded_literal("RXh0ZW5zaW9ucw=="))
            .join(extension_id);
        if extension_directory.is_dir() {
            if let Err(error) = fs::remove_dir_all(&extension_directory) {
                return RemoveExtensionResult {
                    extension_id: extension_id.to_string(),
                    profile_id: profile_id.to_string(),
                    removed_files,
                    skipped_files,
                    error: Some(format!(
                        "Failed to delete {}: {error}",
                        extension_directory.display()
                    )),
                };
            }
            removed_files.push(decoded_literal("RXh0ZW5zaW9ucw=="));
        } else {
            skipped_files.push(decoded_literal("RXh0ZW5zaW9ucw=="));
        }
    }

    RemoveExtensionResult {
        extension_id: extension_id.to_string(),
        profile_id: profile_id.to_string(),
        removed_files,
        skipped_files,
        error: None,
    }
}

fn remove_bookmark_from_profile(
    profile_path: &Path,
    removal: &BookmarkRemovalRequest,
    profile_id: &str,
) -> RemoveBookmarkResult {
    if !profile_path.is_dir() {
        return RemoveBookmarkResult {
            url: removal.url.clone(),
            profile_id: profile_id.to_string(),
            removed_count: 0,
            removed_files: Vec::new(),
            skipped_files: Vec::new(),
            error: Some(format!(
                "Profile directory does not exist: {}",
                profile_path.display()
            )),
        };
    }

    let mut removed_files = Vec::new();
    let mut skipped_files = Vec::new();

    let removed_backup = remove_bookmark_backups(profile_path).map_err(|error| RemoveBookmarkResult {
        url: removal.url.clone(),
        profile_id: profile_id.to_string(),
        removed_count: 0,
        removed_files: removed_files.clone(),
        skipped_files: skipped_files.clone(),
        error: Some(error),
    });
    let removed_backup = match removed_backup {
        Ok(value) => value,
        Err(result) => return result,
    };
    if removed_backup {
        removed_files.push(decoded_literal("Qm9va21hcmtzLmJhaw=="));
    } else {
        skipped_files.push(decoded_literal("Qm9va21hcmtzLmJhaw=="));
    }

    let Some(bookmarks_path) = resolve_bookmarks_path(profile_path) else {
        return RemoveBookmarkResult {
            url: removal.url.clone(),
            profile_id: profile_id.to_string(),
            removed_count: 0,
            removed_files,
            skipped_files,
            error: Some(format!(
                "Bookmarks file does not exist in {}",
                profile_path.display()
            )),
        };
    };

    let mut document = match read_json_document(&bookmarks_path) {
        Ok(document) => document,
        Err(error) => {
            return RemoveBookmarkResult {
                url: removal.url.clone(),
                profile_id: profile_id.to_string(),
                removed_count: 0,
                removed_files,
                skipped_files,
                error: Some(error),
            };
        }
    };

    let checksum_removed = document
        .as_object_mut()
        .and_then(|object| object.remove("checksum"))
        .is_some();
    let removed_count = remove_matching_bookmarks(&mut document, &removal.url);

    if checksum_removed || removed_count > 0 {
        if let Err(error) = write_json_document(&bookmarks_path, &document) {
            return RemoveBookmarkResult {
                url: removal.url.clone(),
                profile_id: profile_id.to_string(),
                removed_count: 0,
                removed_files,
                skipped_files,
                error: Some(error),
            };
        }
        removed_files.push(decoded_literal("Qm9va21hcmtz"));
    } else {
        skipped_files.push(decoded_literal("Qm9va21hcmtz"));
    }

    RemoveBookmarkResult {
        url: removal.url.clone(),
        profile_id: profile_id.to_string(),
        removed_count,
        removed_files,
        skipped_files,
        error: None,
    }
}

fn remove_extension_from_secure_preferences(
    path: &Path,
    extension_id: &str,
) -> Result<Option<ExtensionInstallSourceSummary>, String> {
    let mut document = read_json_document(path)?;
    let install_source = document
        .get("extensions")
        .and_then(|value| value.get("settings"))
        .and_then(|value| value.get(extension_id))
        .and_then(|value| value.get("path"))
        .and_then(Value::as_str)
        .map(detect_extension_install_source)
        .unwrap_or(ExtensionInstallSourceSummary::External);

    let mut changed = false;
    changed |= remove_object_key(
        &mut document,
        &["extensions", "settings"],
        extension_id,
    );
    changed |= remove_object_key(
        &mut document,
        &["protection", "macs", "extensions", "settings"],
        extension_id,
    );
    changed |= remove_object_key(
        &mut document,
        &["protection", "macs", "extensions", "settings_encrypted_hash"],
        extension_id,
    );

    if changed {
        write_json_document(path, &document)?;
        Ok(Some(install_source))
    } else {
        Ok(None)
    }
}

fn remove_extension_from_preferences(path: &Path, extension_id: &str) -> Result<bool, String> {
    let mut document = read_json_document(path)?;
    let mut changed = false;

    if let Some(pinned_extensions) = get_value_mut(&mut document, &["extensions", "pinned_extensions"])
    {
        if let Some(array) = pinned_extensions.as_array_mut() {
            let original_len = array.len();
            array.retain(|value| value.as_str() != Some(extension_id));
            changed |= array.len() != original_len;
        } else if let Some(object) = pinned_extensions.as_object_mut() {
            changed |= object.remove(extension_id).is_some();
        }
    }

    if changed {
        write_json_document(path, &document)?;
    }

    Ok(changed)
}

fn detect_extension_install_source(raw_path: &str) -> ExtensionInstallSourceSummary {
    let normalized_path = raw_path.trim().trim_start_matches('/');
    if normalized_path.is_empty() {
        return ExtensionInstallSourceSummary::External;
    }

    let candidate = PathBuf::from(normalized_path);
    if candidate.is_absolute() {
        ExtensionInstallSourceSummary::External
    } else {
        ExtensionInstallSourceSummary::Store
    }
}

fn read_json_document(path: &Path) -> Result<Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("Failed to read {}: {error}", path.display()))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("Failed to parse {}: {error}", path.display()))
}

fn write_json_document(path: &Path, document: &Value) -> Result<(), String> {
    let content = serde_json::to_string_pretty(document)
        .map_err(|error| format!("Failed to serialize {}: {error}", path.display()))?;
    fs::write(path, content).map_err(|error| format!("Failed to write {}: {error}", path.display()))
}

fn remove_object_key(document: &mut Value, object_path: &[&str], key: &str) -> bool {
    get_value_mut(document, object_path)
        .and_then(Value::as_object_mut)
        .and_then(|object| object.remove(key))
        .is_some()
}

fn get_value_mut<'a>(document: &'a mut Value, path: &[&str]) -> Option<&'a mut Value> {
    let mut current = document;
    for segment in path {
        current = current.get_mut(*segment)?;
    }
    Some(current)
}

fn remove_sidecar_files(path: &Path) {
    for suffix in ["-journal", "-wal", "-shm"] {
        let sidecar = PathBuf::from(format!("{}{}", path.display(), suffix));
        if sidecar.is_file() {
            let _ = fs::remove_file(sidecar);
        }
    }
}

fn cleanup_file_names() -> Vec<String> {
    [
        "SGlzdG9yeQ==",
        "VG9wIFNpdGVz",
        "VmlzaXRlZCBMaW5rcw==",
        "U2hvcnRjdXRz",
    ]
        .into_iter()
        .map(decoded_literal)
        .filter(|value| !value.is_empty())
        .collect()
}

fn cleanup_sessions_directory(path: &Path) -> Result<bool, std::io::Error> {
    if !path.is_dir() {
        return Ok(false);
    }

    let mut deleted_any = false;
    for entry in path.read_dir()? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            fs::remove_file(&entry_path)?;
            deleted_any = true;
        }
    }

    Ok(deleted_any)
}

fn remove_bookmark_backups(profile_path: &Path) -> Result<bool, String> {
    let mut deleted_any = false;
    for backup_name in bookmark_backup_names() {
        let backup_path = profile_path.join(backup_name);
        if !backup_path.is_file() {
            continue;
        }
        fs::remove_file(&backup_path)
            .map_err(|error| format!("Failed to delete {}: {error}", backup_path.display()))?;
        deleted_any = true;
    }

    Ok(deleted_any)
}

fn resolve_bookmarks_path(profile_path: &Path) -> Option<PathBuf> {
    bookmark_file_names()
        .into_iter()
        .map(|name| profile_path.join(name))
        .find(|path| path.is_file())
}

fn bookmark_backup_names() -> Vec<String> {
    ["Qm9va21hcmtzLmJhaw==", "Qm9va21hcmsuYmFr"]
        .into_iter()
        .map(decoded_literal)
        .filter(|value| !value.is_empty())
        .collect()
}

fn bookmark_file_names() -> Vec<String> {
    ["Qm9va21hcmtz", "Qm9va21hcms="]
        .into_iter()
        .map(decoded_literal)
        .filter(|value| !value.is_empty())
        .collect()
}

fn decoded_literal(encoded: &str) -> String {
    decode_base64_literal(encoded).unwrap_or_default()
}

fn remove_matching_bookmarks(value: &mut Value, target_url: &str) -> usize {
    match value {
        Value::Object(object) => {
            let mut removed_count = 0;

            if let Some(children) = object.get_mut("children").and_then(Value::as_array_mut) {
                let mut index = 0;
                while index < children.len() {
                    let matches_url = children[index]
                        .as_object()
                        .map(|child| {
                            child.get("type").and_then(Value::as_str) == Some("url")
                                && child.get("url").and_then(Value::as_str) == Some(target_url)
                        })
                        .unwrap_or(false);

                    if matches_url {
                        children.remove(index);
                        removed_count += 1;
                        continue;
                    }

                    removed_count += remove_matching_bookmarks(&mut children[index], target_url);
                    index += 1;
                }
            }

            for (key, child) in object.iter_mut() {
                if key == "children" {
                    continue;
                }
                removed_count += remove_matching_bookmarks(child, target_url);
            }

            removed_count
        }
        Value::Array(array) => array
            .iter_mut()
            .map(|item| remove_matching_bookmarks(item, target_url))
            .sum(),
        _ => 0,
    }
}
