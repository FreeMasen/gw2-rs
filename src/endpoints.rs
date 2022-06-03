use reqwest::{header::HeaderMap, Client};
use serde::de::DeserializeOwned;

use crate::{codec::{self, Item}, Error, Link, Links, PagedResult};

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
pub mod wvw;

pub async fn cats(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
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

pub async fn cat(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    cat_id: u64,
) -> crate::codec::Cat {
    get_json(
        client
            .get(format!("{}/v2/cats?id={}", api_base_url, cat_id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn colors(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/colors", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn color(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Color {
    get_json(
        client
            .get(format!("{}/v2/colors?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn currencies(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/currencies", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn get_currencies(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    page: usize,
    page_size: usize,
) -> Vec<crate::codec::Currency> {
    get_json(
        client
            .get(format!(
                "{}/v2/currencies?page={}&page-size={}",
                api_base_url, page, page_size
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn get_all_currencies(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Result<Vec<crate::codec::Currency>, crate::Error> {
    let mut ret = Vec::new();
    let url = format!("{}/v2/currencies", api_base_url);
    let initial = get_paged::<crate::codec::Currency>(&client, &url, api_key, 0, 50).await?;
    ret.extend(initial.result);
    for i in 1..initial.page.total_pages.unwrap_or(1) {
        let iter = get_paged::<crate::codec::Currency>(&client, &url, api_key, i, 50).await?;
        ret.extend(iter.result);
    }
    Ok(ret)
}

pub async fn currency(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
    lang: &str,
) -> crate::codec::Currency {
    get_json(
        client
            .get(format!(
                "{}/v2/currencies?id={}&lang={}",
                api_base_url, id, lang
            ))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn dungeons(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/dungeons", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn dungeon(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Dungeon {
    get_json(
        client
            .get(format!("{}/v2/dungeons?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn finishers(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/finishers", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn finisher(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Finisher {
    get_json(
        client
            .get(format!("{}/v2/finishers?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn gliders(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/gliders", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn glider(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Glider {
    get_json(
        client
            .get(format!("{}/v2/gliders?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn items(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/items", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn items_by_ids(client: &Client, api_base_url: &str, api_key: &str, ids: impl Iterator<Item = impl ToString>) -> Result<Vec<Item>, Error> {
    get_json(
        client
            .get(format!("{}/v2/items?ids={}", api_base_url, ids.map(|i| i.to_string()).collect::<Vec<_>>().join(",")))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn item(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> Result<crate::codec::Item, crate::Error> {
    get_json(
        client
            .get(format!("{}/v2/items?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
}

pub async fn item_stats(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/itemstats", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn item_stat(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::ItemStats {
    get_json(
        client
            .get(format!("{}/v2/itemstats?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn legends(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/legends", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn legend(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Item {
    get_json(
        client
            .get(format!("{}/v2/legends?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn mail_carriers(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/mailcarriers", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn mail_carrier(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::MailCarrier {
    get_json(
        client
            .get(format!("{}/v2/mailcarriers?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn masteries(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/masteries", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}
pub async fn mastery(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Mastery {
    get_json(
        client
            .get(format!("{}/v2/masteries?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn material(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Material {
    get_json(
        client
            .get(format!("{}/v2/materials?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn materials(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/materials", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn all_materials(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
) -> Result<Vec<codec::Material>, Error> {
    let mut ret = Vec::new();
    let url = format!("{}/v2/materials", api_base_url);
    let initial = get_paged::<crate::codec::Material>(&client, &url, api_key, 0, 50).await?;
    ret.extend(initial.result);
    for i in 1..initial.page.total_pages.unwrap_or(1) {
        let iter = get_paged::<crate::codec::Material>(&client, &url, api_key, i, 50).await?;
        ret.extend(iter.result);
    }
    Ok(ret)
}

pub async fn mini(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Mini {
    get_json(
        client
            .get(format!("{}/v2/materials?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn minis(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/minis", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn node(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Node {
    get_json(
        client
            .get(format!("{}/v2/nodes?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn nodes(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/nodes", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn outfit(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Outfit {
    get_json(
        client
            .get(format!("{}/v2/outfits?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn outfits(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/outfits", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn pet(client: &Client, api_base_url: &str, api_key: &str, id: u64) -> crate::codec::Pet {
    get_json(
        client
            .get(format!("{}/v2/pets?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn pets(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/pets", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn profession(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Profession {
    get_json(
        client
            .get(format!("{}/v2/professions?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn professions(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/professions", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn race(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Race {
    get_json(
        client
            .get(format!("{}/v2/races?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn races(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/races", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn raid(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Raid {
    get_json(
        client
            .get(format!("{}/v2/raids?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn raids(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/raids", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn recipe(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Recipe {
    get_json(
        client
            .get(format!("{}/v2/recipes?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn recipes(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/recipes", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn skin(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Skin {
    get_json(
        client
            .get(format!("{}/v2/skins?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn skins(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/skins", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn title(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::Title {
    get_json(
        client
            .get(format!("{}/v2/titles?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn titles(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/titles", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn world(
    client: &Client,
    api_base_url: &str,
    api_key: &str,
    id: u64,
) -> crate::codec::World {
    get_json(
        client
            .get(format!("{}/v2/worlds?id={}", api_base_url, id))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn worlds(client: &Client, api_base_url: &str, api_key: &str) -> Vec<u64> {
    get_json(
        client
            .get(format!("{}/v2/worlds", api_base_url))
            .bearer_auth(api_key),
    )
    .await
    .unwrap()
}

pub async fn get_paged<T>(
    client: &reqwest::Client,
    url: &str,
    api_key: &str,
    page: usize,
    page_size: usize,
) -> Result<PagedResult<T>, crate::Error>
where
    T: DeserializeOwned,
{
    let request = client
        .get(&format!("{}?page={}&page-size={}", url, page, page_size))
        .bearer_auth(api_key);
    let response = request.send().await?;
    let headers = response.headers();
    let mut page = super::PageInfo::default();
    page.links = extract_links(headers);
    page.page_size = extract_usize_header(headers, "x-page-size");
    page.total_pages = extract_usize_header(headers, "x-page-total");
    page.result_count = extract_usize_header(headers, "x-result-count");
    let result = response.json().await?;
    Ok(PagedResult { page, result })
}

fn extract_usize_header(headers: &HeaderMap, key: &str) -> Option<usize> {
    headers
        .get(key)
        .map(|value| String::from_utf8_lossy(value.as_bytes()).parse().ok())
        .flatten()
}

fn extract_links(headers: &HeaderMap) -> Links {
    let mut ret = Links::default();
    if let Some(link) = headers.get("link") {
        let parsed = link
            .as_bytes()
            .split(|c| *c == b',')
            .map(|bytes| String::from_utf8_lossy(bytes))
            .filter_map(|part| {
                let mut parts = part.split("; ");
                let link = parts.next()?.trim_start_matches('<').trim_end_matches('>');
                let page_idx = link.find("page=")?;
                let after_page = link.get(page_idx + 5..)?;
                let page = if let Some(end_idx) = after_page.find("&") {
                    &after_page[..end_idx]
                } else {
                    after_page
                };
                let page: usize = page.parse().ok()?;
                let page_size_idx = link.find("page-size=")?;
                let after_page = link.get(page_size_idx + 10..)?;
                let page_size = if let Some(end_idx) = after_page.find("&") {
                    &after_page[..end_idx]
                } else {
                    after_page
                };
                let page_size: usize = page_size.parse().ok()?;
                let rel = parts.next()?.trim_start_matches("rel=");
                Some((Link { page, page_size }, rel.to_string()))
            });
        for link in parsed {
            match link.1.as_str() {
                "next" => ret.next = Some(link.0),
                "self" => ret.this = Some(link.0),
                "first" => ret.first = Some(link.0),
                "last" => ret.last = Some(link.0),
                _ => {}
            }
        }
    }
    ret
}

#[track_caller]
pub async fn get_json<T>(client: reqwest::RequestBuilder) -> Result<T, crate::Error>
where
    T: DeserializeOwned,
{
    let response = client.send().await?;
    let url = response.url().clone();
    let status = response.status();
    match status.as_u16() {
        200 => {}
        429 => return Err(Error::RateLimit),
        507 => return Err(Error::Timeout),
        status => {
            log::warn!("Unknown status: {}", status);
        }
    }
    if status.as_u16() >= 400 && status.as_u16() < 500 {
        return Err(crate::Error::RateLimit);
    }
    let body = response.text().await?;
    match serde_json::from_str(&body) {
        Ok(ret) => Ok(ret),
        Err(e) => {
            eprintln!("failed request to {} ({})\n{}", url, status, body);
            panic!("Invalid json: {:?}", e)
        }
    }
}
