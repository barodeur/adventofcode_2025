use eyre::{eyre, Result};
use std::io::BufRead;

use crate::range::Range;

pub mod range;

pub fn parse(reader: impl BufRead) -> Result<(Vec<Range>, impl Iterator<Item = Result<u64>>)> {
    let mut lines = reader.lines();

    let mut ranges: Vec<Range> = vec![];
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let range = line.parse()?;
        ranges.push(range);
    }

    let numbers =
        lines.flat_map(|line| line.map(|line| line.parse().map_err(|_| eyre!("Invalid number"))));

    Ok((ranges, numbers))
}
