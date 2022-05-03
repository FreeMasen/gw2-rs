use crate::codec::emblems as codec;


pub async fn emblems(
    api_base_url: &str, api_key: &str,
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/emblems", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn foregrounds(
    api_base_url: &str, api_key: &str,
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/emblems/foregrounds", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn foreground(
    api_base_url: &str, api_key: &str, id: u64
) -> codec::Emblem {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/emblems/foregrounds/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn backgrounds(
    api_base_url: &str, api_key: &str,
) -> codec::Emblem {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/emblems/backgrounds", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn background(
    api_base_url: &str, api_key: &str, id: u64
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/emblems/backgrounds/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}