use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "pattern_type")]
pub enum SchedulePattern {
    OneTime { datetime: String },       // 1回限り（例: "2025-04-20T10:00"）
    Monthly { day: u32, time: String }, // 毎月20日 "10:00"
    Weekly { weekday: u32, time: String }, // 毎週月曜（0=Sun〜6=Sat）
    Daily { time: String },             // 毎日 "09:00"
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ScheduledTask {
    pub title: String,
    pub description: String,
    pub pattern: SchedulePattern,
}
