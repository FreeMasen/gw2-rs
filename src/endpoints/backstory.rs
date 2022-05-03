use crate::codec::backstory as codec;

pub async fn answers(
    api_base_url: &str, api_key: &str
)  -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/backstory/answers", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn answer(
    api_base_url: &str, api_key: &str, id: &str,
) -> codec::Answer {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/backstory/answers/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}

pub async fn questions(
    api_base_url: &str, api_key: &str
)  -> Vec<u64> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/backstory/questions", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn question(
    api_base_url: &str, api_key: &str, id: u64
)  -> codec::Question {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/backstory/questions/{}", api_base_url, id))
        .bearer_auth(api_key)).await
}
