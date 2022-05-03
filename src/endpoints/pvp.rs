use uuid::Uuid;

use crate::{codec::pvp as codec, UpperHyphenated};

pub async fn amulets(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/amulets/", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn amulet(
    api_base_url: &str, api_key: &str, id: u64
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/amulets/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn games(
    api_base_url: &str, api_key: &str
) -> Vec<Uuid> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/games/", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn game(
    api_base_url: &str, api_key: &str, id: Uuid
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/games/{}", api_base_url, id.as_upper_hyphenated()))
        .bearer_auth(api_key)).await
}

pub async fn heroes(
    api_base_url: &str, api_key: &str
) -> Vec<Uuid> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/heroes/", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn hero(
    api_base_url: &str, api_key: &str, id: Uuid
) -> codec::Hero {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/heroes/{}", api_base_url, id.as_upper_hyphenated()))
        .bearer_auth(api_key)).await
}

pub async fn ranks(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/ranks/", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn rank(
    api_base_url: &str, api_key: &str, id: u64
) -> codec::Rank {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/ranks/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn seasons(
    api_base_url: &str, api_key: &str
) -> Vec<Uuid> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/seasons/", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn season(
    api_base_url: &str, api_key: &str, id: Uuid
) -> Vec<Uuid> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/seasons/{}", api_base_url, id.as_upper_hyphenated()))
        .bearer_auth(api_key)).await
}

pub async fn season_leaderboard(
    api_base_url: &str, api_key: &str, id: Uuid, region: &str
) -> codec::Leader {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/seasons/{}/leaderboards/legendary/{}", api_base_url, id.as_upper_hyphenated(), region))
        .bearer_auth(api_key)).await
}

pub async fn standings(
    api_base_url: &str, api_key: &str
) -> Vec<codec::Standing> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/standings", api_base_url))
        .bearer_auth(api_key)).await
}
pub async fn stats(
    api_base_url: &str, api_key: &str
) -> Vec<codec::Stats> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/pvp/stats", api_base_url))
        .bearer_auth(api_key)).await
}