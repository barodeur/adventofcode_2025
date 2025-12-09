use aoc_runner::generate_main;
use d04::room::Room;
use eyre::Result;
use std::io::BufRead;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let room = Room::parse(reader)?;

    Ok(room.accessible_rolls_count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 13);
    }
}
