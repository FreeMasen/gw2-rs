use serde::de::DeserializeOwned;

pub mod account;
pub mod achievements;
pub mod backstory;
pub mod characters;
pub mod commerce;
pub mod emblems;
pub mod floors;
pub mod guild;
pub mod pvp;
pub mod skills;
pub mod specializations;
pub mod stories;
pub mod token_info;
pub mod traits;

pub async fn cats(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/v2/cats", api_base_url))
        .bearer_auth(api_key)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn cat(api_base_url: &str, api_key: &str, cat_id: u64) -> crate::codec::Cat {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/cats?id={}", api_base_url, cat_id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn colors(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/colors", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn color(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Color {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/colors?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn currencies(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/currencies", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn currency(
    api_base_url: &str,
    api_key: &str,
    id: u64,
    lang: &str,
) -> crate::codec::Currency {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!(
                "{}/v2/currencies?id={}&lang={}",
                api_base_url, id, lang
            ))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn dungeons(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/dungeons", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn dungeon(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Dungeon {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/dungeons?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn finishers(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/finishers", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn finisher(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Finisher {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/finishers?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn gliders(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/gliders", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn glider(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Glider {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/gliders?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn items(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/items", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn item(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Item {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/items?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn item_stats(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/itemstats", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn item_stat(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::ItemStats {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/itemstats?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn legends(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/legends", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn legend(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Item {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/legends?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn mail_carriers(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/mailcarriers", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn mail_carrier(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::MailCarrier {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/mailcarriers?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn masteries(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/masteries", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}
pub async fn mastery(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Mastery {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/masteries?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn material(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Material {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/materials?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn materials(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/materials", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn mini(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Mini {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/materials?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn minis(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/minis", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn node(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Node {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/nodes?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn nodes(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/nodes", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn outfit(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Outfit {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/outfits?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn outfits(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/outfits", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn pet(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Pet {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/pets?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn pets(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/pets", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn profession(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Profession {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/professions?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn professions(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/professions", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn race(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Race {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/races?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn races(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/races", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn raid(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Raid {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/raids?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn raids(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/raids", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn recipe(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Recipe {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/recipes?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn recipes(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/recipes", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn skin(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Skin {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/skins?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn skins(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/skins", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn title(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Title {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/titles?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn titles(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/titles", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn world(api_base_url: &str, api_key: &str, id: u64) -> crate::codec::World {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/worlds?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn worlds(api_base_url: &str, api_key: &str) -> Vec<u64> {
    let client = reqwest::Client::new();
    get_json(
        client
            .get(format!("{}/v2/worlds", api_base_url))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn get_json<T>(client: reqwest::RequestBuilder) -> T
where
    T: DeserializeOwned,
{
    let response = client.send().await.unwrap();
    let body = response.text().await.unwrap();
    match serde_json::from_str(&body) {
        Ok(ret) => ret,
        Err(e) => {
            eprintln!("{}", body);
            panic!("Invalid json: {:?}", e)
        }
    }
}
