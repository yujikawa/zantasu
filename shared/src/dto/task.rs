use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "pattern_type")]
pub enum Pattern {
    OneTime { datetime: String },
    Monthly { day: u32, time: String },
    Weekly { weekday: u32, time: String },
    Daily { time: String },
}

// タスク作成用dto
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub rank: String,
    pub due_date: Option<String>,
    pub pattern: Option<Pattern>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskIdRequest {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub rank: String,
    pub due_date: Option<String>,
    pub pattern: Option<Pattern>,
}
