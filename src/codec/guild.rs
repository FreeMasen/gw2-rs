use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detail {
    pub id: Uuid,
    pub name: String,
    pub tag: String,
    pub emblem: GuildEmblem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEmblem {
    pub background: EmblemColors,
    pub foreground: EmblemColors,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmblemColors {
    pub id: u64,
    pub colors: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub id: u64,
    pub time: String,
    #[serde(flatten, rename = "type")]
    pub kind: LogInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum LogInfo {
    Joined,
    Invited {
        invited_by: String,
    },
    Kick {
        kicked_by: String,
    },
    RankChange {
        changed_by: Option<String>,
        old_rank: String,
        new_rank: String,
    },
    Motd {
        motd: String,
    },
    Stash {
        operation: String,
        item_id: u64,
        count: u64,
        coins: u64,
    },
    Treasury {
        item_id: u64,
        count: u64,
    },
    Upgrade {
        recipe_id: Option<u64>,
        upgrade_id: u64,
        count: Option<u64>,
        action: String,
    },
    Influence {
        activity: String,
        total_participants: u64,
        participants: Vec<serde_json::Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub rank: String,
    pub joined: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rank {
    pub id: String,
    pub order: u64,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stash {
    pub upgrade_id: u64,
    pub size: u64,
    pub coins: u64,
    pub note: Option<String>,
    pub inventory: Vec<Option<StashEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StashEntry {
    pub id: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: u64,
    pub members: Vec<Member>,
    pub name: String,
    pub aggregate: TeamScores,
    pub ladders: HashMap<String, TeamScores>,
    pub games: Vec<TeamGame>,
    pub seasons: Vec<TeamSeason>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamScores {
    pub wins: u64,
    pub losses: u64,
    pub desertions: u64,
    pub byes: u64,
    pub forfeits: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamGame {
    pub id: Uuid,
    pub map_id: u64,
    pub started: String,
    pub ended: String,
    pub result: String,
    pub team: String,
    pub scores: GameScores,
    pub rating_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameScores {
    pub red: u64,
    pub blue: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamSeason {
    pub id: Uuid,
    pub wins: u64,
    pub losses: u64,
    pub rating: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Treasury {
    pub item_id: u64,
    pub count: u64,
    pub needed_by: Vec<TreasuryUpgrade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryUpgrade {
    pub upgrade_id: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub build_time: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub icon: String,
    pub required_level: u64,
    pub experience: u64,
    pub prerequisites: Vec<u64>,
    pub bag_max_items: Option<u64>,
    pub bag_max_coins: Option<u64>,
    pub costs: Vec<UpgradeCost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeCost {
    #[serde(rename = "type")]
    pub kind: String,
    pub count: u64,
    pub name: Option<String>,
    pub item_id: Option<u64>,
}
