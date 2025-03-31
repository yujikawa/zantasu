use serde::{Deserialize, Serialize};

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
