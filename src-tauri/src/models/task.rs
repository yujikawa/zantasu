use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskCreateDTO {
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub due_date: Option<NaiveDate>, // 任意
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ScheduledTaskCreateDTO {
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub pattern: Option<Pattern>, // 
}


#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "pattern_type")]
pub enum Pattern {
    Monthly { day: u32, time: String }, // 毎月20日 "10:00"
    Weekly { weekday: u32, time: String }, // 毎週月曜（0=Sun〜6=Sat）
    Daily { time: String },             // 毎日 "09:00"
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub due_date: Option<NaiveDate>, // 任意
    pub pattern: Option<Pattern>,
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        due_date: Option<NaiveDate>,
        pattern: Option<Pattern>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            due_date,
            pattern,
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteTaskRequest {
    pub task_id: String,
}
