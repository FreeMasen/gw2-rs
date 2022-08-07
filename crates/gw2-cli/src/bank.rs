use std::collections::HashMap;

use comfy_table::Cell;
use gw2::{client::Gw2Client, codec::{Item}};
use crate::characters::color_from_rarity;



pub async fn handle_subcommand(client: Gw2Client, name: Option<String>) {
    let all = get_all(client).await;
    if let Some(name) = name {
        if let Some(one) = all.into_iter().find(|(i,_ct)| i.name.to_lowercase().starts_with(name.to_lowercase().as_str())) {
            print!("{}({})", one.0.name, one.1);
            if let Some(desc) = one.0.description {
                print!(": {}", desc)
            }
            println!("");
        } else {
            eprintln!("didn't find material: {:?}", name);
            std::process::exit(1)
        }
    } else {
        let mut table = crate::create_table();
        table.set_header(vec![
            Cell::new("Name"),
            Cell::new("Count"),
        ]);
        for (item, count) in all {
            let color = color_from_rarity(&item.rarity);
            table.add_row(vec![
                Cell::new(item.name).fg(color),
                Cell::new(count.to_string()),
            ]);
        }
        println!("{}", table);
    }
}

pub async fn get_all(client: Gw2Client) -> Vec<(Item, u64)> {
    let all: HashMap<u64, Item> = client.get_all_material_defs().await.unwrap().into_iter().map(|mat| {
        (mat.id, mat)
    }).collect();
    println!("all: {}", all.len());
    client.get_acct_mats().await.unwrap().into_iter().filter_map(|a| {
        let mat = all.get(&a.id)?.clone();
        Some((mat, a.count))
    }).collect()
}