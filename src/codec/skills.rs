use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub weapon_type: Option<String>,
    pub professions: Vec<String>,
    pub slot: String,
    pub icon: String,
    pub flags: Vec<String>,
    pub id: u64,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub transform_skills: Vec<u64>,
    #[serde(default)]
    pub bundle_skills: Vec<u64>,
    pub toolbelt_skill: Option<u64>,
    pub flip_skill: Option<u64>,
    pub cost: Option<u64>,
    pub initiative: Option<u64>,
}
