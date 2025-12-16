use aoc_runner::generate_main;
use d07::{parse, Item};
use eyre::Result;
use std::{collections::HashMap, io::BufRead};

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let mut beam_indices = HashMap::new();
    for items in parse(reader) {
        let items = items?;
        for (i, c) in items.iter().enumerate() {
            match c {
                Item::Source => {
                    beam_indices.insert(i, 1);
                }
                Item::Splitter => {
                    let Some(&count) = beam_indices.get(&i) else {
                        continue;
                    };
                    beam_indices.remove(&i);
                    beam_indices.insert(i - 1, beam_indices.get(&(i - 1)).unwrap_or(&0) + count);
                    beam_indices.insert(i + 1, beam_indices.get(&(i + 1)).unwrap_or(&0) + count);
                }
                Item::Empty => {}
            }
        }
    }
    Ok(beam_indices.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 40);
    }
}
