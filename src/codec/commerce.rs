use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub item_id: u64,
    pub price: u64,
    pub quantity: u64,
    pub created: String,
    pub purchased: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delivery {
    pub coins: u64,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exchange {
    pub coins_per_gem: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listings {
    pub id: u64,
    pub buys: Vec<Listing>,
    pub sells: Vec<Listing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub listings: u64,
    pub unit_price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prices {
    pub id: u64,
    pub whitelisted: bool,
    pub buys: Price,
    pub sells: Price,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub unit_price: u64,
    pub quantity: u64,
}
