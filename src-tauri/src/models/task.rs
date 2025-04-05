use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TaskCreateDTO {
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub rank: String,                // 必須
    pub due_date: Option<NaiveDate>, // 任意
}


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Task {
    pub id: Uuid,
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub rank: String,                // 必須
    pub due_date: Option<NaiveDate>, // 任意
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        rank: String,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            rank,
            due_date,
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteTaskRequest {
    pub task_id: String,
}