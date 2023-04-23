use crate::codec::characters as codec;
use reqwest_middleware::ClientWithMiddleware as Client;

use super::get_paged;

pub async fn backstory(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> codec::BackStory {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}/backstory", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn characters(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Result<Vec<String>, crate::Error> {
    super::get_json(
        client
            .get(format!("{}/v2/characters", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn get_all_characters(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Result<Vec<codec::Character>, crate::Error> {
    let url = format!("{}/v2/characters", api_base_url);
    let mut initial = get_paged(&client, &url, api_key, 0, 50).await.unwrap();

    for i in 1..initial.page.total_pages.unwrap_or(1) {
        let iter = get_paged(&client, &url, api_key, i, 50).await.unwrap();
        initial.result.extend(iter.result);
    }
    Ok(initial.result)
}

pub async fn character(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> Result<codec::Character, crate::Error> {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn core(client: &Client, api_base_url: &str, api_key: &str, name: &str) -> codec::Core {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}/core", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn crafting(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> codec::Crafting {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}/crafting", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn equipment(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> Result<codec::Equipment, crate::Error> {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}/equipment", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn hero_points(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> Vec<String> {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!(
                "{}/v2/characters/{}/heropoints",
                api_base_url, name
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn recipes(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> codec::Recipes {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}/recipes", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn sab(client: &Client, api_base_url: &str, api_key: &str, name: &str) -> codec::Sab {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}/sab", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn skills(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> codec::CharacterSkills {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}/skills", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn specializations(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> codec::CharacterSpecializations {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!(
                "{}/v2/characters/{}/specializations",
                api_base_url, name
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn training(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    name: &str,
) -> codec::CharacterTraining {
    let name = pct_str::PctString::encode(name.chars(), pct_str::URIReserved);
    super::get_json(
        client
            .get(format!("{}/v2/characters/{}/training", api_base_url, name))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
