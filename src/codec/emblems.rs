use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emblem {
    pub id: Vec<u64>,
    pub layers: Vec<String>,
}
