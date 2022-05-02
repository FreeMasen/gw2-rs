use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amulet {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub map_id: u64,
    pub started: String,
    pub ended: String,
    pub result: String,
    pub team: String,
    pub profession: String,
    pub scores: GameScores,
    pub rating_type: String,
    pub rating_change: u64,
    pub season: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameScores {
    pub red: u64,
    pub blue: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub stats: HeroStats,
    pub skins: Vec<HeroSkin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroStats {
    pub offense: u64,
    pub defense: u64,
    pub speed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroSkin {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub unlock_items: Vec<u64>,
    pub default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rank {
    pub id: u64,
    pub finisher_id: u64,
    pub name: String,
    pub icon: String,
    pub min_rank: u64,
    pub max_rank: u64,
    pub levels: Vec<RankLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankLevel {
    pub min_rank: u64,
    pub max_rank: u64,
    pub points: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leader {
    pub name: String,
    pub rank: u64,
    pub date: String,
    pub scores: Vec<LeaderScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderScore {
    pub id: Uuid,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionLeader {
    pub id: Uuid,
    pub name: String,
    pub team: String,
    pub team_id: u64,
    pub rank: u64,
    pub date: String,
    pub scores: Vec<TeamScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamScore {
    pub id: Uuid,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: Uuid,
    pub name: String,
    pub start: String,
    pub end: String,
    pub active: bool,
    pub divisions: Vec<SeasonDivision>,
    pub ranks: Vec<SeasonRank>,
    //TODO: type this
    pub leaderboards: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonDivision {
    pub name: String,
    pub flags: Vec<String>,
    pub large_icon: String,
    pub small_icon: String,
    pub pip_icon: String,
    pub tiers: Vec<DivisionTier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivisionTier {
    pub points: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonRank {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub overlay: String,
    pub overlay_small: String,
    pub tiers: Vec<SeasonRankTiers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonRankTiers {
    pub rating: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standing {
    pub current: CurrentStandings,
    pub best: BestStandings,
    pub season_id: Uuid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentStandings {
    pub total_points: u64,
    pub division: u64,
    pub tier: u64,
    pub points: u64,
    pub repeats: u64,
    pub rating: u64,
    pub decay: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestStandings {
    pub total_points: u64,
    pub division: u64,
    pub tier: u64,
    pub points: u64,
    pub repeats: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub pvp_rank: u64,
    pub pvp_rank_points: u64,
    pub pvp_rank_rollovers: u64,
    pub aggregate: HashMap<String, u64>,
    // TODO...
    pub professions: HashMap<String, HashMap<String, u64>>,
    // TODO...
    pub ladders: HashMap<String, HashMap<String, u64>>,
}
