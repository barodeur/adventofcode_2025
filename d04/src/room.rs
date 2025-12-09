use crate::{position::Position, roll::Roll};
use eyre::{eyre, Result};
use itertools::Itertools;
use std::io::BufRead;

#[derive(Debug)]
pub struct Room(Vec<Vec<Option<Roll>>>);

impl Room {
    pub fn parse(reader: impl BufRead) -> Result<Self> {
        Ok(Room(
            reader
                .lines()
                .map(|line| -> Result<Vec<Option<Roll>>> {
                    let line = line?;
                    let row: Vec<Option<Roll>> = line
                        .chars()
                        .map(|c| match c {
                            '.' => Ok(None),
                            '@' => Ok(Some(Roll)),
                            _ => Err(eyre!("Invalid character in input")),
                        })
                        .try_collect()?;
                    Ok(row)
                })
                .try_collect()?,
        ))
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn neighbors(&self, position: &Position) -> Vec<Position> {
        position
            .neighbors()
            .into_iter()
            .filter(|pos| pos.i < self.height() && pos.j < self.width())
            .collect()
    }

    fn is_roll_at_position(&self, position: &Position) -> bool {
        self.0[position.i][position.j] == Some(Roll)
    }

    fn neighbors_with_roll_count(&self, position: &Position) -> usize {
        self.neighbors(position)
            .into_iter()
            .filter(|pos| self.is_roll_at_position(pos))
            .count()
    }

    fn positions(&self) -> Vec<Position> {
        (0..self.height())
            .flat_map(|i| (0..self.width()).map(move |j| Position { i, j }))
            .collect()
    }

    pub fn accessible_rolls_count(&self) -> usize {
        self.accessible_roll_positions().len()
    }

    fn accessible_roll_positions(&self) -> Vec<Position> {
        self.positions()
            .into_iter()
            .filter(|pos| self.is_roll_at_position(pos))
            .filter(|pos| self.neighbors_with_roll_count(pos) < 4)
            .collect()
    }
}
