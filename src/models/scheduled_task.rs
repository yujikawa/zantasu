use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScheduledTask {
    pub title: String,
    pub scheduled_time: String,
}
