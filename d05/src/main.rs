use aoc_runner::generate_main;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::{io::BufRead, ops::RangeInclusive};

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let mut lines = reader.lines();

    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').ok_or(eyre!("Invalid range format"))?;
        let start = start.parse().map_err(|_| eyre!("Invalid start value"))?;
        let end = end.parse().map_err(|_| eyre!("Invalid end value"))?;
        let range = start..=end;
        ranges.push(range);
    }

    Ok(lines
        .map(|line| -> Result<u64> {
            let line = line?;
            let number = line.parse().map_err(|_| eyre!("Invalid number"))?;
            Ok(number)
        })
        .filter_ok(|number| ranges.iter().any(|range| range.contains(number)))
        .count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 3);
    }
}
