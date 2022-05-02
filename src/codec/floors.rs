use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floor {
    pub texture_dims: Vec<u64>,
    pub regions: HashMap<String, Region>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: u64,
    pub name: String,
    pub label_coord: Vec<f64>,
    pub maps: HashMap<String, Map>,
    #[serde(default)]
    pub content_rect: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub min_level: u64,
    pub max_level: u64,
    pub default_floor: u64,
    pub map_rect: Vec<Vec<i64>>,
    pub continent_rect: Vec<Vec<i64>>,
    pub adventures: Vec<Adventure>,
    pub points_of_interest: HashMap<String, Poi>,
    pub tasks: HashMap<String, Task>,
    pub skill_challenges: HashMap<String, SkillChallenge>,
    pub sectors: HashMap<String, Sector>,
    pub god_shrines: Vec<GodShrine>,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adventure {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub coords: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poi {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    pub coord: Vec<f64>,
    pub id: u64,
    pub floor: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub objective: String,
    pub level: u64,
    pub coord: Vec<f64>,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillChallenge {
    pub coord: Vec<f64>,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sector {
    pub name: String,
    pub level: u64,
    pub coord: Vec<f64>,
    pub id: u64,
    pub bounds: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GodShrine {
    pub id: u64,
    pub name: String,
    pub coord: Vec<f64>,
    pub icon: String,
    pub icon_contested: String,
}
