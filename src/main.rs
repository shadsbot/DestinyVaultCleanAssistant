use csv;
use std::fs::File;
use std::ffi::OsString;
use std::error::Error;
use std::env;
use std::str::FromStr;

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
struct Stats {
    mobility: i8,
    resilience: i8,
    recovery: i8,
    discipline: i8,
    intelligence: i8,
    strength: i8,
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
        // println!("{} ({}) - {} from Season {}", name, id, Kind, season);
        //println!("{:?}", r);
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

    Ok(())
}

fn get_first_argument() -> OsString {
    match env::args_os().nth(1) {
        None => OsString::from("./dim.csv"),
        Some(filePath) => filePath,
    }
}