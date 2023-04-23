use crate::codec::emblems as codec;
use reqwest_middleware::ClientWithMiddleware as Client;

pub async fn emblems(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/emblems", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn foregrounds(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/emblems/foregrounds", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn foreground(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> codec::Emblem {
    super::get_json(
        client
            .get(format!("{}/v2/emblems/foregrounds/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn backgrounds(client: &Client, api_base_url: &str, api_key: &str) -> codec::Emblem {
    super::get_json(
        client
            .get(format!("{}/v2/emblems/backgrounds", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn background(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/emblems/backgrounds/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
