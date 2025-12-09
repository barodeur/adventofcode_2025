#[derive(Debug)]
pub struct Position {
    pub i: usize,
    pub j: usize,
}

impl Position {
    pub fn neighbors(&self) -> Vec<Position> {
        [
            (-1, 0),  // N
            (-1, 1),  // NE
            (0, 1),   // E
            (1, 1),   // SE
            (1, 0),   // S
            (1, -1),  // SW
            (0, -1),  // W
            (-1, -1), // NW
        ]
        .iter()
        .filter_map(|(di, dj)| {
            Some(Position {
                i: self.i.checked_add_signed(*di)?,
                j: self.j.checked_add_signed(*dj)?,
            })
        })
        .collect()
    }
}
