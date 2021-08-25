mod datastructures;

use csv::Reader;
use datastructures::d2::*;
use std::process::exit;
use std::{env, fs::File, path::PathBuf, str::FromStr};

#[cfg(test)]
mod tests;

fn main() {
    println!("Destiny: Armour Scrap Advisor");

    let file_path = get_path_env();
    let file = File::open(&file_path)
        .map_err(|e| Error::Io(e))
        .and_then(
            |f| match file_path.extension().unwrap_or_default() == "csv" {
                true => Ok(f),
                false => Err(Error::Other("Couldn't find CSV file.")),
            },
        )
        .unwrap_or_else(|e| {
            println!("\t{}", e);
            println!(
                "\tUsage: Either pass in a .csv from DIM or put dim.csv in the calling directory."
            );
            exit(1);
        });

    let reader = Reader::from_reader(file);
    let vault = import_items(reader);

    println!("Records:\t\t{}", vault.len());

    let warlock: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Warlock).collect();
    let titan: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Titan).collect();
    let hunter: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Hunter).collect();

    println!("Warlock\t\t\t{}", warlock.len());
    println!("Titan\t\t\t{}", titan.len());
    println!("Hunter\t\t\t{}", hunter.len());

    print_full_gear_heirarchy(&vault, Class::Warlock);
    print_full_gear_heirarchy(&vault, Class::Hunter);
    print_full_gear_heirarchy(&vault, Class::Titan);
}

fn import_items(mut reader: Reader<File>) -> Vec<Record> {
    reader
        .records()
        .map(|x| {
            let record = x.expect("Invalid Record");

            let name = &record[0]; // Aeon Swift
            let id = &record[2]; // 27394873298749238792
            let for_kind = &record[5]; // Gauntlets
            let for_class = &record[7];
            let season = &record[17]; // 2
            let is_exotic: bool = &record[4] == "Exotic";

            let mob = &record[27];
            let res = &record[28];
            let rec = &record[29];
            let dis = &record[30];
            let int = &record[31];
            let str = &record[32];

            // create stat array
            let s = Stats {
                mobility: mob.parse::<i8>().unwrap_or_default(),
                resilience: res.parse::<i8>().unwrap_or_default(),
                recovery: rec.parse::<i8>().unwrap_or_default(),
                discipline: dis.parse::<i8>().unwrap_or_default(),
                intelligence: int.parse::<i8>().unwrap_or_default(),
                strength: str.parse::<i8>().unwrap_or_default(),
            };

            Record {
                name: name.to_string(),
                id: id.parse::<u64>().unwrap_or_default(),
                armor: Kind::from_str(for_kind).unwrap(),
                class: Class::from_str(for_class).unwrap(),
                exotic: is_exotic,
                stat_array: s,
            }
        })
        .collect()
}

fn print_full_gear_heirarchy(vault: &Vec<Record>, character_type: Class) {
    print_heirarchy_of_type(vault, &character_type, Kind::Helmet);
    print_heirarchy_of_type(vault, &character_type, Kind::Arms);
    print_heirarchy_of_type(vault, &character_type, Kind::Chest);
    print_heirarchy_of_type(vault, &character_type, Kind::Legs);
    print_heirarchy_of_type(vault, &character_type, Kind::Bond);
}

fn print_heirarchy_of_type(vault: &Vec<Record>, character_type: &Class, gear_slot: Kind) {
    let vault_filtered: Vec<&Record> = vault
        .iter()
        .filter(|r| r.class == *character_type && r.armor == gear_slot)
        .collect();

    for item in vault_filtered.iter() {
        let mut worse: Vec<&Record> = Vec::new();
        for other in vault_filtered.iter() {
            if item.stat_array.collective_ge(&other.stat_array)
                && item.stat_array != other.stat_array
                && !other.exotic
                && !item.exotic
            {
                worse.push(other);
            }
        }
        if !worse.is_empty() {
            print!("{} is objectively better than ", item);
            for other in worse {
                print!("{}, ", other);
            }
            println!();
        }
    }
}

fn get_path_env() -> PathBuf {
    match env::args_os().nth(1) {
        None => PathBuf::from("./dim.csv"),
        Some(file_path) => file_path.into(),
    }
}
