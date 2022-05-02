use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specialization {
    pub id: u64,
    pub name: String,
    pub profession: String,
    pub elite: bool,
    pub major_traits: Vec<u64>,
    pub minor_traits: Vec<u64>,
    pub icon: String,
    pub background: String,
}
