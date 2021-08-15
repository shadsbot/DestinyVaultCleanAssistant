use csv::Reader;
use std::collections::LinkedList;
use std::process::exit;
use std::{cmp::Ordering, env, ffi::OsString, fmt, fs::File, path::PathBuf, str::FromStr};
use thiserror::Error;

fn main() {
    println!("Destiny: Armour Scrap Advisor");

    let file_path = get_path_env();
    let file = match File::open(&file_path)
        .map_err(|e| Error::Io(e))
        .and_then(
            |f| match file_path.extension().unwrap_or_default() == "csv" {
                true => Ok(f),
                false => Err(Error::Other("Couldn't find CSV file.")),
            },
        ) {
        Ok(file) => file,
        Err(e) => {
            println!("\t{}", e);
            println!(
                "\tUsage: Either pass in a .csv from DIM or put dim.csv in the calling directory."
            );
            exit(1);
        }
    };
    let reader = Reader::from_reader(file);
    let vault = import_items(reader);

    println!("Records:\t\t{}", vault.len());

    let warlock: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Warlock).collect();
    let titan: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Titan).collect();
    let hunter: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Hunter).collect();

    println!("Warlock\t\t\t{}", warlock.len());
    println!("Titan\t\t\t{}", titan.len());
    println!("Hunter\t\t\t{}", hunter.len());

    print_full_gear_heirarchy(&vault, &Class::Warlock);
    print_full_gear_heirarchy(&vault, &Class::Hunter);
    print_full_gear_heirarchy(&vault, &Class::Titan);
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid Couldn't Parse")]
    InvalidParse,

    #[error("CSV error {0}")]
    CSV(#[from] csv::Error),

    #[error("{0}")]
    Other(&'static str),
}

#[derive(Debug, PartialEq)]
enum Class {
    Warlock,
    Titan,
    Hunter,
}

impl FromStr for Class {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Warlock" => Ok(Class::Warlock),
            "Titan" => Ok(Class::Titan),
            "Hunter" => Ok(Class::Hunter),
            _ => Err(Error::InvalidParse),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Kind {
    Helmet,
    Arms,
    Chest,
    Legs,
    Bond,
}

impl FromStr for Kind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Helmet" => Ok(Kind::Helmet),
            "Gauntlets" => Ok(Kind::Arms),
            "Chest Armor" => Ok(Kind::Chest),
            "Leg Armor" => Ok(Kind::Legs),
            "Hunter Cloak" => Ok(Kind::Bond),
            "Warlock Bond" => Ok(Kind::Bond),
            "Titan Mark" => Ok(Kind::Bond),
            _ => Err(Error::InvalidParse),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Stats {
    mobility: i8,
    resilience: i8,
    recovery: i8,
    discipline: i8,
    intelligence: i8,
    strength: i8,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:<2} {:<2} {:<2} {:<2} {:<2} {:<2})",
            self.mobility,
            self.resilience,
            self.recovery,
            self.discipline,
            self.intelligence,
            self.strength
        )
    }
}

impl Stats {
    // ord seems to have a hard time with multiple keys to sort by
    // and trying to find help online just results in a bunch of
    // explanations about floating point imprecision? so we're going
    // to try making a rust-looking way of getting what we want
    fn collective_ge(&self, other: &Self) -> bool {
        return &self.mobility >= &other.mobility
            && &self.resilience >= &other.resilience
            && &self.recovery >= &other.recovery
            && &self.discipline >= &other.discipline
            && &self.intelligence >= &other.intelligence
            && &self.strength >= &other.strength;
    }
}

#[derive(Debug)]
struct Record {
    name: String,
    id: u64,
    armor: Kind,
    class: Class,
    exotic: bool,
    stat_array: Stats,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {}",
            match self.exotic {
                true => "*",
                false => "",
            },
            self.name,
            self.stat_array
        )
    }
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

fn print_full_gear_heirarchy(vault: &Vec<Record>, character_type: &Class) {
    print_heirarchy_of_type(vault, character_type, Kind::Helmet);
    print_heirarchy_of_type(vault, character_type, Kind::Arms);
    print_heirarchy_of_type(vault, character_type, Kind::Chest);
    print_heirarchy_of_type(vault, character_type, Kind::Legs);
    print_heirarchy_of_type(vault, character_type, Kind::Bond);
}

fn print_heirarchy_of_type(vault: &Vec<Record>, character_type: &Class, gear_slot: Kind) {
    let vault_filtered: Vec<&Record> = vault
        .iter()
        .filter(|r| r.class == *character_type && r.armor == gear_slot)
        .collect();

    let mut objectively_better: Vec<LinkedList<&Record>> = Vec::new();

    for gear in vault_filtered.iter() {
        let mut ll: LinkedList<&Record> = LinkedList::new();
        ll.push_front(gear);
        for gear_compare in vault_filtered.iter() {
            if gear.stat_array.collective_ge(&gear_compare.stat_array) {
                if gear.stat_array != gear_compare.stat_array {
                    ll.push_back(&gear_compare);
                }
            }
        }
        // ll will always have at least 1 item, if there's more then we've
        // found something that is lower in every stat
        if ll.len() > 1 {
            objectively_better.push(ll);
        }
    }
    // print or export the results
    for mut ll in objectively_better {
        let first_item = ll.front().unwrap();
        if first_item.exotic {
            continue;
        } // ignore exotics
        print!("{} is objectively better than ", first_item);
        ll.pop_front();
        while ll.len() > 0 {
            let item = ll.front().unwrap();
            print!(" {}{}", item, if ll.len() > 1 { "," } else { "" });
            ll.pop_front();
        }
        println!();
    }
}

fn get_path_env() -> PathBuf {
    match env::args_os().nth(1) {
        None => PathBuf::from("./dim.csv"),
        Some(file_path) => file_path.into(),
    }
}
