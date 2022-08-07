use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use gw2::client::Gw2Client;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    /// Your personal arena-net api key
    api_key: String,
    /// The number of seconds to wait between to polls
    #[structopt(default_value = "30")]
    tick: u64,
    /// The port to expose a scrape endpoint at
    #[structopt(long, short)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    env_logger::try_init().ok();
    let opts = Opts::from_args();
    let mut prom = metrics_exporter_prometheus::PrometheusBuilder::new();
    if let Some(port) = opts.port {
        prom = prom.with_http_listener(([0, 0, 0, 0], port))
    }
    prom.install().unwrap();
    metrics::register_gauge!("account.wallet");
    metrics::register_gauge!("account.materials");
    metrics::register_gauge!("account.luck");
    metrics::describe_gauge!("account.wallet", "The contents of an account wallet");
    metrics::describe_gauge!("account.luck", "The contents of an account luck");
    metrics::describe_gauge!(
        "account.materials",
        "The contents of an account material storage"
    );
    let mut currencies = HashMap::new();
    let client = Gw2Client::default().api_key(&opts.api_key);
    let mut mats = client
        .get_all_material_defs()
        .await
        .unwrap()
        .into_iter()
        .map(|mat| (mat.id, mat.name))
        .collect();
    let mut last_refresh = UNIX_EPOCH;
    loop {
        if SystemTime::now().duration_since(last_refresh).unwrap()
            >= Duration::from_secs(24 * 60 * 60)
        {
            update_currencies(&client, &mut currencies).await;
            last_refresh = SystemTime::now();
        }
        wallet(&client, &currencies).await.unwrap();
        luck(&client).await;
        materials(&client, &mut mats).await;
        tokio::time::sleep(std::time::Duration::from_secs(opts.tick)).await;
    }
}

async fn wallet(client: &Gw2Client, currencies: &HashMap<u64, String>) -> Result<(), gw2::Error> {
    let wallets = client.get_account_wallet().await?;
    for wallet in wallets {
        let name = if let Some(name) = currencies.get(&wallet.id) {
            name.to_string()
        } else {
            continue;
        };
        metrics::gauge!("account.wallet", wallet.value.min(4_295_450) as f64,
            "currency" => name
        )
    }
    Ok(())
}

async fn luck(client: &Gw2Client) {
    let luck = client.get_luck().await.unwrap();
    metrics::gauge!("account.luck", luck.value as f64);
}

async fn materials(client: &Gw2Client, known_mats: &mut HashMap<u64, String>) {
    let mats = client.get_acct_mats().await.unwrap();
    for mat in mats {
        let name = if let Some(name) = known_mats.get(&mat.id) {
            name.to_string()
        } else {
            let item = client.item(mat.id).await.unwrap();
            let ret = item.name.clone();
            known_mats.entry(mat.id).or_insert(item.name);
            ret
        };
        metrics::gauge!("account.materials", mat.count as f64, "name" => name)
    }
}

async fn update_currencies(client: &Gw2Client, out: &mut HashMap<u64, String>) {
    let currencies = client.get_all_currencies().await.unwrap();
    for currency in currencies {
        out.entry(currency.id)
            .and_modify({
                let name = currency.name.clone();
                |value| *value = name
            })
            .or_insert(currency.name);
    }
}
