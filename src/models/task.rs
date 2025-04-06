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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "pattern_type")]
pub enum SchedulePattern {
    OneTime { datetime: String },
    Monthly { day: u32, time: String },
    Weekly { weekday: u32, time: String },
    Daily { time: String },
}

impl SchedulePattern {
    pub fn to_pattern_string(&self) -> String {
        match self {
            SchedulePattern::OneTime { datetime } => format!("1回限り: {}", datetime),
            SchedulePattern::Monthly { day, time } => format!("毎月{}日の{}", day, time),
            SchedulePattern::Weekly { weekday, time } => {
                let day_str = match weekday {
                    0 => "日曜",
                    1 => "月曜",
                    2 => "火曜",
                    3 => "水曜",
                    4 => "木曜",
                    5 => "金曜",
                    6 => "土曜",
                    _ => "不明",
                };
                format!("毎週{}の{}", day_str, time)
            }
            SchedulePattern::Daily { time } => format!("毎日{}", time),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScheduledTaskCreateDTO {
    pub task: TaskCreateDTO,
    pub pattern: SchedulePattern,
}

impl ScheduledTaskCreateDTO {
    pub fn new(task: TaskCreateDTO, pattern: SchedulePattern) -> Self {
        Self { task, pattern }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScheduledTask {
    pub id: String,
    pub task: TaskCreateDTO,
    pub pattern: SchedulePattern,
}
