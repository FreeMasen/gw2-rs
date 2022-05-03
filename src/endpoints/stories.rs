
use uuid::Uuid;

use crate::{codec::stories as codec, UpperHyphenated};

pub async fn stories(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/stories", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn story(
    api_base_url: &str, api_key: &str, id: u64
) -> codec::Story {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/stories/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn seasons(
    api_base_url: &str, api_key: &str
) -> Vec<Uuid> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/stories/seasons", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn season(
    api_base_url: &str, api_key: &str, id: Uuid
) -> codec::Season {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/stories/seasons/{}", api_base_url, id.as_upper_hyphenated()))
        .bearer_auth(api_key)).await
}


