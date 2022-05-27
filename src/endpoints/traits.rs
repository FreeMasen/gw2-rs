use reqwest::Client;

use crate::codec::traits as codec;

pub async fn traits(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/traits", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn r#trait(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> codec::Trait {
    super::get_json(
        client
            .get(format!("{}/v2/traits/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
