use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub id: String,
    pub title: String,
    pub description: String,
    pub journal: String,
    pub question: u64,
    #[serde(default)]
    pub professions: Vec<String>,
    #[serde(default)]
    pub races: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub answers: Vec<String>,
    pub order: u64,
    #[serde(default)]
    pub races: Vec<String>,
    #[serde(default)]
    pub professions: Vec<String>,
}
