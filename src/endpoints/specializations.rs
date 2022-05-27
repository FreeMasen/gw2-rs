use reqwest::Client;

use crate::codec::specializations as codec;

pub async fn specializations(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/specializations", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn specialization(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> codec::Specialization {
    super::get_json(
        client
            .get(format!("{}/v2/specializations/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
