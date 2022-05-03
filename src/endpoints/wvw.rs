

pub async fn abilities(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/abilities", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn ability(
    api_base_url: &str, api_key: &str, id: u64
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/abilities/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn matches(
    api_base_url: &str, api_key: &str
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/matches", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn r#match(
    api_base_url: &str, api_key: &str, id: String
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/matches/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

// todo guild-kdrs

pub async fn matches_overviews(
    api_base_url: &str, api_key: &str
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/matches/overview", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn r#matches_overview(
    api_base_url: &str, api_key: &str, id: String
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/matches/overview/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn matches_scores(
    api_base_url: &str, api_key: &str
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/matches/scores", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn r#matches_score(
    api_base_url: &str, api_key: &str, id: String
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/matches/scores/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn matches_stats(
    api_base_url: &str, api_key: &str
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/matches/stats", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn matches_stat(
    api_base_url: &str, api_key: &str, id: String
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/matches/stats/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn objectives(
    api_base_url: &str, api_key: &str
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/objectives", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn objective(
    api_base_url: &str, api_key: &str, id: String
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/objectives/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn ranks(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/ranks", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn rank(
    api_base_url: &str, api_key: &str, id: u64
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/ranks/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn upgrades(
    api_base_url: &str, api_key: &str
) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/upgrades", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn upgrade(
    api_base_url: &str, api_key: &str, id: u64
) -> serde_json::Value {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/wvw/upgrades/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}
