use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HardWorker {
    pub name: String,
    pub rank: String,
}

impl HardWorker {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            rank: "F".to_string(),
        }
    }
}
