use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TaskCreateDTO {
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub due_date: Option<String>,    // 任意
}

impl TaskCreateDTO {
    pub fn new(title: String, description: Option<String>, due_date: Option<String>) -> Self {
        Self {
            title,
            description,
            due_date,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Serialize)]
pub struct DeleteTaskRequest {
    pub task_id: String,
}

#[derive(Serialize)]
#[serde(tag = "pattern_type")]
pub enum SchedulePattern {
    OneTime { datetime: String },
    Monthly { day: u32, time: String },
    Weekly { weekday: u32, time: String },
    Daily { time: String },
}

#[derive(Serialize)]
pub struct ScheduledTaskDTO {
    pub task: TaskCreateDTO,
    pub pattern: SchedulePattern,
}

impl ScheduledTaskDTO {
    pub fn new(task: TaskCreateDTO, pattern: SchedulePattern) -> Self {
        Self { task, pattern }
    }
}
