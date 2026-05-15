mod browsers;
mod commands;
mod config_store;
mod models;
mod scanner;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::scan_browsers,
            commands::scan_browser,
            commands::scan_password_sites,
            commands::list_browser_configs,
            commands::create_custom_browser_config,
            commands::delete_custom_browser_config,
            commands::open_browser_profile,
            commands::cleanup_history_files,
            commands::remove_extensions,
            commands::remove_bookmarks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
