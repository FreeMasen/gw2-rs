
use crate::{codec::skills as codec};

pub async fn skills(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/skills/", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn skill(
    api_base_url: &str, api_key: &str, id: u64
) -> codec::Skill{
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/skills/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}
