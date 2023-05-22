//! Advent Input
//!
//! `advent_input` provides a function to easily retrieve Advent of Code input for a given date.
extern crate reqwest;
use std::{io::Read, path::Path, fs, env, error::Error};

struct Day {
    num: u8
}

impl Day {
    fn new(num: u8) -> Day {
        if num < 1 || num > 25 {
            panic!("Invalid day specified.");
        }
        Day { num }
    }
}

struct Year {
    num : u16
}

impl Year {
    fn new(num: u16) -> Year {
        if num < 2015 {
            panic!("Invalid year specified.");
        }
        Year { num }
    }
}

/// Gets input of specified year and day, writing to given path.
/// Reads from environment variable for authentication. Set ADVENT_COOKIE to your session value in
/// your environment variables to configure.
///
/// # Examples
///
/// ```
/// advent_input::get_input(2017, 16, "sixteen.txt").unwrap(); // Writes input for Day 16, 2017 to sixteen.txt
/// ```
pub fn get_input(year: u16, day: u8, path: &str) -> Result<(), Box<dyn Error>> {
    let y = Year::new(year);
    let d = Day::new(day);

    let path = Path::new(path);
    if path.exists() {
        return Ok(());
    }

    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", y.num, d.num);
    let cookie = env::var("ADVENT_COOKIE")?;
    let mut res = client.get(url)
        .header("Cookie", format!("session={}", cookie)).send()?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    fs::write(path, body)?;

    Ok(())
}
