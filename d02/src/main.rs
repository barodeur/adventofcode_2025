use std::{
    io::{stdin, BufRead},
    ops::RangeInclusive,
};

use color_eyre::eyre::{self, Result};
use tracing::debug;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let ranges = stdin()
        .lock()
        .split(b',')
        .map(|range_bytes| -> Result<RangeInclusive<u64>> {
            let range_bytes = range_bytes?;
            let range_string = std::str::from_utf8(&range_bytes)?.trim();
            let (start, end) = range_string
                .split_once('-')
                .ok_or_else(|| eyre::eyre!("Invalid range format"))?;
            let start = start.parse()?;
            let end = end.parse()?;
            Ok(start..=end)
        });

    let sum = ranges
        .flat_map(|range| -> Result<u64> {
            let range = range?;
            Ok(range
                .filter(|&number| {
                    let digits_count = (number.ilog10() + 1) / 2;
                    let ten_power = 10_u64.pow(digits_count);
                    let lower_part = number % ten_power;
                    let upper_part = number / ten_power;
                    debug!(?number, ?lower_part, ?upper_part);
                    lower_part == upper_part
                })
                .sum::<u64>())
        })
        .sum::<u64>();

    println!("{}", sum);

    Ok(())
}
