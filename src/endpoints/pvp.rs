use reqwest_middleware::ClientWithMiddleware as Client;
use uuid::Uuid;

use crate::{codec::pvp as codec, UpperHyphenated};

pub async fn amulets(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/amulets/", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn amulet(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/amulets/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn games(client: &Client, api_base_url: &str, api_key: &str) -> Vec<Uuid> {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/games/", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn game(client: &Client, api_base_url: &str, api_key: &str, id: Uuid) -> Vec<u64> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/pvp/games/{}",
                api_base_url,
                id.as_upper_hyphenated()
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn heroes(client: &Client, api_base_url: &str, api_key: &str) -> Vec<Uuid> {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/heroes/", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn hero(client: &Client, api_base_url: &str, api_key: &str, id: Uuid) -> codec::Hero {
    super::get_json(
        client
            .get(format!(
                "{}/v2/pvp/heroes/{}",
                api_base_url,
                id.as_upper_hyphenated()
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn ranks(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/ranks/", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn rank(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> codec::Rank {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/ranks/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn seasons(client: &Client, api_base_url: &str, api_key: &str) -> Vec<Uuid> {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/seasons/", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn season(client: &Client, api_base_url: &str, api_key: &str, id: Uuid) -> Vec<Uuid> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/pvp/seasons/{}",
                api_base_url,
                id.as_upper_hyphenated()
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn season_leaderboard(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: Uuid,
    region: &str,
) -> codec::Leader {
    super::get_json(
        client
            .get(format!(
                "{}/v2/pvp/seasons/{}/leaderboards/legendary/{}",
                api_base_url,
                id.as_upper_hyphenated(),
                region
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn standings(client: &Client, api_base_url: &str, api_key: &str) -> Vec<codec::Standing> {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/standings", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
pub async fn stats(client: &Client, api_base_url: &str, api_key: &str) -> Vec<codec::Stats> {
    super::get_json(
        client
            .get(format!("{}/v2/pvp/stats", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
