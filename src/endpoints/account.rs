use crate::codec::{account as codec, Cat, Node};
use reqwest::Client;

pub async fn account(client: &Client, api_base_url: &str, api_key: &str) -> codec::Account {
    super::get_json(
        client
            .get(format!("{}/v2/account", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn achievements(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<codec::Achievement> {
    super::get_json(
        client
            .get(format!("{}/v2/account/achievements", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn achievement(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> Vec<u64> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/account/achievements?id={}",
                api_base_url, id
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn bank(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<Option<codec::BankItem>> {
    super::get_json(
        client
            .get(format!("{}/v2/account/bank", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn dungeons(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/account/dungeons", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn dyes(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/dyes", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn finishers(client: &Client, api_base_url: &str, api_key: &str) -> Vec<codec::Finisher> {
    super::get_json(
        client
            .get(format!("{}/v2/account/finishers", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn gliders(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/finishers", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn home_cats(client: &Client, api_base_url: &str, api_key: &str) -> Vec<Cat> {
    super::get_json(
        client
            .get(format!("{}/v2/account/home/cats", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn home_nodes(client: &Client, api_base_url: &str, api_key: &str) -> Vec<Node> {
    super::get_json(
        client
            .get(format!("{}/v2/account/home/nodes", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn inventory(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<Option<codec::Inventory>> {
    super::get_json(
        client
            .get(format!("{}/v2/account/inventory", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn luck(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Result<Vec<codec::Luck>, crate::Error> {
    super::get_json(
        client
            .get(format!("{}/v2/account/luck", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn mailcarriers(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/mailcarriers", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn masteries(client: &Client, api_base_url: &str, api_key: &str) -> Vec<codec::Mastery> {
    super::get_json(
        client
            .get(format!("{}/v2/account/masteries", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn mastery_points(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> codec::MasteryPoints {
    super::get_json(
        client
            .get(format!("{}/v2/account/mastery/points", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn materials(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Result<Vec<codec::Material>, crate::Error> {
    super::get_json(
        client
            .get(format!("{}/v2/account/materials", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn minis(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/minis", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn outfits(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/minis", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn pvp_heros(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/pvp/heroes", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn raids(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/raids", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn recipes(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/recipes", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn skins(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/skins", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn titles(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/account/titles", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn wallet(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Result<Vec<codec::Wallet>, crate::Error> {
    super::get_json(
        client
            .get(format!("{}/v2/account/wallet", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}
