use crate::models::hard_worker::HardWorker;
use crate::models::task::Task;
use serde::{Deserialize, Serialize};
use tauri::{fs, path::BaseDirectory, AppHandle, Manager};

#[tauri::command]
pub fn save_hardworker(app: AppHandle, name: String) -> Result<HardWorker, String> {
    let path = app
        .path()
        .resolve("zantas/hardworker.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

    let hw = HardWorker::new(name);
    let json = serde_json::to_string_pretty(&hw).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(hw)
}

#[tauri::command]
pub fn get_hardworker(app: AppHandle) -> Result<HardWorker, String> {
    let path = app
        .path()
        .resolve("zantas/hardworker.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        // ファイルがない場合はデフォルトを返す
        return Ok(HardWorker::new("".to_string()));
    }
    let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let hw: HardWorker = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    Ok(hw)
}

fn load_hardworker(app: &AppHandle) -> Result<HardWorker, String> {
    let path = app
        .path()
        .resolve("zantas/hardworker.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        // ファイルがない場合はデフォルトを返す
        return Ok(HardWorker::new("".to_string()));
    }
    let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let hw: HardWorker = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    Ok(hw)
}

#[tauri::command]
pub fn complete_task(app: AppHandle, tasks: Vec<Task>) -> Result<(), String> {
    // ---------- タスク側更新 ----------
    save_tasks_to_file(&app, &tasks)?;

    // // ---------- ハードワーカー更新 ----------
    let hw_path = app
        .path()
        .resolve("zantas/hardworker.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    // let json_hw = std::fs::read_to_string(&hw_path).map_err(|e| e.to_string())?;
    let mut hw: HardWorker = load_hardworker(&app)?;

    hw.achievement += 1;
    hw.last_complete = Some(chrono::Local::now().format("%Y-%m-%d").to_string());

    let json_hw = serde_json::to_string_pretty(&hw).map_err(|e| e.to_string())?;
    std::fs::write(&hw_path, json_hw).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn save_tasks(app: AppHandle, tasks: Vec<Task>) -> Result<(), String> {
    save_tasks_to_file(&app, &tasks)
}

fn save_tasks_to_file(app: &AppHandle, tasks: &Vec<Task>) -> Result<(), String> {
    let path = app
        .path()
        .resolve("zantas/tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&tasks).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

// -----------------------------
// タスク読み込み
// -----------------------------

#[tauri::command]
pub fn load_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let path = app
        .path()
        .resolve("zantas/tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        // 初回は空でいい
        return Ok(vec![]);
    }

    let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let tasks: Vec<Task> = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    Ok(tasks)
}

#[tauri::command]
pub fn close_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}
