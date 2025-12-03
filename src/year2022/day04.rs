use std::ops::RangeInclusive;

use anyhow::{Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    let ranges = parse_ranges(input)?;

    Ok(ranges
        .iter()
        .filter(|(r0, r1)| range_contains_range(r0, r1) || range_contains_range(r1, r0))
        .count()
        .to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let ranges = parse_ranges(input)?;

    Ok(ranges
        .iter()
        .filter(|(r0, r1)| range_overlaps_range(r0, r1))
        .count()
        .to_string())
}

fn parse_range(s: &str) -> Result<RangeInclusive<u32>> {
    let (start, end) = match s.split_once("-") {
        Some((s, e)) => (s.parse::<u32>()?, e.parse::<u32>()?),
        None => bail!("Invalid range"),
    };

    Ok(start..=end)
}

fn parse_ranges(input: &str) -> Result<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let mut ranges = Vec::new();

    for line in input.lines() {
        let (r0, r1) = match line.split_once(",") {
            Some((s0, s1)) => (parse_range(s0)?, parse_range(s1)?),
            None => bail!("Invalid line"),
        };

        ranges.push((r0, r1));
    }

    Ok(ranges)
}

fn range_contains_range(r0: &RangeInclusive<u32>, r1: &RangeInclusive<u32>) -> bool {
    r0.end() < r1.start() || r0.start() > r1.end()
}

fn range_overlaps_range(r0: &RangeInclusive<u32>, r1: &RangeInclusive<u32>) -> bool {
    !(r0.end() < r1.start() || r0.start() > r1.end())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "2");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "4");

        Ok(())
    }
}
