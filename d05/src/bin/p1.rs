use aoc_runner::generate_main;
use d05::parse;
use eyre::Result;
use itertools::Itertools;
use std::io::BufRead;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let (ranges, numbers) = parse(reader)?;

    Ok(numbers
        .filter_ok(|number| ranges.iter().any(|range| range.contains(*number)))
        .count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 3);
    }
}
