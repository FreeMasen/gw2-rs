
use crate::{codec::specializations as codec};

pub async fn specializations(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/specializations", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn specialization(
    api_base_url: &str, api_key: &str, id: u64
) -> codec::Specialization{
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/specializations/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}
