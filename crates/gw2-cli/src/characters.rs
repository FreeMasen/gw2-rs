use std::collections::HashMap;

use comfy_table::Cell;
use gw2::{
    client::Gw2Client,
    codec::{InfixUpgrade, Item, Upgrade, _ItemDetail},
};
use structopt::StructOpt;

use crate::create_table;

#[derive(Debug, StructOpt)]
pub enum SubCmds {
    /// Get the equipment for a single character by name
    Equipment {
        /// The name of the character
        #[structopt(short, long)]
        name: String,
    },
}

pub async fn handle_subcommand(client: &Gw2Client, cmd: Option<SubCmds>) {
    let cmd = match cmd {
        None => return characters(client).await,
        Some(cmd) => cmd,
    };
    match cmd {
        SubCmds::Equipment { name } => equipment(client, &name).await,
    }
}

async fn characters(client: &Gw2Client) {
    let characters = client.get_all_characters().await.unwrap();
    let mut table = create_table();
    table.set_header(vec![
        Cell::new("Name"),
        Cell::new("Profession"),
        Cell::new("Level"),
    ]);
    for character in characters {
        table.add_row(vec![
            Cell::new(&character.name),
            Cell::new(&character.profession).fg(profession_color(&character.profession)),
            Cell::new(&character.level.to_string()),
        ]);
    }
    println!("{}", table);
}

fn profession_color(prof: &str) -> comfy_table::Color {
    use comfy_table::Color::*;
    match prof {
        "Mesmer" => Magenta,
        "Guardian" => Yellow,
        "Revenant" => Red,
        "Thief" => DarkGrey,
        "Elementalist" => Blue,
        "Ranger" => Cyan,
        "Necromancer" => Green,
        "Warrior" => DarkCyan,
        _ => White,
    }
}

#[derive(Debug, Clone, Default)]
pub struct CharacterEquipment {
    head: Option<Item>,
    shoulders: Option<Item>,
    chest: Option<Item>,
    hands: Option<Item>,
    legs: Option<Item>,
    feet: Option<Item>,
    left_hand_a: Option<Item>,
    right_hand_a: Option<Item>,
    left_hand_b: Option<Item>,
    right_hand_b: Option<Item>,
    back: Option<Item>,
    neck: Option<Item>,
    ear1: Option<Item>,
    ear2: Option<Item>,
    finger1: Option<Item>,
    finger2: Option<Item>,
    head_aq: Option<Item>,
    _left_hand_aq_a: Option<Item>,
    right_hand_aq_a: Option<Item>,
    _left_hand_aq_b: Option<Item>,
    right_hand_aq_b: Option<Item>,
}

async fn equipment(client: &Gw2Client, name: &str) {
    let character = client.get_character(name).await.unwrap();
    // let mut equipment = BTreeSet::new();
    let mut equipment = CharacterEquipment::default();
    for raw in character
        .equipment
        .iter()
        .filter(|e| e.slot != "Sickle" && e.slot != "Axe" && e.slot != "Pick")
    {
        match client.item(raw.id).await {
            Ok(item) => {
                match raw.slot.as_str() {
                    "Helm" => equipment.head = Some(item),
                    "HelmAquatic" => equipment.head_aq = Some(item),
                    "Backpack" => equipment.back = Some(item),
                    "Coat" => equipment.chest = Some(item),
                    "Boots" => equipment.feet = Some(item),
                    "Gloves" => equipment.hands = Some(item),
                    "Leggings" => equipment.legs = Some(item),
                    "Shoulders" => equipment.shoulders = Some(item),
                    "Accessory1" => equipment.ear1 = Some(item),
                    "Accessory2" => equipment.ear2 = Some(item),
                    "Ring1" => equipment.finger1 = Some(item),
                    "Ring2" => equipment.finger2 = Some(item),
                    "Amulet" => equipment.neck = Some(item),
                    "WeaponAquaticA" => equipment.right_hand_aq_a = Some(item),
                    "WeaponAquaticB" => equipment.right_hand_aq_b = Some(item),
                    "WeaponA1" => equipment.right_hand_a = Some(item),
                    "WeaponB1" => equipment.left_hand_a = Some(item),
                    "WeaponA2" => equipment.right_hand_b = Some(item),
                    "WeaponB2" => equipment.left_hand_b = Some(item),
                    _ => {
                        continue;
                    }
                }
                // println!("{:#?}", item);
            }
            Err(e) => {
                eprintln!("Error feting item {}: {}", raw.id, e);
                continue;
            }
        };
    }
    print_item_table(equipment.head.as_ref());
    print_item_table(equipment.shoulders.as_ref());
    print_item_table(equipment.chest.as_ref());
    print_item_table(equipment.hands.as_ref());
    print_item_table(equipment.legs.as_ref());
    print_item_table(equipment.feet.as_ref());
    print_item_table(equipment.right_hand_a.as_ref());
    print_item_table(equipment.right_hand_b.as_ref());
    print_item_table(equipment.left_hand_a.as_ref());
    print_item_table(equipment.left_hand_b.as_ref());
    print_item_table(equipment.back.as_ref());
    print_item_table(equipment.neck.as_ref());
    print_item_table(equipment.ear1.as_ref());
    print_item_table(equipment.ear2.as_ref());
    print_item_table(equipment.finger1.as_ref());
    print_item_table(equipment.finger2.as_ref());
    print_item_table(equipment.right_hand_aq_a.as_ref());
    print_item_table(equipment.right_hand_aq_b.as_ref());
    print_item_table(equipment._left_hand_aq_a.as_ref());
    print_item_table(equipment._left_hand_aq_b.as_ref());
    let mut attrs = HashMap::new();
    collect_attrs(equipment.head.as_ref(), &mut attrs);
    collect_attrs(equipment.shoulders.as_ref(), &mut attrs);
    collect_attrs(equipment.chest.as_ref(), &mut attrs);
    collect_attrs(equipment.hands.as_ref(), &mut attrs);
    collect_attrs(equipment.legs.as_ref(), &mut attrs);
    collect_attrs(equipment.feet.as_ref(), &mut attrs);
    collect_attrs(equipment.right_hand_a.as_ref(), &mut attrs);
    collect_attrs(equipment.right_hand_b.as_ref(), &mut attrs);
    collect_attrs(equipment.left_hand_a.as_ref(), &mut attrs);
    collect_attrs(equipment.left_hand_b.as_ref(), &mut attrs);
    collect_attrs(equipment.back.as_ref(), &mut attrs);
    collect_attrs(equipment.neck.as_ref(), &mut attrs);
    collect_attrs(equipment.ear1.as_ref(), &mut attrs);
    collect_attrs(equipment.ear2.as_ref(), &mut attrs);
    collect_attrs(equipment.finger1.as_ref(), &mut attrs);
    collect_attrs(equipment.finger2.as_ref(), &mut attrs);
    collect_attrs(equipment.right_hand_aq_a.as_ref(), &mut attrs);
    collect_attrs(equipment.right_hand_aq_b.as_ref(), &mut attrs);
    collect_attrs(equipment._left_hand_aq_a.as_ref(), &mut attrs);
    collect_attrs(equipment._left_hand_aq_b.as_ref(), &mut attrs);
    let mut table = crate::create_table();
    table.set_header(vec![Cell::new("Totals")]);
    for (k, v) in attrs {
        table.add_row(vec![Cell::new(&k), Cell::new(&v.to_string())]);
    }
    println!("{}", table);
}

