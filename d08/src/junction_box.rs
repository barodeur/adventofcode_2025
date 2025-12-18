use eyre::eyre;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct JunctionBox {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl JunctionBox {
    pub fn distance(&self, other: &JunctionBox) -> u64 {
        (self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2)
    }
}

impl FromStr for JunctionBox {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',').map(|s| s.parse::<u64>());

        match (it.next(), it.next(), it.next(), it.next()) {
            (Some(Ok(x)), Some(Ok(y)), Some(Ok(z)), None) => Ok(JunctionBox { x, y, z }),
            _ => Err(eyre!("Invalid input format")),
        }
    }
}
