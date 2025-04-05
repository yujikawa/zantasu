use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TaskCreateDTO {
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub rank: String,                // 必須
    pub due_date: Option<String>,    // 任意
}

impl TaskCreateDTO {
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Task {
    pub id: String,
    pub title: String,               // 必須
    pub description: Option<String>, // 任意
    pub rank: String,                // 必須
    pub due_date: Option<String>,    // 任意
}


#[derive(Serialize)]
pub struct DeleteTaskRequest {
    pub task_id: String,
}