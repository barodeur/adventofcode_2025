use aoc_runner::generate_main;
use d03::battery_bank::BatteryBank;
use eyre::Result;
use itertools::Itertools;
use std::io::BufRead;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let banks = BatteryBank::iter_from_reader(reader);

    banks.map_ok(|b| b.max_joltage::<2>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 357);
    }
}
