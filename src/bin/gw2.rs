use my_gw2::endpoints;
const API_BASE_URL: &str = "https://api.guildwars2.com";

#[tokio::main]
async fn main() {
    let api_key = std::fs::read_to_string("api-key").unwrap();
    dbg!(endpoints::account::materials(API_BASE_URL, &api_key).await);
}
