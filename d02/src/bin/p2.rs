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
    for size in 2..=(number.ilog10() + 1) as u64 {
        if has_n_repetitions(number, size) {
            return true;
        }
    }
    false
}
