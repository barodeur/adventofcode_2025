use eyre::{eyre, Result};
use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
pub struct Range(RangeInclusive<u64>);

impl FromStr for Range {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s.split_once('-').ok_or(eyre!("Invalid range format"))?;
        let start = start.parse().map_err(|_| eyre!("Invalid start value"))?;
        let end = end.parse().map_err(|_| eyre!("Invalid end value"))?;
        Ok(Self::new(start, end))
    }
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Range(start..=end)
    }

    pub fn contains(&self, value: u64) -> bool {
        self.0.contains(&value)
    }

    pub fn start(&self) -> &u64 {
        self.0.start()
    }

    pub fn end(&self) -> &u64 {
        self.0.end()
    }

    pub fn len(&self) -> u64 {
        self.0.end() - self.0.start() + 1
    }

    pub fn is_empty(&self) -> bool {
        self.0.start() > self.0.end()
    }
}
