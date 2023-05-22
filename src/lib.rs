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

pub fn get_input(y: u16, d: u8, path: &str) -> Result<(), Box<dyn Error>> {
    let y = Year::new(y);
    let d = Day::new(d);

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
