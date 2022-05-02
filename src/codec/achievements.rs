use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub order: u64,
    pub icon: String,
    pub achievements: Vec<u64>,
    #[serde(default)]
    pub required_access: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Daily {
    pve: Vec<DailyAchievement>,
    pvp: Vec<DailyAchievement>,
    wvw: Vec<DailyAchievement>,
    fractal: Vec<DailyAchievement>,
    special: Vec<DailyAchievement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyAchievement {
    pub id: u64,
    pub level: AchievementLevel,
    #[serde(default)]
    pub required_access: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementLevel {
    min: u64,
    max: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub order: u64,
    pub categories: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: u64,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub requirement: String,
    pub locked_text: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub flags: Vec<String>,
    #[serde(default)]
    pub tiers: Vec<AchievementTier>,
    #[serde(default)]
    pub rewards: Vec<AchievementReward>,
    #[serde(default)]
    pub bits: Vec<AchievementBit>,
    #[serde(default)]
    pub prerequisites: Vec<u64>,
    pub points_cap: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementTier {
    pub count: u64,
    pub points: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementReward {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementBit {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
}
