use reqwest::Client;

use crate::codec::floors as codec;

pub async fn continents(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/continents/", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn continent(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    continent_id: u64,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/continents/{}", api_base_url, continent_id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn floors(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
) -> Vec<i64> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/",
                api_base_url, contenent_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn floor(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
) -> codec::Floor {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}",
                api_base_url, contenent_id, floor_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn regions(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
) -> Vec<u64> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/",
                api_base_url, contenent_id, floor_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn maps(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
    region_id: u64,
) -> Vec<u64> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/{}/maps",
                api_base_url, contenent_id, floor_id, region_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn map(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
    region_id: u64,
    map_id: u64,
) -> codec::Map {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/{}/maps/{}",
                api_base_url, contenent_id, floor_id, region_id, map_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn sector(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
    region_id: u64,
    map_id: u64,
    sector_id: u64,
) -> codec::Floor {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/sectors/{}",
                api_base_url, contenent_id, floor_id, region_id, map_id, sector_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn tasks(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
    region_id: u64,
    map_id: u64,
) -> Vec<u64> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/tasks/",
                api_base_url, contenent_id, floor_id, region_id, map_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn task(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
    region_id: u64,
    map_id: u64,
    task_id: u64,
) -> codec::Task {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/tasks/{}",
                api_base_url, contenent_id, floor_id, region_id, map_id, task_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn pois(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
    region_id: u64,
    map_id: u64,
) -> Vec<u64> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/pois",
                api_base_url, contenent_id, floor_id, region_id, map_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn poi(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
    region_id: u64,
    map_id: u64,
    poi_id: u64,
) -> codec::Poi {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/pois/{}",
                api_base_url, contenent_id, floor_id, region_id, map_id, poi_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn region(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    contenent_id: u64,
    floor_id: i64,
    region_id: u64,
) -> codec::Region {
    super::get_json(
        client
            .get(format!(
                "{}/v2/continents/{}/floors/{}/regions/{}",
                api_base_url, contenent_id, floor_id, region_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
