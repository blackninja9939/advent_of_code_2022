use color_eyre::eyre::{eyre, Result};
use std::{ops::RangeInclusive, str::FromStr};

struct Range(RangeInclusive<usize>);

impl FromStr for Range {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let lhs = split.next().ok_or_else(|| eyre!("No split point"))?;
        let rhs = split.next().ok_or_else(|| eyre!("No split point"))?;
        if split.count() != 0 {
            return Err(eyre!("Range should be in the form of <start>-<end>"));
        }

        let start = lhs.parse::<usize>()?;
        let end = rhs.parse::<usize>()?;
        Ok(Self(start..=end))
    }
}

impl Range {
    fn contains_range(&self, other: &Range) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }

    fn overlaps_range(&self, other: &Range) -> bool {
        self.0.contains(other.0.start()) || self.0.contains(other.0.end())
    }
}

struct Pair {
    first: Range,
    second: Range,
}

impl FromStr for Pair {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let lhs = split.next().ok_or_else(|| eyre!("No split point"))?;
        let rhs = split.next().ok_or_else(|| eyre!("No split point"))?;
        if split.count() != 0 {
            return Err(eyre!("Pair should be in the form of <range>,<range>"));
        }
        Ok(Self {
            first: lhs.parse::<Range>()?,
            second: rhs.parse::<Range>()?,
        })
    }
}

impl Pair {
    fn contains_other(&self) -> bool {
        self.first.contains_range(&self.second) || self.second.contains_range(&self.first)
    }
    fn overlaps_other(&self) -> bool {
        self.first.overlaps_range(&self.second) || self.second.overlaps_range(&self.first)
    }
}

pub fn puzzle_1(data: String) {
    let overlapping_count = data
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .filter(|p| p.contains_other())
        .count();
    println!(
        "Total pairs with fully overlapping ranges {}",
        overlapping_count
    );
}

pub fn puzzle_2(data: String) {
    let overlapping_count = data
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .filter(|p| p.overlaps_other())
        .count();
    println!(
        "Total pairs with fully overlapping ranges {}",
        overlapping_count
    );
}
