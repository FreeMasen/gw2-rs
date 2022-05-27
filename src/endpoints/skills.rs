use reqwest::Client;

use crate::codec::skills as codec;

pub async fn skills(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/skills/", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn skill(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> codec::Skill {
    super::get_json(
        client
            .get(format!("{}/v2/skills/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
