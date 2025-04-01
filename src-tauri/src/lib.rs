mod commands;
use commands::load_hardworker;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_hardworker,
            commands::save_hardworker,
            commands::save_tasks,
            commands::load_tasks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
