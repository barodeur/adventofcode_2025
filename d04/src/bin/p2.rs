use aoc_runner::generate_main;
use d04::room::Room;
use eyre::Result;
use std::io::BufRead;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let mut room = Room::parse(reader)?;

    Ok((0..)
        .map(|_| room.remove_accessible_rolls())
        .take_while(|removed_count| *removed_count > 0)
        .sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 43);
    }
}
