use aoc_runner::generate_main;
use eyre::Result;
use itertools::Itertools;
use std::io::BufRead;

#[derive(Debug)]
struct Battery(u8);

#[derive(Debug)]
struct BatteryBank(Vec<Battery>);

impl BatteryBank {
    fn max_joltage(&self) -> u8 {
        let mut a = 0;
        let mut b = 0;
        for battery in self.0.iter() {
            if b > a {
                a = b;
                b = battery.0;
            } else if battery.0 > b {
                b = battery.0;
            }
        }
        a * 10 + b
    }
}

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let banks = reader.lines().map(|line| -> Result<BatteryBank> {
        let line = line?;
        Ok(BatteryBank(
            line.bytes().map(|b| Battery(b - b'0')).collect(),
        ))
    });

    banks.map_ok(|b| b.max_joltage() as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 357);
    }
}
