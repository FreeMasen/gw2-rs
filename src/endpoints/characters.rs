use crate::codec::characters as codec;

pub async fn backstory(
    api_base_url: &str, api_key: &str, character: &str,
)  ->codec::BackStory {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/backstory", api_base_url, character))
        .bearer_auth(api_key)).await
}

pub async fn characters(
    api_base_url: &str, api_key: &str
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters", api_base_url))
        .bearer_auth(api_key)).await
}

pub async fn character(
    api_base_url: &str, api_key: &str, name: &str
) -> codec::Character {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn core(
    api_base_url: &str, api_key: &str, name: &str
) -> codec::Core {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/core", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn crafting(
    api_base_url: &str, api_key: &str, name: &str
) -> codec::Crafting {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/crafting", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn equipment(
    api_base_url: &str, api_key: &str, name: &str
) -> codec::Equipment {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/equipment", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn hero_points(
    api_base_url: &str, api_key: &str, name: &str
) -> Vec<String> {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/heropoints", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn recipes(
    api_base_url: &str, api_key: &str, name: &str
) ->codec::Recipes {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/recipes", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn sab(
    api_base_url: &str, api_key: &str, name: &str
) -> codec::Sab {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/sab", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn skills(
    api_base_url: &str, api_key: &str, name: &str
) -> codec::CharacterSkills {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/skills", api_base_url, name))
        .bearer_auth(api_key)).await
    }

pub async fn specializations(
    api_base_url: &str, api_key: &str, name: &str
) -> codec::CharacterSpecializations {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/specializations", api_base_url, name))
        .bearer_auth(api_key)).await
}

pub async fn training(
    api_base_url: &str, api_key: &str, name: &str
) -> codec::CharacterTraining {
    let client = reqwest::Client::new();
    super::get_json(client
        .get(format!("{}/v2/characters/{}/training", api_base_url, name))
        .bearer_auth(api_key)).await
}
