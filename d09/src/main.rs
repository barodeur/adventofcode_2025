use aoc_runner::generate_main;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::io::BufRead;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let corners: Vec<_> = reader
        .lines()
        .map(|line| -> Result<(usize, usize)> {
            let line = line?;
            let (x, y) = line.split_once(',').ok_or_else(|| eyre!("Invalid input"))?;
            let x = x.parse::<usize>()?;
            let y = y.parse::<usize>()?;
            Ok((x, y))
        })
        .try_collect()?;

    let max = corners
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|corners| corners_area((*corners.0, *corners.1)))
        .max()
        .ok_or_else(|| eyre!("No corners"))?;

    Ok(max as u64)
}

fn corners_area(corners: ((usize, usize), (usize, usize))) -> usize {
    let (x1, y1) = corners.0;
    let (x2, y2) = corners.1;
    (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 50);
    }
}
