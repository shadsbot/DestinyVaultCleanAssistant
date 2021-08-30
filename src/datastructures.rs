use std::{fmt, str::FromStr};

use thiserror::Error;

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
pub enum Class {
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
pub enum Kind {
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
pub struct Stats {
    pub mobility: i8,
    pub resilience: i8,
    pub recovery: i8,
    pub discipline: i8,
    pub intelligence: i8,
    pub strength: i8,
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
    pub fn collective_ge(&self, other: &Self) -> bool {
        return self.mobility >= other.mobility
            && self.resilience >= other.resilience
            && self.recovery >= other.recovery
            && self.discipline >= other.discipline
            && self.intelligence >= other.intelligence
            && self.strength >= other.strength;
    }
}

#[derive(Debug)]
pub(crate) struct Record {
    pub name: String,
    pub id: u64,
    pub armor: Kind,
    pub class: Class,
    pub exotic: bool,
    pub stat_array: Stats,
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
