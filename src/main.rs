use std::{
    io::{self, BufRead, BufReader},
    str::FromStr,
    sync::LazyLock,
};

use regex::Regex;

#[derive(Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

// Parse `"L10"` into `Left(10)`
// Parse `"R10"` into `Right(10)`
impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(L|R)(\d+)$").unwrap());
        let caps = RE.captures(s).ok_or("Invalid rotation format")?;
        let direction = caps.get(1).unwrap().as_str();
        let angle = caps
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| "Invalid angle")?;

        match direction {
            "L" => Ok(Rotation::Left(angle)),
            "R" => Ok(Rotation::Right(angle)),
            _ => Err("Invalid direction".to_string()),
        }
    }
}

fn main() {
    let buffer = BufReader::new(io::stdin());
    let rotations = buffer
        .lines()
        .map_while(Result::ok)
        .map(|line| Rotation::from_str(line.as_str()).unwrap());

    let mut position = 50;
    let mut count = 0;
    for rotation in rotations {
        println!("position = {:?}; rotation = {:?}", position, rotation);

        position = match rotation {
            Rotation::Left(angle) => (position + 100 - (angle % 100)) % 100,
            Rotation::Right(angle) => (position + angle) % 100,
        };

        if position == 0 {
            count += 1;
        }
    }

    println!("Final position: {}", position);
    println!("Count: {}", count);
}
