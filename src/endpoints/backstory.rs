use crate::codec::backstory as codec;
use reqwest_middleware::ClientWithMiddleware as Client;

pub async fn answers(client: &Client, api_base_url: &str, api_key: &str) -> Vec<String> {
    super::get_json(
        client
            .get(format!("{}/v2/backstory/answers", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn answer(client: &Client, api_base_url: &str, api_key: &str, id: &str) -> codec::Answer {
    super::get_json(
        client
            .get(format!("{}/v2/backstory/answers/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn questions(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    super::get_json(
        client
            .get(format!("{}/v2/backstory/questions", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn question(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> codec::Question {
    super::get_json(
        client
            .get(format!("{}/v2/backstory/questions/{}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
