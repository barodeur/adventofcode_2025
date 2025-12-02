use color_eyre::{eyre, Result};
use regex::Regex;
use std::{io::BufRead, str::FromStr, sync::LazyLock};

#[derive(Debug)]
pub enum Rotation {
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

impl Rotation {
    pub fn angle(&self) -> isize {
        match self {
            Rotation::Right(a) => *a as isize,
            Rotation::Left(a) => -(*a as isize),
        }
    }

    pub fn iter_from(input: impl BufRead) -> impl Iterator<Item = Result<Rotation>> {
        input.lines().map(|line| {
            let line = line.map_err(|e| eyre::eyre!("Failed to read line: {}", e))?;
            let rotation = line
                .parse::<Rotation>()
                .map_err(|err| eyre::eyre!("Failed to parse rotation '{}': {}", line, err))?;
            Ok(rotation)
        })
    }
}
