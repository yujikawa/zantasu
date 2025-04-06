use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Rank {
    F,
    E,
    D,
    C,
    B,
    A,
    S,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Job {
    // 前衛
    Swordsman,
    AxeWarrior,
    Spearman,
    Knight,
    Monk,
    // 後衛
    Mage,
    Priest,
    Summoner,
    Sage,
    Necromancer,
    // 特殊
    Hunter,
    Thief,
    Ninja,
    Artificer,
    Illusionist,
    // 生活
    Blacksmith,
    Alchemist,
    Cook,
    Miner,
    Farmer,
    // レア
    Hero,
    DemonLord,
    Merchant,
    Jobless,
    BeastTamer,
}

// HardWorker
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HardWorker {
    pub name: String,
    pub rank: Rank,
    pub job: Option<Job>,
    pub achievement: u32,
    pub last_complete: Option<String>,
}
