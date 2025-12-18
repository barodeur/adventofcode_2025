use crate::junction_box::JunctionBox;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::{collections::HashMap, io::BufRead};

pub struct Circuits {
    junction_boxes: Vec<JunctionBox>,
    circuit_id_by_box_id: HashMap<usize, usize>,
    pub box_ids_by_circuit_id: HashMap<usize, Vec<usize>>,
}

impl Circuits {
    pub fn from_reader(reader: impl BufRead) -> Result<Self> {
        let junction_boxes: Vec<_> = reader
            .lines()
            .map(|line| -> Result<JunctionBox> {
                let line = line?;
                line.parse()
            })
            .try_collect()?;
        let circuit_id_by_box_id: HashMap<_, _> =
            (0..junction_boxes.len()).map(|i| (i, i)).collect();
        let box_ids_by_circuit_id: HashMap<_, _> =
            (0..junction_boxes.len()).map(|i| (i, vec![i])).collect();

        Ok(Circuits {
            junction_boxes,
            circuit_id_by_box_id,
            box_ids_by_circuit_id,
        })
    }

    pub fn junction_boxes_pair_ids_by_distance_asc_iter(
        &self,
    ) -> impl Iterator<Item = (usize, usize)> {
        (0..self.junction_boxes.len())
            .tuple_combinations::<(_, _)>()
            .sorted_by(|(a, b), (c, d)| {
                self.junction_boxes[*a]
                    .distance(&self.junction_boxes[*b])
                    .cmp(&self.junction_boxes[*c].distance(&self.junction_boxes[*d]))
            })
    }

    pub fn get_circuit_id_by_box_id(&self, box_id: usize) -> Result<usize> {
        self.circuit_id_by_box_id
            .get(&box_id)
            .copied()
            .ok_or_else(|| eyre!("Cannot find circuit id for box id {}", box_id))
    }

    pub fn get_box_ids_by_circuit_id(&self, circuit_id: usize) -> Result<&Vec<usize>> {
        self.box_ids_by_circuit_id
            .get(&circuit_id)
            .ok_or_else(|| eyre!("Cannot find box ids for circuit id {}", circuit_id))
    }

    pub fn get_junction_box_by_id(&self, box_id: usize) -> Result<&JunctionBox> {
        self.junction_boxes
            .get(box_id)
            .ok_or_else(|| eyre!("Cannot find junction box for id {}", box_id))
    }

    pub fn merge_circuits(&mut self, c1_id: usize, c2_id: usize) -> Result<()> {
        if c1_id == c2_id {
            return Err(eyre!("Cannot merge circuit with itself"));
        }

        let c2_box_ids = self
            .box_ids_by_circuit_id
            .remove(&c2_id)
            .ok_or_else(|| eyre!("Cannot find box ids for circuit id {}", c2_id))?;
        let c1_box_ids = self
            .box_ids_by_circuit_id
            .get_mut(&c1_id)
            .ok_or_else(|| eyre!("Cannot find box ids for circuit id {}", c1_id))?;

        for c2_box_id in c2_box_ids {
            c1_box_ids.push(c2_box_id);
            self.circuit_id_by_box_id.insert(c2_box_id, c1_id);
        }

        Ok(())
    }

    pub fn merge_junction_boxes_circuits(&mut self, b1_id: usize, b2_id: usize) -> Result<bool> {
        let c1_id = self.get_circuit_id_by_box_id(b1_id)?;
        let c2_id = self.get_circuit_id_by_box_id(b2_id)?;

        if c1_id == c2_id {
            return Ok(false);
        }

        self.merge_circuits(c1_id, c2_id)?;

        Ok(true)
    }
}
