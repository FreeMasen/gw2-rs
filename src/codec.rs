use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub mod account;
pub mod achievements;
pub mod backstory;
pub mod characters;
pub mod commerce;
pub mod emblems;
pub mod floors;
pub mod guild;
pub mod pvp;
pub mod skills;
pub mod specializations;
pub mod stories;
pub mod token_info;
pub mod traits;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cat {
    pub id: u64,
    pub hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub id: u64,
    pub name: String,
    pub base_rgb: [u8; 3],
    pub cloth: MaterialColor,
    pub leather: MaterialColor,
    pub metal: MaterialColor,
    pub fur: MaterialColor,
    pub item: Option<u64>,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialColor {
    pub brightness: i32,
    pub contrast: f64,
    pub hue: u32,
    pub saturation: f64,
    pub lightness: f64,
    pub rgb: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub order: u64,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dungeon {
    pub id: String,
    pub paths: Vec<DungeonPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonPath {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finisher {
    pub id: u64,
    pub unlock_details: String,
    pub unlock_items: Vec<u64>,
    pub order: u64,
    pub icon: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glider {
    pub id: u64,
    pub unlock_items: Vec<u64>,
    pub order: u64,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub default_dyes: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub level: u64,
    pub rarity: String,
    pub vender_value: u64,
    pub game_types: String,
    pub flags: Vec<()>,
    pub restrictions: Vec<()>,
    pub chat_link: String,
    pub icon: String,
    pub details: ItemDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDetails {
    #[serde(rename = "type")]
    pub kind: String,
    pub damage_type: String,
    pub min_power: u64,
    pub max_power: u64,
    pub defense: u64,
    pub infusion_slots: Vec<u64>,
    pub attribute_adjustment: f64,
    pub secondary_suffix_item_id: u64,
    pub infix_upgrade: Upgrade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    pub id: u64,
    pub attributes: Vec<UpgradeAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeAttribute {
    pub attribute: String,
    pub modifier: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStats {
    pub name: String,
    pub attributes: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legend {
    pub id: u64,
    pub swap: u64,
    pub heal: u64,
    pub elite: u64,
    pub utilities: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailCarrier {
    pub id: u64,
    pub unlock_items: Vec<u64>,
    pub order: u64,
    pub icon: String,
    pub name: String,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mastery {
    pub id: u64,
    pub name: String,
    pub requirement: String,
    pub order: u64,
    pub background: String,
    pub region: String,
    pub levels: Vec<MasteryLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryLevel {
    pub name: String,
    pub description: String,
    pub instruction: String,
    pub icon: String,
    pub point_cost: u8,
    pub exp_cost: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub id: u64,
    pub name: String,
    pub order: u64,
    pub items: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mini {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub item_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outfit {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub unlock_items: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub skills: Vec<PetSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetSkill {
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profession {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub icon_big: String,
    pub specialization: Vec<u64>,
    pub training: Vec<ProfessionTraining>,
    pub weapons: HashMap<String, Weapon>,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessionTraining {
    pub id: u64,
    pub category: String,
    pub name: String,
    pub track: TrainingTrack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingTrack {
    pub cost: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub trait_id: Option<u64>,
    pub skill_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub flags: Vec<String>,
    pub skills: Vec<Skill>,
    pub specialization: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: u64,
    pub slot: String,
    pub offhand: Option<String>,
    pub attunement: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Race {
    pub id: String,
    pub name: String,
    pub skills: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raid {
    pub id: String,
    pub wings: Vec<RaidWing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidWing {
    pub id: String,
    pub events: Vec<RaidWingEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidWingEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u64,
    pub chat_link: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub output_item_id: u64,
    pub output_item_count: u64,
    pub min_rating: u64,
    pub time_to_craft_ms: u64,
    pub disciplines: Vec<String>,
    pub flags: Vec<String>,
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub item_id: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skin {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub icon: String,
    pub flags: Vec<String>,
    pub rarity: String,
    pub restrictions: Vec<String>,
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: u64,
    pub name: String,
    pub achievement: Option<u64>,
    pub achievements: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub id: u64,
    pub name: String,
    pub population: String,
}
