use uuid::Uuid;

pub use crate::codec::{achievements as codec};

pub async fn achievements(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/achievements", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn achievement(
    api_base_url: &str, api_key: &str, id: u64
) -> codec::Achievement {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/achievements/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn categories(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/achievements/categories", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn category(api_base_url: &str, api_key: &str, id: u64) -> codec::Category {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/achievements/categories/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn daily(api_base_url: &str, api_key: &str) -> Vec<codec::Daily> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/achievements/daily", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn groups(api_base_url: &str, api_key: &str) -> Vec<Uuid> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/achievements/groups", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn group(api_base_url: &str, api_key: &str, id: impl std::fmt::Display) -> codec::Group {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/achievements/groups/{}", api_base_url, formatted_id))
        .bearer_auth(api_key)).await
}
