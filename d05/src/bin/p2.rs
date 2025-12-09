use aoc_runner::generate_main;
use d05::{parse, range::Range};
use eyre::Result;
use std::{cmp, io::BufRead};

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let (mut ranges, _) = parse(reader)?;

    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut new_ranges: Vec<Range> = vec![];
    let mut tmp_range = None;
    for range in ranges {
        match tmp_range {
            None => {
                tmp_range = Some(range);
            }
            Some(inner_tmp_range) if inner_tmp_range.end() >= range.start() => {
                tmp_range = Some(Range::new(
                    *inner_tmp_range.start(),
                    cmp::max(*inner_tmp_range.end(), *range.end()),
                ));
            }
            Some(inner_tmp_range) => {
                new_ranges.push(inner_tmp_range);
                tmp_range = Some(range);
            }
        }
    }
    if let Some(inner_tmp_range) = tmp_range {
        new_ranges.push(inner_tmp_range);
    }

    Ok(new_ranges.iter().map(|range| range.len()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 14);
    }
}
