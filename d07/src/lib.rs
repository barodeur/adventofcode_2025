use eyre::{eyre, Result};
use itertools::Itertools;
use std::io::BufRead;

pub enum Item {
    Source,
    Splitter,
    Empty,
}

pub fn parse(reader: impl BufRead) -> impl Iterator<Item = Result<Vec<Item>>> {
    reader.lines().map(|line| {
        let line = line?;
        line.chars()
            .map(|c| match c {
                'S' => Ok(Item::Source),
                '^' => Ok(Item::Splitter),
                '.' => Ok(Item::Empty),
                _ => Err(eyre!("Invalid tile")),
            })
            .try_collect()
    })
}
