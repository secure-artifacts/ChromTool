use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Manager};

use crate::{
    browsers::{browser_definitions, default_browser_executable, resolve_browser_executable},
    models::{
        BrowserConfigEntry, BrowserConfigListResponse, BrowserConfigSource,
        CreateCustomBrowserConfigInput, CustomBrowserConfigRecord, StoredBrowserConfigs,
    },
    utils::platform_user_data_root_dir,
};

const CONFIG_FILE_NAME: &str = "browser-configs.json";

pub fn load_browser_config_list(app: &AppHandle) -> Result<BrowserConfigListResponse, String> {
    Ok(BrowserConfigListResponse {
        configs: resolve_browser_configs(app)?,
    })
}

pub fn resolve_browser_configs(app: &AppHandle) -> Result<Vec<BrowserConfigEntry>, String> {
    let mut configs = default_browser_configs()?;
    let stored = load_stored_configs(app)?;

    configs.extend(
        stored
            .custom_configs
            .into_iter()
            .map(|config| BrowserConfigEntry {
                id: config.id,
                source: BrowserConfigSource::Custom,
                browser_family_id: config.browser_family_id.or(config.icon_key.clone()),
                icon_key: config.icon_key,
                name: config.name,
                executable_found: PathBuf::from(&config.executable_path).is_file(),
                executable_path: config.executable_path,
                user_data_path: config.user_data_path,
                deletable: true,
            }),
    );

    Ok(configs)
}

pub fn create_custom_browser_config(
    app: &AppHandle,
    input: CreateCustomBrowserConfigInput,
) -> Result<BrowserConfigListResponse, String> {
    let name = input.name.trim();
    let icon_key = input.icon_key.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    });
    let executable_path = input.executable_path.trim();
    let user_data_path = input.user_data_path.trim();
    let browser_family_id = infer_browser_family_id(icon_key.as_deref());

    if name.is_empty() {
        return Err("Name is required.".to_string());
    }
    if executable_path.is_empty() {
        return Err("Executable path is required.".to_string());
    }
    if user_data_path.is_empty() {
        return Err("User data path is required.".to_string());
    }

    let mut stored = load_stored_configs(app)?;
    stored.custom_configs.push(CustomBrowserConfigRecord {
        id: generate_custom_config_id(),
        name: name.to_string(),
        icon_key,
        browser_family_id,
        executable_path: executable_path.to_string(),
        user_data_path: user_data_path.to_string(),
    });

    save_stored_configs(app, &stored)?;
    load_browser_config_list(app)
}

pub fn delete_custom_browser_config(
    app: &AppHandle,
    config_id: &str,
) -> Result<BrowserConfigListResponse, String> {
    let mut stored = load_stored_configs(app)?;
    let original_len = stored.custom_configs.len();
    stored
        .custom_configs
        .retain(|config| config.id != config_id);

    if stored.custom_configs.len() == original_len {
        return Err(format!("Custom browser config not found: {config_id}"));
    }

    save_stored_configs(app, &stored)?;
    load_browser_config_list(app)
}

pub fn find_browser_config(app: &AppHandle, config_id: &str) -> Result<BrowserConfigEntry, String> {
    resolve_browser_configs(app)?
        .into_iter()
        .find(|config| config.id == config_id)
        .ok_or_else(|| format!("Browser config not found: {config_id}"))
}

fn default_browser_configs() -> Result<Vec<BrowserConfigEntry>, String> {
    let user_data_root = platform_user_data_root_dir().ok_or_else(|| {
        "Unable to resolve the default browser data directory for the current user.".to_string()
    })?;

    Ok(browser_definitions()
        .into_iter()
        .map(|definition| {
            let user_data_path = definition
                .local_app_data_segments
                .iter()
                .fold(user_data_root.clone(), |path, segment| path.join(segment));

            let executable = resolve_browser_executable(definition.id);
            let executable_found = executable.is_some();

            BrowserConfigEntry {
                id: definition.id.to_string(),
                source: BrowserConfigSource::Default,
                browser_family_id: Some(definition.id.to_string()),
                icon_key: Some(definition.id.to_string()),
                name: definition.name.to_string(),
                executable_found,
                executable_path: executable
                    .or_else(|| default_browser_executable(definition.id))
                    .map(|path| path.display().to_string())
                    .unwrap_or_default(),
                user_data_path: user_data_path.display().to_string(),
                deletable: false,
            }
        })
        .collect())
}

fn load_stored_configs(app: &AppHandle) -> Result<StoredBrowserConfigs, String> {
    let path = config_file_path(app)?;
    if !path.is_file() {
        return Ok(StoredBrowserConfigs {
            custom_configs: Vec::new(),
        });
    }

    let content = fs::read_to_string(&path).map_err(|error| {
        format!(
            "Failed to read browser config file {}: {error}",
            path.display()
        )
    })?;
    serde_json::from_str(&content).map_err(|error| {
        format!(
            "Failed to parse browser config file {}: {error}",
            path.display()
        )
    })
}

fn save_stored_configs(app: &AppHandle, stored: &StoredBrowserConfigs) -> Result<(), String> {
    let path = config_file_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "Failed to create browser config directory {}: {error}",
                parent.display()
            )
        })?;
    }

    let content = serde_json::to_string_pretty(stored)
        .map_err(|error| format!("Failed to serialize browser configs: {error}"))?;
    fs::write(&path, content).map_err(|error| {
        format!(
            "Failed to write browser config file {}: {error}",
            path.display()
        )
    })
}

fn config_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;
    Ok(app_data_dir.join(CONFIG_FILE_NAME))
}

fn generate_custom_config_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    format!("custom-{timestamp}")
}

fn infer_browser_family_id(icon_key: Option<&str>) -> Option<String> {
    match icon_key {
        Some("chrome") => Some("chrome".to_string()),
        Some("edge") => Some("edge".to_string()),
        Some("brave") => Some("brave".to_string()),
        Some("vivaldi") => Some("vivaldi".to_string()),
        Some("yandex") => Some("yandex".to_string()),
        Some("chromium") => Some("chromium".to_string()),
        _ => None,
    }
}
