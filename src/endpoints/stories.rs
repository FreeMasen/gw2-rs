use reqwest::Client;

use uuid::Uuid;

use crate::{codec::stories as codec, UpperHyphenated};

pub async fn stories(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/stories", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn story(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> codec::Story {
    super::get_json(
        client
            .get(format!("{}/v2/stories/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn seasons(client: &Client, api_base_url: &str, api_key: &str) -> Vec<Uuid> {
    super::get_json(
        client
            .get(format!("{}/v2/stories/seasons", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn season(client: &Client, api_base_url: &str, api_key: &str, id: Uuid) -> codec::Season {
    super::get_json(
        client
            .get(format!(
                "{}/v2/stories/seasons/{}",
                api_base_url,
                id.as_upper_hyphenated()
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
