use reqwest_middleware::ClientWithMiddleware as Client;

pub async fn abilities(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/abilities", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn ability(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/abilities/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn matches(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/matches", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn r#match(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: String,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/matches/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

// todo guild-kdrs

pub async fn matches_overviews(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/matches/overview", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn r#matches_overview(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: String,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/matches/overview/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn matches_scores(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/matches/scores", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn r#matches_score(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: String,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/matches/scores/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn matches_stats(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/matches/stats", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn matches_stat(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: String,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/matches/stats/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn objectives(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/objectives", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn objective(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: String,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/objectives/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn ranks(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/ranks", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn rank(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/ranks/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn upgrades(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/upgrades", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn upgrade(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> serde_json::Value {
    super::get_json(
        client
            .get(format!("{}/v2/wvw/upgrades/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
