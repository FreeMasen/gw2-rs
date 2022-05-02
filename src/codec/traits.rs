use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub id: u64,
    pub tier: u64,
    pub order: u64,
    pub name: String,
    pub description: String,
    pub slot: String,
    pub facts: Vec<serde_json::Value>,
    pub traited_facts: Vec<serde_json::Value>,
    pub skills: Vec<Skill>,
    pub specialization: u64,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub facts: Vec<serde_json::Value>,
    pub description: String,
    pub traited_facts: Vec<serde_json::Value>,
    pub icon: String,
    pub id: u64,
}
