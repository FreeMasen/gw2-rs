use gw2::client::Gw2Client;
use rusqlite::{params, Connection};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    /// The api key to use for requests
    api_key: String,
    /// How often to request data from the server in seconds
    #[structopt(default_value = "30")]
    tick: u64,
    /// If the database should be in memory
    #[structopt(long, short)]
    mem: bool,
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    let client = Gw2Client::builder().api_key(opts.api_key).build();
    let conn = if opts.mem {
        Connection::open_in_memory()
    } else {
        let home = dirs::home_dir().unwrap();
        let file_path = home.join(".wallet-watcher");
        Connection::open(file_path)
    }
    .unwrap();
    ensure_tables(&conn);
    update_currency(&client, &conn).await;
    let mut insert_stmt = conn
        .prepare(
            "INSERT INTO wallet
                (timestamp, id, value)
                VALUES (?1, ?2, ?3)",
        )
        .unwrap();

    loop {
        let when = chrono::Utc::now();
        let wallet = client.get_account_wallet().await;
        for entry in wallet {
            insert_stmt
                .execute(params![&when, &entry.id, &entry.value])
                .unwrap();
        }
        tokio::time::sleep(std::time::Duration::from_secs(opts.tick)).await;
    }
}

fn ensure_tables(conn: &Connection) {
    let mut stmt = conn
        .prepare(
            "SELECT name, sql FROM sqlite_master
                WHERE type='table'
                ORDER BY name;",
        )
        .unwrap();
    let table_names = stmt
        .query_map([], |row| row.get::<&str, String>("name"))
        .unwrap();
    let mut seen_wallet = false;
    let mut seen_currency = false;
    for table_name in table_names {
        match table_name.unwrap().as_str() {
            "wallet" => seen_wallet = true,
            "currency" => seen_currency = true,
            name => eprintln!("Unknown table: {}", name),
        }
    }
    if !seen_wallet {
        conn.execute(
            "CREATE TABLE wallet (
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            id INTEGER NOT NULL,
            value INTEGER NOT NULL
        )",
            [],
        )
        .unwrap();
    }
    if !seen_currency {
        conn.execute(
            "CREATE TABLE currency (
            id INTEGER NOT NULL,
            name TEXT NOT NULL,
            desc TEXT,
            icon TEXT,
            UNIQUE(id, name)
            )",
            [],
        )
        .unwrap();
    }
}

async fn update_currency(client: &Gw2Client, conn: &Connection) {
    let currency = client.get_all_currencies().await.unwrap();
    let mut stmt = conn
        .prepare("INSERT OR IGNORE INTO currency (id, name, desc, icon) VALUES (?1, ?2, ?3, ?4)")
        .unwrap();
    for currency in currency {
        stmt.execute(params![
            &currency.id,
            &currency.name,
            &currency.description,
            &currency.icon
        ])
        .unwrap();
    }
}
