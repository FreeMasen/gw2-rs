use comfy_table::{Cell, Table};
use gw2::client::Gw2Client;
use std::collections::HashMap;
use structopt::StructOpt;

pub mod characters;

#[derive(Debug, StructOpt)]
#[structopt(name = "gw2", about = "query the gw2 api endpoints")]
pub struct Opts {
    #[structopt(long, short)]
    /// The api key to use for this request
    api_key: Option<String>,
    #[structopt(subcommand)]
    command: Cmd,
}

/// The sub command to execute
#[derive(Debug, StructOpt)]
pub enum Cmd {
    Wallet {
        #[structopt(short, long)]
        name: Option<String>,
    },
    Characters {
        #[structopt(subcommand)]
        cmd: Option<characters::SubCmds>,
    },
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    let api_key = if let Some(key) = opts.api_key {
        key
    } else {
        let config = dirs::config_dir().unwrap();
        match std::fs::read_to_string(config.join("gw2").join("api-key")) {
            Ok(key) => key.trim().to_string(),
            Err(_e) => {
                eprintln!("Error, either the --api-key flag must be provided or the file ~/.config/gw2/api-key must exist");
                std::process::exit(1);
            }
        }
    };
    let client = Gw2Client::default().api_key(api_key);
    match opts.command {
        Cmd::Wallet { name } => wallet(&client, name.as_ref().map(|s| s.as_str())).await,
        Cmd::Characters { cmd } => characters::handle_subcommand(&client, cmd).await,
    }
}

async fn wallet(client: &Gw2Client, name: Option<&str>) {
    let owned = client.get_account_wallet().await;
    let first_part = client.get_all_currencies().await.unwrap();
    let mut map = HashMap::new();
    for currency in first_part.into_iter() {
        if let Some(name) = name {
            if currency
                .name
                .to_lowercase()
                .starts_with(&name.to_lowercase())
            {
                map.insert(currency.id, (currency.name, 0u64));
            }
        } else {
            map.insert(currency.id, (currency.name, 0u64));
        }
    }
    let mut table = create_table();
    table.set_header(vec![Cell::new("Name"), Cell::new("Quantity")]);
    for currency in owned {
        map.entry(currency.id).and_modify(|t| {
            t.1 = currency.value;
        });
    }
    // let mut table_values = Vec::with_capacity(map.len());
    let mut table_values: Vec<_> = map.values().collect();
    table_values.sort_by(|l, r| {
        if l.0 == "Coin" {
            std::cmp::Ordering::Less
        } else if r.0 == "Coin" {
            std::cmp::Ordering::Greater
        } else {
            r.1.cmp(&l.1)
        }
    });
    for (name, quantity) in table_values {
        let quantity = if name == "Coin" {
            format_coin(*quantity)
        } else {
            comma_seperated(*quantity)
        };
        table.add_row(vec![
            Cell::new(&name),
            Cell::new(&quantity).set_alignment(comfy_table::CellAlignment::Right),
        ]);
    }
    println!("{}", table)
}

fn comma_seperated(value: u64) -> String {
    if value > 999 {
        value
            .to_string()
            .chars()
            .rev()
            .enumerate()
            .fold(String::new(), |mut acc, (idx, ch)| {
                if idx > 0 && idx % 3 == 0 {
                    acc.push(',');
                }
                acc.push(ch);
                acc
            })
            .chars()
            .rev()
            .collect()
    } else {
        value.to_string()
    }
}

fn format_coin(value: u64) -> String {
    let gold = (value as f64 / 10_000.0).floor();
    let silver = ((value % 10_000) as f64 / 100.0).floor();
    let copper = value % 100;
    format!("{gold}g {silver}s {copper}c")
}

fn create_table() -> Table {
    let mut table = Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table
}
