use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tauri::utils::pattern;
use uuid::Uuid;
#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "pattern_type")]
pub enum SchedulePattern {
    OneTime { datetime: String },       // 1回限り（例: "2025-04-20T10:00"）
    Monthly { day: u32, time: String }, // 毎月20日 "10:00"
    Weekly { weekday: u32, time: String }, // 毎週月曜（0=Sun〜6=Sat）
    Daily { time: String },             // 毎日 "09:00"
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskCreateDTO {
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub due_date: Option<NaiveDate>, // 任意
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ScheduledTaskCreateDTO {
    pub task: TaskCreateDTO,
    pub pattern: SchedulePattern, // 必須
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub due_date: Option<NaiveDate>, // 任意
}

impl Task {
    pub fn new(title: String, description: Option<String>, due_date: Option<NaiveDate>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            due_date,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ScheduledTask {
    pub id: Uuid,
    pub task: TaskCreateDTO,
    pub pattern: SchedulePattern,
    pub last_triggered: Option<NaiveDate>,
}

impl ScheduledTask {
    pub fn new(task: TaskCreateDTO, pattern: SchedulePattern) -> Self {
        Self {
            id: Uuid::new_v4(),
            task,
            pattern,
            last_triggered: None,
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteTaskRequest {
    pub task_id: String,
}
