use rand::seq::IndexedMutRandom;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    F,
    E,
    D,
    C,
    B,
    A,
    S,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

impl Job {
    fn random_job() -> Job {
        let mut r = rand::rng();
        // 職業リスト
        let mut jobs = vec![
            Job::Swordsman,
            Job::AxeWarrior,
            Job::Spearman,
            Job::Knight,
            Job::Monk,
            Job::Mage,
            Job::Priest,
            Job::Summoner,
            Job::Sage,
            Job::Necromancer,
            Job::Hunter,
            Job::Thief,
            Job::Ninja,
            Job::Artificer,
            Job::Illusionist,
            Job::Blacksmith,
            Job::Alchemist,
            Job::Cook,
            Job::Miner,
            Job::Farmer,
            Job::Hero,
            Job::DemonLord,
            Job::Merchant,
            Job::Jobless,
            Job::BeastTamer,
        ];

        // ランダム選択
        jobs.choose_mut(&mut r).unwrap().clone()
    }
}

// HardWorker
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HardWorker {
    pub name: String,
    pub rank: Rank,
    pub job: Job,
    pub achievement: u32,
    pub last_complete: Option<String>,
}

impl HardWorker {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            rank: Rank::F,
            job: Job::random_job(),
            achievement: 0,
            last_complete: None,
        }
    }
}
