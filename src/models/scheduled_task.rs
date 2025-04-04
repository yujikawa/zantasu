use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "pattern_type")]
pub enum PatternDTO {
    OneTime { datetime: String },
    Monthly { day: u32, time: String },
    Weekly { weekday: u32, time: String },
    Daily { time: String },
}

#[derive(Serialize)]
pub struct ScheduledTaskDTO {
    pub title: String,
    pub description: String,
    pub pattern: PatternDTO,
}
