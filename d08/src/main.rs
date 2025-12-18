use aoc_runner::generate_main;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env,
    io::BufRead,
    str::FromStr,
};

#[derive(Debug, Eq, Hash, PartialEq)]
struct JunctionBox((u64, u64, u64));

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> u64 {
        let (x1, y1, z1) = self.0;
        let (x2, y2, z2) = other.0;
        (x1.abs_diff(x2)).pow(2) + (y1.abs_diff(y2)).pow(2) + (z1.abs_diff(z2)).pow(2)
    }
}

impl FromStr for JunctionBox {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',').map(|s| s.parse::<u64>());

        match (it.next(), it.next(), it.next(), it.next()) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c)), None) => Ok(JunctionBox((a, b, c))),
            _ => Err(eyre!("Invalid input format")),
        }
    }
}

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
    let boxes: HashSet<_> = reader
        .lines()
        .map(|line| -> Result<JunctionBox> {
            let line = line?;
            line.parse()
        })
        .try_collect()?;
    let mut box_to_circuit: HashMap<&JunctionBox, usize> = HashMap::new();
    let mut circuit_boxes: HashMap<usize, HashSet<&JunctionBox>> = HashMap::new();

    for (i, b) in boxes.iter().enumerate() {
        box_to_circuit.insert(b, i);
        circuit_boxes.insert(i, HashSet::from([b]));
    }

    let mut sorted_tuples_iter =
        boxes
            .iter()
            .tuple_combinations::<(_, _)>()
            .sorted_by(|(a, b), (c, d)| {
                let dist_a = a.distance(b);
                let dist_c = c.distance(d);
                dist_a.cmp(&dist_c)
            });

    for _ in 0..n {
        let tuple = sorted_tuples_iter
            .next()
            .ok_or_else(|| eyre!("No more tuples to process"))?;

        let (b1, b2) = tuple;
        let b1_circuit_idx = *box_to_circuit
            .get(b1)
            .ok_or_else(|| eyre!("Cannot find b1 circuit index"))?;
        let b2_circuit_idx = *box_to_circuit
            .get(b2)
            .ok_or_else(|| eyre!("Cannot find b2 circuit index"))?;
        if b1_circuit_idx == b2_circuit_idx {
            continue;
        }
        let b2_circuit_boxes = circuit_boxes
            .remove(&b2_circuit_idx)
            .ok_or_else(|| eyre!("Cannot find b2 circuit boxes"))?;
        let b1_circuit_boxes = circuit_boxes
            .get_mut(&b1_circuit_idx)
            .ok_or_else(|| eyre!("Cannot find b1 circuit boxes"))?;
        for b2_box in b2_circuit_boxes {
            b1_circuit_boxes.insert(b2_box);
            box_to_circuit.insert(b2_box, b1_circuit_idx);
        }
    }

    Ok(circuit_boxes
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
        let reader = std::io::Cursor::new(include_bytes!("../inputs/example.txt"));
        let result = solve(reader).unwrap();
        assert_eq!(result, 40);
    }
}
