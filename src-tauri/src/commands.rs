use crate::models::hard_worker::HardWorker;
use crate::models::task::{
    DeleteTaskRequest, ScheduledTask, ScheduledTaskCreateDTO, Task, TaskCreateDTO,
};
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::sync::RwLock;
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;
use uuid::Uuid;

static SCHEDULED_TASK_CACHE: OnceCell<RwLock<Vec<ScheduledTask>>> = OnceCell::new();

#[tauri::command]
pub fn notify(app: AppHandle, title: String, message: String) -> Result<(), String> {
    notify_message(&app, title, message)?;
    Ok(())
}

fn notify_message(app: &AppHandle, title: String, message: String) -> Result<(), String> {
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
pub fn reset(app: AppHandle) -> Result<(), String> {
    delete_json_file(&app, "tasks.json".to_string())?;
    delete_json_file(&app, "hardworker.json".to_string())?;
    delete_json_file(&app, "scheduled_tasks.json".to_string())?;

    Ok(())
}

pub fn delete_json_file(app: &AppHandle, filename: String) -> Result<(), String> {
    // 例: "zantas/tasks.json" のようにファイル名を受け取る
    let path = app
        .path()
        .resolve(&format!("zantas/{}", filename), BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    // ファイルが存在していれば削除
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("削除失敗: {}", e))?;
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
    let hw: HardWorker = load_hardworker(&app)?;
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
pub fn complete_task(app: AppHandle, dto: DeleteTaskRequest) -> Result<Vec<Task>, String> {
    // ---------- タスク側更新 ----------
    let tasks = delete_task_from_file(&app, &dto)?;

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

    Ok(tasks)
}

#[tauri::command]
pub fn save_task(app: AppHandle, task: TaskCreateDTO) -> Result<Task, String> {
    save_task_to_file(&app, &task)
}

fn save_task_to_file(app: &AppHandle, task: &TaskCreateDTO) -> Result<Task, String> {
    let path = app
        .path()
        .resolve("zantas/tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    // 既存の定期タスクを読み込む
    let mut list: Vec<Task> = load_tasks(&app)?;

    let new_task = Task::new(
        task.title.clone(),
        task.description.clone(),
        task.due_date.clone(),
    );

    // 新しいタスクを追加
    list.push(new_task.clone());

    // 保存
    let json = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(new_task.clone())
}

#[tauri::command]
pub fn delete_task(app: AppHandle, dto: DeleteTaskRequest) -> Result<Vec<Task>, String> {
    let tasks = delete_task_from_file(&app, &dto)?;

    Ok(tasks)
}

pub fn delete_task_from_file(
    app: &AppHandle,
    dto: &DeleteTaskRequest,
) -> Result<Vec<Task>, String> {
    let path = app
        .path()
        .resolve("zantas/tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        return Err("タスクファイルが存在しません。".to_string());
    }

    let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut tasks: Vec<Task> = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    let target_id = Uuid::parse_str(&dto.task_id).map_err(|e| e.to_string())?;
    let before_count = tasks.len();

    tasks.retain(|task| task.id != target_id);

    if tasks.len() == before_count {
        return Err("該当するタスクが見つかりませんでした。".to_string());
    }

    let new_json = serde_json::to_string_pretty(&tasks).map_err(|e| e.to_string())?;
    std::fs::write(&path, new_json).map_err(|e| e.to_string())?;

    Ok(tasks)
}

#[tauri::command]
pub fn delete_scheduled_task(
    app: AppHandle,
    dto: DeleteTaskRequest,
) -> Result<Vec<ScheduledTask>, String> {
    let tasks = delete_scheduled_task_from_file(&app, &dto)?;
    update_scheduled_task_cache(tasks.clone());

    Ok(tasks)
}

pub fn delete_scheduled_task_from_file(
    app: &AppHandle,
    dto: &DeleteTaskRequest,
) -> Result<Vec<ScheduledTask>, String> {
    let path = app
        .path()
        .resolve("zantas/scheduled_tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        return Err("タスクファイルが存在しません。".to_string());
    }

    let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut tasks: Vec<ScheduledTask> = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    let target_id = Uuid::parse_str(&dto.task_id).map_err(|e| e.to_string())?;
    let before_count = tasks.len();

    tasks.retain(|task| task.id != target_id);

    if tasks.len() == before_count {
        return Err("該当するタスクが見つかりませんでした。".to_string());
    }

    let new_json = serde_json::to_string_pretty(&tasks).map_err(|e| e.to_string())?;
    std::fs::write(&path, new_json).map_err(|e| e.to_string())?;

    Ok(tasks)
}

// -----------------------------
// タスク読み込み
// -----------------------------

#[tauri::command]
pub fn get_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let tasks = load_tasks(&app)?;
    Ok(tasks)
}

#[tauri::command]
pub fn get_scheduled_tasks(app: AppHandle) -> Result<Vec<ScheduledTask>, String> {
    let tasks = load_scheduled_tasks(&app)?;
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

fn load_scheduled_tasks(app: &AppHandle) -> Result<Vec<ScheduledTask>, String> {
    let path = app
        .path()
        .resolve("zantas/scheduled_tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        // 初回は空でいい
        return Ok(vec![]);
    }

    let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let tasks: Vec<ScheduledTask> = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    Ok(tasks)
}

#[tauri::command]
pub fn save_scheduled_task(
    app: AppHandle,
    dto: ScheduledTaskCreateDTO,
) -> Result<ScheduledTask, String> {
    let path = app
        .path()
        .resolve("zantas/scheduled_tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

    let new_task = Task::new(dto.task.title, dto.task.description, None);

    let new_schedule_task = ScheduledTask::new(new_task, dto.pattern);

    // // 既存の定期タスクを読み込む
    let mut list: Vec<ScheduledTask> = if path.exists() {
        let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())?
    } else {
        vec![]
    };

    // 新しいタスクを追加
    list.push(new_schedule_task.clone());

    // 保存
    let json = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    update_scheduled_task_cache(list);
    Ok(new_schedule_task)
}

// 定期チェック
// === 初期化または取得 ===
fn get_scheduled_tasks_cached(app: &AppHandle) -> Result<Vec<ScheduledTask>, String> {
    let cache = SCHEDULED_TASK_CACHE.get_or_init(|| {
        let tasks = load_scheduled_tasks(app).unwrap_or_default();
        RwLock::new(tasks)
    });
    let guard = cache.read().unwrap();
    Ok(guard.clone())
}

// === キャッシュ更新（保存後など） ===
fn update_scheduled_task_cache(tasks: Vec<ScheduledTask>) {
    if let Some(lock) = SCHEDULED_TASK_CACHE.get() {
        let mut guard = lock.write().unwrap();
        *guard = tasks;
    }
}
#[tauri::command]
pub fn check_scheduled_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let now = chrono::Local::now().naive_local();
    // let mut scheduled_tasks = load_scheduled_tasks(&app)?;
    let mut scheduled_tasks = get_scheduled_tasks_cached(&app)?;

    let mut tasks_to_add = vec![];

    for scheduled in &mut scheduled_tasks {
        if scheduled.should_trigger(now) {
            let new_task = Task::new(
                scheduled.task.title.clone(),
                scheduled.task.description.clone(),
                Some(now.date()), // または due_date ロジック
            );
            tasks_to_add.push(new_task);
            scheduled.last_triggered = Some(now.date());
            notify_message(
                &app,
                "定期依頼登録のお知らせ".to_string(),
                "定期依頼を登録しました。一覧を再読み込みして確認ください。".to_string(),
            )?;
        }
    }

    if !tasks_to_add.is_empty() {
        save_new_tasks(&app, tasks_to_add.clone())?;
        update_scheduled_task_cache(scheduled_tasks.clone());
        save_scheduled_tasks(&app, scheduled_tasks)?; // last_triggered の更新
    }

    Ok(tasks_to_add.clone())
}

fn save_new_tasks(app: &AppHandle, new_tasks: Vec<Task>) -> Result<Vec<Task>, String> {
    let path = app
        .path()
        .resolve("zantas/tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    let mut all_tasks: Vec<Task> = if path.exists() {
        let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())?
    } else {
        vec![]
    };

    all_tasks.extend(new_tasks);

    let new_json = serde_json::to_string_pretty(&all_tasks).map_err(|e| e.to_string())?;
    std::fs::write(&path, new_json).map_err(|e| e.to_string())?;

    Ok(all_tasks)
}

fn save_scheduled_tasks(
    app: &AppHandle,
    new_scheduled_tasks: Vec<ScheduledTask>,
) -> Result<(), String> {
    let path = app
        .path()
        .resolve("zantas/scheduled_tasks.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

    let json = serde_json::to_string_pretty(&new_scheduled_tasks).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn close_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}
