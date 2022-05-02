use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: Uuid,
    pub name: String,
    pub order: u64,
    pub stories: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub id: u64,
    pub season: Uuid,
    pub name: String,
    pub description: String,
    pub timeline: String,
    pub level: u64,
    pub races: Vec<String>,
    pub order: u64,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub name: String,
}
