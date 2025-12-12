use aoc_runner::generate_main;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::io::BufRead;
use tracing::debug;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let lines = reader.lines();
    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut operations: Option<Vec<char>> = None;
    for line in lines {
        let line = line?;
        debug!(?line);

        let parts = line.split_whitespace();
        let parsed_parts_as_number: Result<Vec<u64>> = parts
            .map(|p| p.parse::<u64>())
            .try_collect()
            .map_err(|_| eyre!("Failed to parse line"));

        if let Ok(ns) = parsed_parts_as_number {
            numbers.push(ns);
            continue;
        }

        let parts = line.split_whitespace();
        let parsed_parts_as_char: Result<Vec<char>> = parts
            .map(|p| p.parse::<char>())
            .try_collect()
            .map_err(|_| eyre!("Failed to parse line"));
        if let Ok(ns) = parsed_parts_as_char {
            operations = Some(ns);
            break;
        }
    }
    debug!(?numbers, ?operations);

    let operations = operations.ok_or_else(|| eyre!("No operations found"))?;

    operations
        .iter()
        .enumerate()
        .map(|(idx, op)| {
            let nums: Vec<_> = numbers.iter().map(|ns| ns[idx]).collect();
            match op {
                '+' => Ok(nums.iter().sum::<u64>()),
                '*' => Ok(nums.iter().product::<u64>()),
                _ => Err(eyre!("Unsupported operation: {}", op)),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 4277556);
    }
}
