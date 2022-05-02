use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackStory {
    pub backstory: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub race: String,
    pub gender: String,
    pub profession: String,
    pub level: u8,
    pub guild: Uuid,
    pub created: String,
    pub age: u64,
    pub deaths: u64,
    pub title: u64,
    pub crafting: Vec<Craft>,
    pub backstory: Vec<u64>,
    pub equipment: Vec<EquipmentPiece>,
    pub bags: Vec<Option<Bag>>,
    pub recipes: Vec<u64>,
    #[serde(default)]
    pub wvw_abilities: Vec<WvwAbility>,
    pub skills: Option<Skills>,
    pub specializations: Option<Specializations>,
    pub equipment_pvp: Option<PvPEquipmentPiece>,
    #[serde(default)]
    pub training: Vec<Training>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Craft {
    pub discipline: String,
    pub active: bool,
    pub rating: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentPiece {
    pub id: u64,
    pub slot: String,
    pub binding: String,
    pub bound_to: Option<String>,
    pub stats: Option<super::ItemStats>,
    #[serde(default)]
    pub dyes: Vec<Option<u64>>,
    #[serde(default)]
    pub upgrades: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bag {
    pub id: u64,
    pub size: u64,
    pub inventory: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WvwAbility {
    pub id: u64,
    pub rank: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub pve: Skill,
    pub pvp: Skill,
    pub wvw: Skill,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    #[serde(default)]
    pub legends: Vec<String>,
    pub heal: u64,
    pub utility: Vec<u64>,
    pub elite: u64,
    pub pets: Option<PetSkills>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetSkills {
    terrestrial: Vec<u64>,
    aquatic: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specializations {
    pve: Vec<Option<Specialization>>,
    pvp: Vec<Option<Specialization>>,
    wvw: Vec<Option<Specialization>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specialization {
    pub id: u64,
    pub traits: Vec<Option<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PvPEquipmentPiece {
    pub amulet: u64,
    pub rune: u64,
    pub sigils: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Training {
    pub id: u64,
    pub spent: u64,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Core {
    pub name: String,
    pub race: String,
    pub gender: String,
    pub profession: String,
    pub level: u64,
    pub guild: Uuid,
    pub created: String,
    pub age: u64,
    pub deaths: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crafting {
    pub crafting: Vec<Craft>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub equipment: Vec<EquipmentPiece>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sab {
    pub zones: Vec<SabZone>,
    pub unlocks: Vec<SabUnlock>,
    pub songs: Vec<SabSong>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabZone {
    pub id: u64,
    pub mode: String,
    pub world: u64,
    pub zone: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabUnlock {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabSong {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkills {
    pub skills: Skills,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSpecializations {
    pub specializations: Specializations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTraining {
    pub training: Vec<Training>,
}
