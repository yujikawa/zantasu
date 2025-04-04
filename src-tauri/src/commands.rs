use crate::models::hard_worker::HardWorker;
use crate::models::scheduled_task::ScheduledTask;
use crate::models::task::Task;
use std::path::PathBuf;
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
pub fn notify(app: AppHandle, title: String, message: String) -> Result<(), String> {
    notify_message(app, title, message)?;
    Ok(())
}

fn notify_message(app: AppHandle, title: String, message: String) -> Result<(), String> {
    let _ = app
        .notification()
        .builder()
        .title(&title)
        .body(&message)
        .show()
        .map_err(|e| e.to_string())?;
    Ok(())
}
#[tauri::command]
pub fn check_due_date_notify(app: AppHandle) -> Result<(), String> {
    let tasks = load_tasks(&app)?;
    for task in &tasks {
        if let Some(due_date) = task.due_date {
            let now = chrono::Local::now().date_naive();
            let term = due_date - now;
            let days = term.num_days().max(0);
            if days == 3 {}
        }
    }
    Ok(())
}

#[tauri::command]
pub fn save_hardworker(app: AppHandle, name: String) -> Result<HardWorker, String> {
    let path = app
        .path()
        .resolve("zantas/hardworker.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

    let hw = HardWorker::new(name);
    save_hardworker_json(&path, &hw)?;

    Ok(hw)
}

fn save_hardworker_json(path: &PathBuf, hw: &HardWorker) -> Result<(), String> {
    let json = serde_json::to_string_pretty(hw).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_hardworker(app: AppHandle) -> Result<HardWorker, String> {
    let mut hw: HardWorker = load_hardworker(&app)?;
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
    let mut hw: HardWorker = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    hw.update_rank();
    save_hardworker_json(&path, &hw)?;
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
    hw.last_complete = Some(chrono::Local::now().date_naive());

    let json_hw = serde_json::to_string_pretty(&hw).map_err(|e| e.to_string())?;
    std::fs::write(&hw_path, json_hw).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn save_task(app: AppHandle, task: Task) -> Result<(), String> {
    save_task_to_file(&app, &task)
}

fn save_task_to_file(app: &AppHandle, task: &Task) -> Result<(), String> {
    let path = app
        .path()
        .resolve("zantas/tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    // 既存の定期タスクを読み込む
    let mut list: Vec<Task> = load_tasks(&app)?;

    // 新しいタスクを追加
    list.push(task.clone());

    // 保存
    let json = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

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
pub fn get_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let tasks = load_tasks(&app)?;
    Ok(tasks)
}

fn load_tasks(app: &AppHandle) -> Result<Vec<Task>, String> {
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
pub fn save_scheduled_task(app: AppHandle, dto: ScheduledTask) -> Result<(), String> {
    let path = app
        .path()
        .resolve("zantas/scheduled_tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

    // // 既存の定期タスクを読み込む
    let mut list: Vec<ScheduledTask> = if path.exists() {
        let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())?
    } else {
        vec![]
    };

    // 新しいタスクを追加
    list.push(dto);

    // 保存
    let json = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn close_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}
