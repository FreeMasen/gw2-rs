use reqwest::Client;

use crate::codec::commerce as codec;

pub async fn transactions(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/transactions", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn transaction_current(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/transactions/current", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn transaction_history(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/transactions/history", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn transaction_history_buys(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<codec::Transaction> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/commerce/transactions/history/buys",
                api_base_url
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn transaction_history_sells(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<codec::Transaction> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/commerce/transactions/history/sells",
                api_base_url
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn transaction_current_buys(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<codec::Transaction> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/commerce/transactions/current/buys",
                api_base_url
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn transaction_current_sells(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Vec<codec::Transaction> {
    super::get_json(
        client
            .get(format!(
                "{}/v2/commerce/transactions/current/sells",
                api_base_url
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn delivery(client: &Client, api_base_url: &str, api_key: &str) -> codec::Delivery {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/delivery", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn exchange(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/exchange", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn exchange_gems(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    count: u64,
) -> codec::Exchange {
    super::get_json(
        client
            .get(format!(
                "{}/v2/commerce/exchange/gems?quantity={}",
                api_base_url, count
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn exchange_coins(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    count: u64,
) -> codec::Exchange {
    super::get_json(
        client
            .get(format!(
                "{}/v2/commerce/exchange/coins?quantity={}",
                api_base_url, count
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn listings(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/listings", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn listing(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> codec::Listings {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/listings/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn prices(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/prices", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn price(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> codec::Prices {
    super::get_json(
        client
            .get(format!("{}/v2/commerce/prices/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
