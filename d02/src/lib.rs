use color_eyre::eyre::{self, Result};
use std::{io::BufRead, ops::RangeInclusive};
use tracing::debug;

pub fn parse_reader(reader: impl BufRead) -> impl Iterator<Item = Result<RangeInclusive<u64>>> {
    reader
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
        })
}

pub fn sum_invalid_numbers(
    ranges: impl Iterator<Item = Result<RangeInclusive<u64>>>,
    is_invalid: impl Fn(u64) -> bool,
) -> u64 {
    ranges
        .flat_map(|range| -> Result<u64> {
            let range = range?;
            Ok(range.filter(|n| is_invalid(*n)).sum::<u64>())
        })
        .sum::<u64>()
}

pub fn has_n_repetitions(number: u64, n: u64) -> bool {
    let digits_count = number.ilog10() as u64 + 1;
    if digits_count % n != 0 {
        return false;
    }

    let pattern_size = digits_count / n;
    let power_ten_pattern = 10_u64.pow(pattern_size as u32);
    let pattern = number % power_ten_pattern;
    debug!(?pattern);

    let mut rest = number / power_ten_pattern;
    debug!(?rest);
    while rest != 0 {
        let candidate = rest % power_ten_pattern;
        rest /= power_ten_pattern;
        debug!(?candidate, ?rest);
        if candidate != pattern {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_has_n_repetitions_121212_3() {
        assert!(has_n_repetitions(121212_u64, 3));
    }

    #[test_log::test]
    fn test_has_n_repetitions_1212_2() {
        assert!(has_n_repetitions(1212_u64, 2));
    }

    #[test_log::test]
    fn test_has_n_repetitions_121312_3() {
        assert!(!has_n_repetitions(121312_u64, 3));
    }
}
