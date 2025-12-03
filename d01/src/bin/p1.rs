use aoc_runner::generate_main;
use color_eyre::Result;
use d01::rotation::Rotation;
use std::io::BufRead;
use tracing::debug;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let rotations = Rotation::iter_from(reader);

    let mut position = 50;
    let mut count = 0;

    for rotation in rotations {
        let rotation = rotation?;
        debug!(?position, ?rotation);

        position = (100 + position + (rotation.angle() % 100)) % 100;

        if position == 0 {
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 3);
    }
}
