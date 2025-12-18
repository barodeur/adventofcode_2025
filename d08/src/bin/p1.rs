use aoc_runner::generate_main;
use d08::circuits::Circuits;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::{env, io::BufRead};

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let args = env::args().collect_vec();
    let n = args
        .get(1)
        .ok_or_else(|| eyre!("missing number of iterations"))?;
    let n = n.parse()?;
    solve_n(reader, n)
}

fn solve_n(reader: impl BufRead, n: usize) -> Result<u64> {
    let mut circuits = Circuits::from_reader(reader)?;
    let mut iter = circuits.junction_boxes_pair_ids_by_distance_asc_iter();

    for _ in 0..n {
        let (b1_id, b2_id) = iter
            .next()
            .ok_or_else(|| eyre!("No more tuples to process"))?;

        circuits.merge_junction_boxes_circuits(b1_id, b2_id)?;
    }

    Ok(circuits
        .box_ids_by_circuit_id
        .values()
        .map(|boxes| boxes.len() as u64)
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve_n(reader, 10).unwrap();
        assert_eq!(result, 40);
    }
}
