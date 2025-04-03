use chrono::NaiveDate;
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

impl Rank {
    pub fn as_index(&self) -> usize {
        match self {
            Rank::F => 0,
            Rank::E => 1,
            Rank::D => 2,
            Rank::C => 3,
            Rank::B => 4,
            Rank::A => 5,
            Rank::S => 6,
        }
    }

    pub fn from_index(index: usize) -> Rank {
        match index {
            0 => Rank::F,
            1 => Rank::E,
            2 => Rank::D,
            3 => Rank::C,
            4 => Rank::B,
            5 => Rank::A,
            _ => Rank::S,
        }
    }
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
    pub last_complete: Option<NaiveDate>,
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

    pub fn update_rank(&mut self) {
        if let Some(last_complete) = self.last_complete {
            let now = chrono::Local::now().date_naive();
            let term = now - last_complete;
            let days = term.num_days().max(0);
            let rank_down_num = (days / 10) as usize;
            if rank_down_num > 0 {
                let current_rank = self.rank.as_index();
                let update_rank = current_rank.saturating_sub(rank_down_num);
                let new_rank = Rank::from_index(update_rank);
                self.rank = new_rank;
            } else {
                let new_rank = match self.achievement {
                    0..10 => Rank::F,
                    10..20 => Rank::E,
                    20..30 => Rank::D,
                    30..40 => Rank::C,
                    40..50 => Rank::B,
                    50..60 => Rank::A,
                    _ => Rank::S,
                };
                self.rank = new_rank;
            }
        }
    }
}