fn collect_attrs(item: Option<&Item>, map: &mut HashMap<String, u64>) {
    let item = match item {
        Some(item) => item,
        None => return,
    };
    fn _collect(upgrade: Option<&InfixUpgrade>, map: &mut HashMap<String, u64>) {
        if let Some(upgrade) = upgrade {
            for attr in &upgrade.attributes {
                map.entry(attr.attribute.to_string())
                    .and_modify(|existing| {
                        *existing += attr.modifier;
                    })
                    .or_insert(attr.modifier);
            }
        }
    }
    eprintln!("{:?}", item);
    match &item.details {
        _ItemDetail::Armor { kind: _, details } => {
            map.entry("Defense".to_string())
                .and_modify(|existing| *existing += details.defense)
                .or_insert(details.defense);
            _collect(details.infix_upgrade.as_ref(), map);
        }
        _ItemDetail::Weapon {
            defense,
            infix_upgrade,
            max_power,
            ..
        } => {
            map.entry("Defense".to_string())
                .and_modify(|existing| *existing += *defense)
                .or_insert(*defense);
            map.entry("Power".to_string())
                .and_modify(|existing| *existing += *max_power)
                .or_insert(*max_power);
            _collect(infix_upgrade.as_ref(), map);
        }
        _ItemDetail::Trinket { infix_upgrade, .. } => {
            _collect(infix_upgrade.as_ref(), map);
        }
        _ => {}
    }
}

fn print_item_table(item: Option<&Item>) {
    let item = match item {
        Some(item) => item,
        None => return,
    };
    let mut table = crate::create_table();
    let color = color_from_rarity(&item.rarity);
    table.set_header(vec![Cell::new(&item.name).fg(color)]);
    match &item.details {
        _ItemDetail::Armor { kind: _, details } => {
            table.add_row(vec![
                Cell::new("defense"),
                Cell::new(&details.defense.to_string()),
            ]);
            if let Some(upgrade) = details.infix_upgrade.as_ref() {
                for attr in &upgrade.attributes {
                    table.add_row(vec![
                        Cell::new(&attr.attribute),
                        Cell::new(attr.modifier.to_string()),
                    ]);
                }
            }
        }
        _ItemDetail::Weapon {
            defense,
            infix_upgrade,
            ..
        } => {
            table.add_row(vec![Cell::new("defense"), Cell::new(&defense.to_string())]);
            if let Some(upgrade) = infix_upgrade.as_ref() {
                for attr in &upgrade.attributes {
                    table.add_row(vec![
                        Cell::new(&attr.attribute),
                        Cell::new(attr.modifier.to_string()),
                    ]);
                }
            }
        }
        _ItemDetail::Trinket { infix_upgrade, .. } => {
            if let Some(upgrade) = infix_upgrade.as_ref() {
                for attr in &upgrade.attributes {
                    table.add_row(vec![
                        Cell::new(&attr.attribute),
                        Cell::new(attr.modifier.to_string()),
                    ]);
                }
            }
        }
        _ => {}
    }
    println!("{}", table);
}

fn color_from_rarity(rarity: &str) -> comfy_table::Color {
    match rarity {
        "Legendary" => comfy_table::Color::Rgb {
            r: 0xa0,
            g: 0x20,
            b: 0xf0,
        },
        "Ascended" => comfy_table::Color::Magenta,
        "Exotic" => comfy_table::Color::Rgb {
            r: 0xFF,
            g: 0xA5,
            b: 0x00,
        },
        "Rare" => comfy_table::Color::Yellow,
        "Masterwork" => comfy_table::Color::Green,
        "Fine" => comfy_table::Color::Cyan,
        "Junk" => comfy_table::Color::Rgb {
            r: 0xD3,
            g: 0xD3,
            b: 0xD3,
        },
        _ => comfy_table::Color::White,
    }
}
