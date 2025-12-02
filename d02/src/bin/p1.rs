use aoc_runner::resolver;
use color_eyre::eyre::Result;
use d02::{has_n_repetitions, parse_reader, sum_invalid_numbers};
use std::io::BufRead;

#[resolver]
fn main(reader: impl BufRead) -> Result<u64> {
    let ranges = parse_reader(reader);
    let sum = sum_invalid_numbers(ranges, is_invalid);
    Ok(sum)
}

fn is_invalid(number: u64) -> bool {
    has_n_repetitions(number, 2)
}
