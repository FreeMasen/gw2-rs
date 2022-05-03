
use crate::{codec::traits as codec};

pub async fn traits(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/traits", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn r#trait(
    api_base_url: &str, api_key: &str, id: u64
) -> codec::Trait {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/traits/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}
