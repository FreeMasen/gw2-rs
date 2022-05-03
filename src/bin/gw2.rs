use gw2::endpoints;
const API_BASE_URL: &str = "https://api.guildwars2.com";

#[tokio::main]
async fn main() {
    let api_key = std::fs::read_to_string("api-key").unwrap();
    let contentents = dbg!(endpoints::floors::continents( API_BASE_URL, &api_key, ).await);
    let _contentent = dbg!(endpoints::floors::continent( API_BASE_URL, &api_key, contentents[0]).await);
    let floors =dbg!( endpoints::floors::floors(API_BASE_URL, &api_key, contentents[0]).await);
    let _floor =dbg!( endpoints::floors::floor(API_BASE_URL, &api_key, contentents[0], floors[0]).await);
    let regions = dbg!( endpoints::floors::regions(API_BASE_URL, &api_key, contentents[0], floors[0]).await);
    let _region = dbg!( endpoints::floors::region(API_BASE_URL, &api_key, contentents[0], floors[0], regions[0]).await);
    let maps = dbg!( endpoints::floors::maps(API_BASE_URL, &api_key, contentents[0], floors[0], regions[0]).await);
    let _map = dbg!( endpoints::floors::map(API_BASE_URL, &api_key, contentents[0], floors[0], regions[0], maps[0]).await);
    let tasks = dbg!( endpoints::floors::tasks(API_BASE_URL, &api_key, contentents[0], floors[0], regions[0], maps[0]).await);
    let _task = dbg!( endpoints::floors::task(API_BASE_URL, &api_key, contentents[0], floors[0], regions[0], maps[0], tasks[0]).await);
    let pois = dbg!( endpoints::floors::pois(API_BASE_URL, &api_key, contentents[0], floors[0], regions[0], maps[0]).await);
    let _poi = dbg!( endpoints::floors::poi(API_BASE_URL, &api_key, contentents[0], floors[0], regions[0], maps[0], pois[0]).await);
}
