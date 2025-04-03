use leptos::attr::Datetime;
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

    pub fn to_str(&self) -> &'static str {
        match self {
            Rank::F => "F 等級",
            Rank::E => "E 等級",
            Rank::D => "D 等級",
            Rank::C => "C 等級",
            Rank::B => "B 等級",
            Rank::A => "A 等級",
            Rank::S => "S 等級",
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
    pub fn to_str(&self) -> &'static str {
        match self {
            // Frontline
            Job::Swordsman => "剣士",
            Job::AxeWarrior => "斧戦士",
            Job::Spearman => "槍使い",
            Job::Knight => "騎士",
            Job::Monk => "武闘家",
            // Rearguard
            Job::Mage => "魔法使い",
            Job::Priest => "僧侶",
            Job::Summoner => "召喚士",
            Job::Sage => "賢者",
            Job::Necromancer => "呪術師",
            // Special
            Job::Hunter => "狩人",
            Job::Thief => "盗賊",
            Job::Ninja => "忍者",
            Job::Artificer => "魔道具師",
            Job::Illusionist => "幻術師",
            // Life
            Job::Blacksmith => "鍛冶師",
            Job::Alchemist => "錬金術師",
            Job::Cook => "料理人",
            Job::Miner => "採掘師",
            Job::Farmer => "農民",
            // Rare
            Job::Hero => "勇者",
            Job::DemonLord => "魔王",
            Job::Merchant => "商人",
            Job::Jobless => "無職",
            Job::BeastTamer => "魔物使い",
        }
    }
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

impl HardWorker {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            rank: Rank::F,
            job: Some(Job::Jobless),
            achievement: 0,
            last_complete: Some("実績なし".to_string()),
        }
    }
}
