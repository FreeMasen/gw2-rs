use crate::codec::floors as codec;



pub async fn continents(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn continent(
    api_base_url: &str, api_key: &str, continent_id: u64,
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}", api_base_url, continent_id))
        .bearer_auth(api_key)).await
}

pub async fn floors(
    api_base_url: &str, api_key: &str, contenent_id: u64
) -> Vec<i64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/", api_base_url,contenent_id))
        .bearer_auth(api_key)).await
}

pub async fn floor(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64
) -> codec::Floor {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}", api_base_url,contenent_id, floor_id))
        .bearer_auth(api_key)).await
}

pub async fn regions(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/", api_base_url,contenent_id, floor_id))
        .bearer_auth(api_key)).await
}

pub async fn maps(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64, region_id: u64
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/{}/maps", api_base_url,contenent_id, floor_id, region_id))
        .bearer_auth(api_key)).await
}

pub async fn map(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64, region_id: u64, map_id: u64
) -> codec::Map {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/{}/maps/{}", api_base_url,contenent_id, floor_id, region_id, map_id))
        .bearer_auth(api_key)).await
}

pub async fn sector(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64, region_id: u64, map_id: u64, sector_id: u64
) -> codec::Floor {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/sectors/{}", api_base_url,contenent_id, floor_id, region_id, map_id, sector_id))
        .bearer_auth(api_key)).await
}

pub async fn tasks(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64, region_id: u64, map_id: u64
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/tasks/", api_base_url,contenent_id, floor_id, region_id, map_id))
        .bearer_auth(api_key)).await
}

pub async fn task(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64, region_id: u64, map_id: u64, task_id: u64
) -> codec::Task {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/tasks/{}", api_base_url,contenent_id, floor_id, region_id, map_id, task_id))
        .bearer_auth(api_key)).await
}

pub async fn pois(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64, region_id: u64, map_id: u64,
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/pois", api_base_url,contenent_id, floor_id, region_id, map_id))
        .bearer_auth(api_key)).await
}

pub async fn poi(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64, region_id: u64, map_id: u64, poi_id: u64,
) -> codec::Poi {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/{}/maps/{}/pois/{}", api_base_url,contenent_id, floor_id, region_id, map_id, poi_id))
        .bearer_auth(api_key)).await
}


pub async fn region(
    api_base_url: &str, api_key: &str, contenent_id: u64, floor_id: i64, region_id: u64
) -> codec::Region {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/continents/{}/floors/{}/regions/{}", api_base_url,contenent_id, floor_id, region_id))
        .bearer_auth(api_key)).await
}