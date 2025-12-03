use aoc_runner::generate_main;
use color_eyre::eyre::Result;
use d02::{has_n_repetitions, parse_reader, sum_invalid_numbers};
use std::io::BufRead;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let ranges = parse_reader(reader);
    let sum = sum_invalid_numbers(ranges, is_invalid);
    Ok(sum)
}

fn is_invalid(number: u64) -> bool {
    has_n_repetitions(number, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 1227775554);
    }
}
