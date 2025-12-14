use aoc_runner::generate_main;
use eyre::Result;
use std::{collections::HashSet, io::BufRead};

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let mut count = 0;
    let mut beam_indices = HashSet::new();
    for line in reader.lines() {
        let line = line?;
        for (i, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    beam_indices.insert(i);
                }
                '^' => {
                    if beam_indices.contains(&i) {
                        beam_indices.remove(&i);
                        beam_indices.insert(i - 1);
                        beam_indices.insert(i + 1);
                        count += 1;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 21);
    }
}
