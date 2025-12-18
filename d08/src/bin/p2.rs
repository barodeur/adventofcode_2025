use aoc_runner::generate_main;
use d08::circuits::Circuits;
use eyre::{eyre, Result};
use std::io::BufRead;

generate_main!(solve);

fn solve(reader: impl BufRead) -> Result<u64> {
    let mut circuits = Circuits::from_reader(reader)?;
    let mut iter = circuits.junction_boxes_pair_ids_by_distance_asc_iter();

    let mut last_merged_box_ids = None;

    while circuits.box_ids_by_circuit_id.len() > 1 {
        let (b1_id, b2_id) = iter
            .next()
            .ok_or_else(|| eyre!("No more tuples to process"))?;

        if circuits.merge_junction_boxes_circuits(b1_id, b2_id)? {
            last_merged_box_ids = Some((b1_id, b2_id));
        }
    }

    let (b1_id, b2_id) = last_merged_box_ids.ok_or_else(|| eyre!("No tuples merged"))?;
    let b1 = circuits.get_junction_box_by_id(b1_id)?;
    let b2 = circuits.get_junction_box_by_id(b2_id)?;

    Ok(b1.x * b2.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() {
        let reader = std::io::Cursor::new(include_bytes!("../../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 25272);
    }
}
