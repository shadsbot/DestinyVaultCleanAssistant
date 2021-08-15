use csv;
use std::collections::LinkedList;
use std::fs::File;
use std::ffi::OsString;
use std::error::Error;
use std::{env, vec};
use std::str::FromStr;
use std::fmt;

fn main() {
    println!("Destiny: Armour Scrap Advisor");
    println!("\tUsage: Either pass in a .csv from DIM or put dim.csv in the calling directory.");
    let err = run();
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Class {
    Warlock,
    Titan,
    Hunter,
}

impl FromStr for Class {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Warlock" => Ok(Class::Warlock),
            "Titan" => Ok(Class::Titan),
            "Hunter" => Ok(Class::Hunter),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Kind {
    Helmet,
    Arms,
    Chest,
    Legs,
    Bond
}

impl FromStr for Kind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Helmet" => Ok(Kind::Helmet),
            "Gauntlets" => Ok(Kind::Arms),
            "Chest Armor" => Ok(Kind::Chest),
            "Leg Armor" => Ok(Kind::Legs),
            "Hunter Cloak" => Ok(Kind::Bond),
            "Warlock Bond" => Ok(Kind::Bond),
            "Titan Mark" => Ok(Kind::Bond),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
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
        write!(f, "({:<2} {:<2} {:<2} {:<2} {:<2} {:<2})", self.mobility, self.resilience, self.recovery, self.discipline, self.intelligence, self.strength)
    }
}

#[derive(Debug)]
struct Record {
    name: String,
    id: u64,
    armor: Kind,
    class: Class,
    stat_array: Stats,
}

fn import_items(mut reader: csv::Reader<File>) -> Result<Vec<Record>, Box<Error>> {

    let mut vault: Vec<Record> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let name = &record[0]; // Aeon Swift
        let id = &record[2]; // 27394873298749238792
        let for_kind = &record[5]; // Gauntlets
        let for_class = &record[7];
        let season = &record[17]; // 2
        
        let mob = &record[27];
        let res = &record[28];
        let rec = &record[29];
        let dis = &record[30];
        let int = &record[31];
        let str = &record[32];

        // create stat array
        let s  = Stats{
            mobility: mob.parse::<i8>().unwrap_or_default(),
            resilience: res.parse::<i8>().unwrap_or_default(),
            recovery: rec.parse::<i8>().unwrap_or_default(),
            discipline: dis.parse::<i8>().unwrap_or_default(),
            intelligence: int.parse::<i8>().unwrap_or_default(),
            strength: str.parse::<i8>().unwrap_or_default()
        };

        let r = Record {
            name: name.to_string(),
            id: id.parse::<u64>().unwrap_or_default(),
            armor: Kind::from_str(for_kind).unwrap(),
            class: Class::from_str(for_class).unwrap(),
            stat_array: s,
        };
        vault.push(r);
    };
    Ok(vault)
}

fn run() -> Result<(), Box<Error>> {
    let filePath = get_first_argument();

    if !std::path::PathBuf::from(&filePath).exists() || std::path::PathBuf::from(&filePath).ends_with(".csv") {
        panic!("File not found or incorrect type.")
    };
    
    let file = File::open(filePath)?;
    let mut reader = csv::Reader::from_reader(file);

    let vault = import_items(reader).unwrap();

    println!("Records:\t\t{}", vault.len());

    let warlock: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Warlock).collect();
    let titan: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Titan).collect();
    let hunter: Vec<&Record> = vault.iter().filter(|r| r.class == Class::Hunter).collect();

    println!("Warlock\t\t\t{}", warlock.len());
    println!("Titan\t\t\t{}", titan.len());
    println!("Hunter\t\t\t{}", hunter.len());

    print_heirarchy_of_type(&vault, Class::Warlock, Kind::Helmet);
    print_heirarchy_of_type(&vault, Class::Warlock, Kind::Chest);
    print_heirarchy_of_type(&vault, Class::Warlock, Kind::Arms);
    print_heirarchy_of_type(&vault, Class::Warlock, Kind::Legs);
    print_heirarchy_of_type(&vault, Class::Warlock, Kind::Bond);

    Ok(())
}

fn print_heirarchy_of_type(vault: &Vec<Record>, character_type: Class, gear_slot: Kind) {
    let vault_filtered: Vec<&Record> = vault.iter().filter(|r| r.class == character_type && r.armor == gear_slot).collect();

    let mut objectively_better: Vec<LinkedList<&Record>> = Vec::new();
    for record_to_compare in vault_filtered.iter() {
        let mut ll: LinkedList<&Record> = LinkedList::new();
        ll.push_front(record_to_compare);
        for record_to_check_against in vault_filtered.iter() {     
            if record_to_compare.stat_array.mobility >= record_to_check_against.stat_array.mobility &&
            record_to_compare.stat_array.resilience >= record_to_check_against.stat_array.resilience &&
            record_to_compare.stat_array.recovery >= record_to_check_against.stat_array.recovery &&
            record_to_compare.stat_array.discipline >= record_to_check_against.stat_array.discipline &&
            record_to_compare.stat_array.intelligence >= record_to_check_against.stat_array.intelligence &&
            record_to_compare.stat_array.strength >= record_to_check_against.stat_array.strength {
                if !record_to_compare.stat_array.eq(&record_to_check_against.stat_array) {
                    ll.push_back(&record_to_check_against);
                } 
            }                
        }
        // ll will always have at least 1 item, if there's more then we've
        // found something that is lower in every stat
        if ll.len() > 1 {
            objectively_better.push(ll);
        }
    }

    for mut ll in objectively_better {
        let firstI = ll.front().unwrap();
        print!("{} {} is objectively better than ", firstI.name, firstI.stat_array);
        ll.pop_front();
        while ll.len() > 0 {
            let item = ll.front().unwrap();
            print!(" {} {},", item.name, item.stat_array);
            ll.pop_front();
        }
        println!();
    }
}

fn get_first_argument() -> OsString {
    match env::args_os().nth(1) {
        None => OsString::from("./dim.csv"),
        Some(filePath) => filePath,
    }
}