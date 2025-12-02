use color_eyre::eyre::Result;
use d02::{has_n_repetitions, parse_reader, sum_invalid_numbers};
use std::io::stdin;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let ranges = parse_reader(stdin().lock());
    let sum = sum_invalid_numbers(ranges, is_invalid);

    println!("{}", sum);

    Ok(())
}

fn is_invalid(number: u64) -> bool {
    for size in 2..=(number.ilog10() + 1) as u64 {
        if has_n_repetitions(number, size) {
            return true;
        }
    }
    false
}
