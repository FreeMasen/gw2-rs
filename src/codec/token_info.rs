use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: Uuid,
    pub name: String,
    pub permissions: Vec<String>,
}
