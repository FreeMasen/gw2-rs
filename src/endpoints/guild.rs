use reqwest_middleware::ClientWithMiddleware as Client;
use uuid::Uuid;

use crate::codec::guild as codec;

pub async fn detail(client: &Client, api_base_url: &str, api_key: &str, id: Uuid) -> codec::Detail {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    super::get_json(
        client
            .get(format!("{}/v2/guild/{}", api_base_url, formatted_id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn log(client: &Client, api_base_url: &str, api_key: &str, id: Uuid) -> Vec<codec::Log> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    super::get_json(
        client
            .get(format!("{}/v2/guild/{}/log", api_base_url, formatted_id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn members(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: Uuid,
) -> Vec<codec::Member> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    super::get_json(
        client
            .get(format!(
                "{}/v2/guild/{}/members",
                api_base_url, formatted_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn permissions(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/guild/permissions", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn permission(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: &str,
) -> codec::Permission {
    super::get_json(
        client
            .get(format!("{}/v2/guild/permissions/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn ranks(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: Uuid,
) -> Vec<codec::Rank> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    super::get_json(
        client
            .get(format!("{}/v2/guild/{}/ranks", api_base_url, formatted_id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn search(client: &Client, api_base_url: &str, api_key: &str, name: &str) -> Vec<Uuid> {
    super::get_json(
        client
            .get(format!("{}/v2/guild/search?name={}", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn stash(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: Uuid,
) -> Vec<codec::Stash> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    super::get_json(
        client
            .get(format!("{}/v2/guild/{}/stash", api_base_url, formatted_id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn teams(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: Uuid,
) -> Vec<codec::Team> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    super::get_json(
        client
            .get(format!("{}/v2/guild/{}/teams", api_base_url, formatted_id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn treasury(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: Uuid,
) -> Vec<codec::Treasury> {
    let formatted_id = format!("{}", id).to_ascii_uppercase();
    super::get_json(
        client
            .get(format!(
                "{}/v2/guild/{}/treasury",
                api_base_url, formatted_id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn upgrades(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/guild/upgrades", api_base_url))
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
) -> codec::Upgrade {
    super::get_json(
        client
            .get(format!("{}/v2/guild/upgrades/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
