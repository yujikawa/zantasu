use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::path::BaseDirectory::AppData;
use tauri::{fs, path::BaseDirectory, AppHandle, Manager};

#[derive(Debug, Deserialize, Serialize)]
pub struct HardWorker {
    pub name: String,
    pub rank: String,
}

impl HardWorker {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rank: "F".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Task {
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub rank: String,                // 必須
    pub due_date: Option<String>,    // 任意
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        rank: String,
        due_date: Option<String>,
    ) -> Self {
        Self {
            title,
            description,
            rank,
            due_date,
        }
    }
}

#[tauri::command]
pub fn save_hardworker(app: AppHandle, name: String) -> Result<(), String> {
    let path = app
        .path()
        .resolve("zantas/hardworker.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

    let hw = HardWorker::new(name);
    let json = serde_json::to_string_pretty(&hw).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn load_hardworker(app: AppHandle) -> Result<HardWorker, String> {
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
pub fn save_tasks(app: AppHandle, tasks: Vec<Task>) -> Result<(), String> {
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
