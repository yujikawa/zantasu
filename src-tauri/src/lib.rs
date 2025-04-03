use tauri::Manager;

mod commands;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os="windows")]
            window.set_decorations(false)?;
            
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_hardworker,
            commands::save_hardworker,
            commands::save_tasks,
            commands::load_tasks,
            commands::close_app,
            commands::complete_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
