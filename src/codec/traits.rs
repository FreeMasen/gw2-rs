use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub id: u64,
    pub tier: u64,
    pub order: u64,
    pub name: String,
    pub description: Option<String>,
    pub slot: String,
    #[serde(default)]
    pub facts: Vec<serde_json::Value>,
    #[serde(default)]
    pub traited_facts: Vec<serde_json::Value>,
    #[serde(default)]
    pub skills: Vec<Skill>,
    pub specialization: u64,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    #[serde(default)]
    pub facts: Vec<serde_json::Value>,
    pub description: Option<String>,
    #[serde(default)]
    pub traited_facts: Vec<serde_json::Value>,
    pub icon: String,
    pub id: u64,
}
