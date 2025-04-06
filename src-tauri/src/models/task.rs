use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};
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
    pub task: Task,
    pub pattern: SchedulePattern,
    pub last_triggered: Option<NaiveDate>,
}

impl ScheduledTask {
    pub fn new(task: Task, pattern: SchedulePattern) -> Self {
        Self {
            id: Uuid::new_v4(),
            task,
            pattern,
            last_triggered: None,
        }
    }

    pub fn should_trigger(&self, now: NaiveDateTime) -> bool {
        match &self.pattern {
            SchedulePattern::OneTime { datetime } => {
                if let Ok(scheduled_time) =
                    NaiveDateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M")
                {
                    if self.last_triggered.is_some() {
                        return false; // 一度だけ実行済み
                    }
                    return now >= scheduled_time;
                }
                false
            }

            SchedulePattern::Monthly { day, time } => {
                if now.day() != *day {
                    return false;
                }

                if let Some(last) = self.last_triggered {
                    if last.month() == now.month() && last.year() == now.year() {
                        return false; // 今月実行済み
                    }
                }

                if let Ok(target_time) = NaiveTime::parse_from_str(time, "%H:%M") {
                    now.time() >= target_time
                } else {
                    false
                }
            }

            SchedulePattern::Weekly { weekday, time } => {
                if now.weekday().num_days_from_sunday() != *weekday {
                    return false;
                }

                if let Some(last) = self.last_triggered {
                    let same_week = now.iso_week() == last.iso_week() && now.year() == last.year();
                    if same_week {
                        return false; // 今週実行済み
                    }
                }

                if let Ok(target_time) = NaiveTime::parse_from_str(time, "%H:%M") {
                    now.time() >= target_time
                } else {
                    false
                }
            }

            SchedulePattern::Daily { time } => {
                if let Some(last) = self.last_triggered {
                    if last == now.date() {
                        return false; // 今日実行済み
                    }
                }

                if let Ok(target_time) = NaiveTime::parse_from_str(time, "%H:%M") {
                    now.time() >= target_time
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteTaskRequest {
    pub task_id: String,
}
