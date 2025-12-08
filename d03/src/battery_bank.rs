use crate::battery::Battery;
use eyre::Result;
use std::io::BufRead;
use tracing::debug;

#[derive(Debug)]
pub struct BatteryBank(Vec<Battery>);

impl BatteryBank {
    fn parse_line(line: &str) -> BatteryBank {
        BatteryBank(line.chars().map(Battery::parse_char).collect())
    }

    pub fn iter_from_reader(reader: impl BufRead) -> impl Iterator<Item = Result<BatteryBank>> {
        reader.lines().map(|line| -> Result<BatteryBank> {
            let line = line?;
            Ok(BatteryBank::parse_line(&line))
        })
    }

    pub fn max_joltage<const N: usize>(&self) -> u64 {
        let mut tmp = [0; N];
        for (i, b) in self.0.iter().enumerate().take(N) {
            tmp[i] = b.joltage();
        }
        for battery in self.0.iter().skip(N) {
            debug!("test {:?} {:?}", tmp, battery);
            let mut i = 0;
            while i < N - 1 && tmp[i] >= tmp[i + 1] {
                i += 1;
            }
            let mut pushed = false;
            while i < N - 1 {
                tmp[i] = tmp[i + 1];
                i += 1;
                pushed = true;
            }
            if pushed || (battery.joltage() > tmp[N - 1]) {
                tmp[N - 1] = battery.joltage();
            }
        }
        let mut n = 0;
        for d in tmp {
            n = n * 10 + (d as u64);
        }
        debug!(?n);
        n
    }
}

pub trait MaxJoltage {
    fn max_joltage(&self) -> u8;
}
