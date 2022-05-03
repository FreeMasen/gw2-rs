use uuid::Uuid;

use crate::codec::guild as codec;

pub async fn detail(api_base_url: &str, api_key: &str, id: Uuid) -> codec::Detail {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/{}", api_base_url, formatted_id))
        .bearer_auth(api_key)).await
}

pub async fn log(api_base_url: &str, api_key: &str, id: Uuid) -> Vec<codec::Log> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/{}/log", api_base_url, formatted_id))
        .bearer_auth(api_key)).await
}

pub async fn members(api_base_url: &str, api_key: &str, id: Uuid) -> Vec<codec::Member> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/{}/members", api_base_url, formatted_id))
        .bearer_auth(api_key)).await
}

pub async fn permissions(api_base_url: &str, api_key: &str) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/permissions", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn permission(api_base_url: &str, api_key: &str, id: &str) -> codec::Permission {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/permissions/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn ranks(api_base_url: &str, api_key: &str, id: Uuid) -> Vec<codec::Rank> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/{}/ranks", api_base_url, formatted_id))
        .bearer_auth(api_key)).await
}

pub async fn search(api_base_url: &str, api_key: &str, name: &str) -> Vec<Uuid> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/search?name={}", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn stash(api_base_url: &str, api_key: &str, id: Uuid) -> Vec<codec::Stash> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/{}/stash", api_base_url, formatted_id))
        .bearer_auth(api_key)).await
}

pub async fn teams(api_base_url: &str, api_key: &str, id: Uuid) -> Vec<codec::Team> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/{}/teams", api_base_url, formatted_id))
        .bearer_auth(api_key)).await
}

pub async fn treasury(api_base_url: &str, api_key: &str, id: Uuid) -> Vec<codec::Treasury> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/{}/treasury", api_base_url, formatted_id))
        .bearer_auth(api_key)).await
}

pub async fn upgrades(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/upgrades", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn upgrade(api_base_url: &str, api_key: &str, id: u64) -> codec::Upgrade {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/guild/upgrades/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

