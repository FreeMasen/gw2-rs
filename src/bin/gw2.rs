use gw2::endpoints;
const API_BASE_URL: &str = "https://api.guildwars2.com";

#[tokio::main]
async fn main() {
    let api_key = std::fs::read_to_string("api-key").unwrap();
    let ids = endpoints::traits::traits(API_BASE_URL, &api_key).await;
    println!("Fetching {} objects", ids.len());
    for id in ids {
        println!("{}, {:#?}", id, endpoints::traits::r#trait(API_BASE_URL, &api_key, id).await);
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }
}
