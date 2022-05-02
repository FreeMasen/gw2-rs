use crate::codec::{account as codec, Cat, Node};

pub async fn account(api_base_url: &str, api_key: &str) -> codec::Account {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn achievements(api_base_url: &str, api_key: &str) -> Vec<codec::Achievement> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/achievements", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn achievement(api_base_url: &str, api_key: &str, id: u64) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/achievements?id={}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn bank(api_base_url: &str, api_key: &str) -> Vec<Option<codec::BankItem>> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/bank", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn dungeons(api_base_url: &str, api_key: &str) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/dungeons", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn dyes(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/dyes", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn finishers(api_base_url: &str, api_key: &str) -> Vec<codec::Finisher> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/finishers", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn gliders(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/finishers", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn home_cats(api_base_url: &str, api_key: &str)  -> Vec<Cat> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/home/cats", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn home_nodes(api_base_url: &str, api_key: &str)  -> Vec<Node> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/home/nodes", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn inventory(api_base_url: &str, api_key: &str) -> Vec<Option<codec::Inventory>> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/inventory", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn mailcarriers(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/mailcarriers", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn masteries(api_base_url: &str, api_key: &str) -> Vec<codec::Mastery> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/masteries", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn mastery_points(api_base_url: &str, api_key: &str) -> codec::MasteryPoints {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/mastery/points", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn materials(api_base_url: &str, api_key: &str) -> Vec<codec::Material> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/materials", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn minis(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/minis", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn outfits(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/minis", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn pvp_heros(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/pvp/heroes", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn raids(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/raids", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn recipes(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/recipes", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn skins(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/skins", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn titles(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/titles", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn wallet(api_base_url: &str, api_key: &str) -> Vec<codec::Wallet> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/account/wallet", api_base_url))
        .bearer_auth(api_key)).await
}
