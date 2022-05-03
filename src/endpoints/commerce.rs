use crate::codec::commerce as codec;

pub async fn transactions(
    api_base_url: &str, api_key: &str,
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/transactions", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn transaction_current(api_base_url: &str, api_key: &str,) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/transactions/current", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn transaction_history(api_base_url: &str, api_key: &str,) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/transactions/history", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn transaction_history_buys(api_base_url: &str, api_key: &str,) -> Vec<codec::Transaction> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/transactions/history/buys", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn transaction_history_sells(api_base_url: &str, api_key: &str,) -> Vec<codec::Transaction> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/transactions/history/sells", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn transaction_current_buys(api_base_url: &str, api_key: &str,) -> Vec<codec::Transaction> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/transactions/current/buys", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn transaction_current_sells(api_base_url: &str, api_key: &str,) -> Vec<codec::Transaction> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/transactions/current/sells", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn delivery(api_base_url: &str, api_key: &str,) -> codec::Delivery {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/delivery", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn exchange(api_base_url: &str, api_key: &str,) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/exchange", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn exchange_gems(api_base_url: &str, api_key: &str, count: u64) -> codec::Exchange {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/exchange/gems?quantity={}", api_base_url, count))
        .bearer_auth(api_key)).await
}

pub async fn exchange_coins(api_base_url: &str, api_key: &str, count: u64) -> codec::Exchange {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/exchange/coins?quantity={}", api_base_url, count))
        .bearer_auth(api_key)).await
}

pub async fn listings(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/listings", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn listing(api_base_url: &str, api_key: &str, id: u64) -> codec::Listings {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/listings/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn prices(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/prices", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn price(api_base_url: &str, api_key: &str, id: u64) -> codec::Prices {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/commerce/prices/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}


