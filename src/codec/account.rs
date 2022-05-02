use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub age: u64,
    pub name: String,
    pub world: u64,
    pub commander: bool,
    pub guilds: Vec<Uuid>,
    pub access: Vec<String>,
    pub created: String,
    #[serde(default)]
    pub guild_leader: Vec<Uuid>,
    pub fractal_level: Option<u32>,
    pub daily_ap: Option<u64>,
    pub monthly_ap: Option<u64>,
    pub wvw_rank: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: u64,
    pub current: u64,
    pub max: u64,
    pub done: bool,
    pub bits: Option<Vec<u32>>,
    pub unlocked: Option<bool>,
    pub repeated: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankItem {
    pub id: u64,
    pub slot: Option<u64>,
    pub count: u64,
    #[serde(default)]
    pub upgrades: Vec<u64>,
    pub skin: Option<u64>,
    pub charges: Option<u64>,
    pub stats: Option<super::ItemStats>,
    pub binding: Option<String>,
    pub bound_to: Option<String>,
    pub infusions: Option<Vec<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finisher {
    pub id: u64,
    pub permanent: bool,
    pub quantity: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub id: u64,
    pub count: u64,
    pub charges: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mastery {
    pub id: u64,
    pub level: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryPoints {
    pub totals: Vec<MasteryPointTotal>,
    pub unlocked: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryPointTotal {
    pub region: String,
    pub spent: u64,
    pub earned: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub id: u64,
    pub category: u64,
    pub count: u64,
    pub binding: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: u64,
    pub value: u64,
}
