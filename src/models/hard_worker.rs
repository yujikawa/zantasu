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

pub struct HardWorkerStats {
    pub diligence: u8,  // 勤勉さ
    pub focus: u8,      // 集中力
    pub planning: u8,   // 段取り力
    pub discipline: u8, // 自己管理
    pub stamina: u8,    // 持続力
}

impl HardWorkerStats {
    pub fn calculate_polygon_points(&self) -> String {
        let values = vec![
            self.diligence,
            self.focus,
            self.planning,
            self.discipline,
            self.stamina,
        ];

        let center = (125.0, 125.0);
        let radius = 80.0;
        let angle_step = std::f64::consts::PI * 2.0 / 5.0;

        values
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                let angle = angle_step * i as f64 - std::f64::consts::FRAC_PI_2;
                let r = (v as f64 / 100.0) * radius;
                let x = center.0 + r * angle.cos();
                let y = center.1 + r * angle.sin();
                format!("{x},{y}")
            })
            .collect::<Vec<_>>()
            .join(" ")
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

    pub fn title(&self) -> &'static str {
        match self {
            Job::Swordsman => "一撃必殺の剣士",
            Job::AxeWarrior => "力で押し切る破砕者",
            Job::Spearman => "間合いを制する突撃手",
            Job::Knight => "忠義に生きる守護騎士",
            Job::Monk => "気を操る拳の使い手",
            Job::Mage => "禁書を操る魔導師",
            Job::Priest => "癒しと祈りの導き手",
            Job::Summoner => "異界を繋ぐ契約者",
            Job::Sage => "万象を知る賢き者",
            Job::Necromancer => "死霊を操る禁術士",
            Job::Hunter => "森を駆ける追跡者",
            Job::Thief => "影を歩む俊敏者",
            Job::Ninja => "静かなる一閃の影",
            Job::Artificer => "魔技を織り成す創造者",
            Job::Illusionist => "幻を現に変える術士",
            Job::Blacksmith => "鍛冶場の職人魂",
            Job::Alchemist => "物質の理を操る者",
            Job::Cook => "料理は命の燃料",
            Job::Miner => "地底に挑む採掘者",
            Job::Farmer => "実りを育てる生活者",
            Job::Hero => "伝説に名を刻む勇者",
            Job::DemonLord => "絶対的な支配の象徴",
            Job::Merchant => "富を操る取引の達人",
            Job::Jobless => "すべてはここから始まる",
            Job::BeastTamer => "魔獣と歩む調教師",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Job::Swordsman => {
                "剣の扱いに長けた戦闘職。最前線での作業が中心で、持久力と集中力が問われる職務。"
            }
            Job::AxeWarrior => {
                "重装備と高負荷作業に従事する力仕事専門職。全身を使うため体力勝負の業務。"
            }
            Job::Spearman => {
                "距離を保って安定した攻撃を行う職。空間認識力が求められ、安全性重視の働き方。"
            }
            Job::Knight => {
                "防衛・護衛の責務を担う重装職。チームの安全を最優先にする堅実な働きぶりが特徴。"
            }
            Job::Monk => {
                "素手や体術での作業を行う職。自己管理と肉体鍛錬が欠かせないフィジカル系労働者。"
            }
            Job::Mage => {
                "知識と集中力が求められる技術職。長時間の座学・詠唱作業が多く、精神労働系。"
            }
            Job::Priest => {
                "癒しと支援を専門とするケア職。仲間の管理・サポートが中心の人間関係重視型の仕事。"
            }
            Job::Summoner => {
                "召喚管理と調整が主な業務。多タスク処理とコミュニケーション能力が必要な特殊職。"
            }
            Job::Sage => "幅広い業務に対応する総合職。知識と判断力を求められるマルチロール。",
            Job::Necromancer => {
                "管理困難な存在を扱う高ストレス業務。リスク管理と責任感が必要な専門職。"
            }
            Job::Hunter => {
                "自然環境での探索・採集作業に従事。機動力と現場対応力が評価される実務型。"
            }
            Job::Thief => "迅速な移動と判断が求められる運搬・探索系の短期集中型労働。",
            Job::Ninja => {
                "隠密作業と特殊任務に従事。プレッシャー下での作業に慣れた独立系ワーカー。"
            }
            Job::Artificer => {
                "機械や道具を扱う開発・保守系職。精密さとアイデアが活きるクラフト職。"
            }
            Job::Illusionist => "印象操作や視覚効果の専門職。演出系の業務や現場演出に長ける。",
            Job::Blacksmith => "鍛冶と加工に特化した職人職。繰り返し作業と精密さの両立が重要。",
            Job::Alchemist => "化学・薬品系のクラフト職。素材の取り扱いと調整作業に強い。",
            Job::Cook => "食品を扱う準備・提供系の業務。衛生・味覚・スピードが求められる。",
            Job::Miner => {
                "採掘現場での重労働に従事。危険度と引き換えに安定収入が得られる体力勝負の職。"
            }
            Job::Farmer => {
                "農作物の育成と管理を担う自然相手の仕事。季節と天候に左右される日常系職。"
            }
            Job::Hero => "複数の職能を統合したリーダー的職種。常に注目と期待がかかる総合職。",
            Job::DemonLord => "圧倒的な裁量と責任を持つ管理者職。強力だが孤立しやすい。",
            Job::Merchant => "取引・交渉に特化した営業職。利益と信用のバランスを取る経済的ロール。",
            Job::Jobless => {
                "何も決まっていない状態。今後の方向性次第であらゆる業務に就ける柔軟さが特徴。"
            }
            Job::BeastTamer => {
                "動物・魔獣を扱う調教職。管理と信頼関係構築がカギとなるパートナー型職業。"
            }
        }
    }

    pub fn icon_path(&self) -> &'static str {
        match self {
            Job::Swordsman => "jobs/swordsman.png",
            Job::AxeWarrior => "jobs/axewarrior.png",
            Job::Spearman => "jobs/spearman.png",
            Job::Knight => "jobs/knight.png",
            Job::Monk => "jobs/monk.png",
            Job::Mage => "jobs/mage.png",
            Job::Priest => "jobs/priest.png",
            Job::Summoner => "jobs/summoner.png",
            Job::Sage => "jobs/sage.png",
            Job::Necromancer => "jobs/necromancer.png",
            Job::Hunter => "jobs/hunter.png",
            Job::Thief => "jobs/thief.png",
            Job::Ninja => "jobs/ninja.png",
            Job::Artificer => "jobs/artificer.png",
            Job::Illusionist => "jobs/illusionist.png",
            Job::Blacksmith => "jobs/blacksmith.png",
            Job::Alchemist => "jobs/alchemist.png",
            Job::Cook => "jobs/cook.png",
            Job::Miner => "jobs/miner.png",
            Job::Farmer => "jobs/farmer.png",
            Job::Hero => "jobs/hero.png",
            Job::DemonLord => "jobs/demonlord.png",
            Job::Merchant => "jobs/merchant.png",
            Job::Jobless => "jobs/jobless.png",
            Job::BeastTamer => "jobs/beasttamer.png",
        }
    }

    pub fn stats(&self) -> HardWorkerStats {
        match self {
            // Frontline
            Job::Swordsman => HardWorkerStats {
                diligence: 80,
                focus: 60,
                planning: 65,
                discipline: 70,
                stamina: 85,
            },
            Job::AxeWarrior => HardWorkerStats {
                diligence: 75,
                focus: 55,
                planning: 60,
                discipline: 65,
                stamina: 90,
            },
            Job::Spearman => HardWorkerStats {
                diligence: 78,
                focus: 65,
                planning: 70,
                discipline: 68,
                stamina: 82,
            },
            Job::Knight => HardWorkerStats {
                diligence: 85,
                focus: 60,
                planning: 75,
                discipline: 80,
                stamina: 88,
            },
            Job::Monk => HardWorkerStats {
                diligence: 82,
                focus: 70,
                planning: 60,
                discipline: 75,
                stamina: 92,
            },

            // Rearguard
            Job::Mage => HardWorkerStats {
                diligence: 65,
                focus: 90,
                planning: 80,
                discipline: 55,
                stamina: 60,
            },
            Job::Priest => HardWorkerStats {
                diligence: 80,
                focus: 80,
                planning: 75,
                discipline: 80,
                stamina: 70,
            },
            Job::Summoner => HardWorkerStats {
                diligence: 70,
                focus: 85,
                planning: 80,
                discipline: 60,
                stamina: 65,
            },
            Job::Sage => HardWorkerStats {
                diligence: 75,
                focus: 88,
                planning: 85,
                discipline: 70,
                stamina: 68,
            },
            Job::Necromancer => HardWorkerStats {
                diligence: 68,
                focus: 85,
                planning: 78,
                discipline: 55,
                stamina: 70,
            },

            // Special
            Job::Hunter => HardWorkerStats {
                diligence: 72,
                focus: 75,
                planning: 68,
                discipline: 60,
                stamina: 80,
            },
            Job::Thief => HardWorkerStats {
                diligence: 70,
                focus: 80,
                planning: 85,
                discipline: 60,
                stamina: 75,
            },
            Job::Ninja => HardWorkerStats {
                diligence: 75,
                focus: 85,
                planning: 80,
                discipline: 65,
                stamina: 78,
            },
            Job::Artificer => HardWorkerStats {
                diligence: 70,
                focus: 80,
                planning: 88,
                discipline: 68,
                stamina: 62,
            },
            Job::Illusionist => HardWorkerStats {
                diligence: 65,
                focus: 90,
                planning: 85,
                discipline: 55,
                stamina: 60,
            },

            // Life
            Job::Blacksmith => HardWorkerStats {
                diligence: 85,
                focus: 60,
                planning: 65,
                discipline: 75,
                stamina: 88,
            },
            Job::Alchemist => HardWorkerStats {
                diligence: 70,
                focus: 85,
                planning: 80,
                discipline: 65,
                stamina: 70,
            },
            Job::Cook => HardWorkerStats {
                diligence: 80,
                focus: 70,
                planning: 75,
                discipline: 80,
                stamina: 65,
            },
            Job::Miner => HardWorkerStats {
                diligence: 88,
                focus: 55,
                planning: 60,
                discipline: 70,
                stamina: 90,
            },
            Job::Farmer => HardWorkerStats {
                diligence: 95,
                focus: 50,
                planning: 60,
                discipline: 80,
                stamina: 95,
            },

            // Rare
            Job::Hero => HardWorkerStats {
                diligence: 90,
                focus: 85,
                planning: 80,
                discipline: 85,
                stamina: 95,
            },
            Job::DemonLord => HardWorkerStats {
                diligence: 85,
                focus: 95,
                planning: 90,
                discipline: 75,
                stamina: 90,
            },
            Job::Merchant => HardWorkerStats {
                diligence: 75,
                focus: 65,
                planning: 90,
                discipline: 70,
                stamina: 55,
            },
            Job::Jobless => HardWorkerStats {
                diligence: 40,
                focus: 40,
                planning: 40,
                discipline: 40,
                stamina: 40,
            },
            Job::BeastTamer => HardWorkerStats {
                diligence: 75,
                focus: 70,
                planning: 65,
                discipline: 60,
                stamina: 85,
            },
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
