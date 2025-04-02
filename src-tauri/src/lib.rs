mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_hardworker,
            commands::save_hardworker,
            commands::save_tasks,
            commands::load_tasks,
            commands::close_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
