use reqwest::Client;
use uuid::Uuid;

pub use crate::codec::achievements as codec;

pub async fn achievements(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/achievements", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn achievement(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> codec::Achievement {
    super::get_json(
        client
            .get(format!("{}/v2/achievements/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn categories(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/achievements/categories", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn category(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> codec::Category {
    super::get_json(
        client
            .get(format!(
                "{}/v2/achievements/categories/{}",
                api_base_url, id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn daily(client: &Client, api_base_url: &str, api_key: &str) -> Vec<codec::Daily> {
    super::get_json(
        client
            .get(format!("{}/v2/achievements/daily", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn groups(client: &Client, api_base_url: &str, api_key: &str) -> Vec<Uuid> {
    super::get_json(
        client
            .get(format!("{}/v2/achievements/groups", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn group(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: impl std::fmt::Display,
) -> codec::Group {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    super::get_json(
        client
            .get(format!(
                "{}/v2/achievements/groups/{}",
                api_base_url, formatted_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
