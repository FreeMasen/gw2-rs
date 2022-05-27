use reqwest::Client;

use crate::codec::token_info::Token;

pub async fn token_info(client: &Client, api_base_url: &str, api_key: &str) -> Token {
    super::get_json(
        client
            .get(format!("{}/v2/tokeninfo", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
