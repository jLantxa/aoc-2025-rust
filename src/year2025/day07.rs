use anyhow::{Result, anyhow};
use std::collections::HashSet;

const SPLITTER: char = '^';
const START: char = 'S';

struct Manifold {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start_col: usize,
}

impl Manifold {
    fn parse(input: &str) -> Result<Self> {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        if grid.is_empty() {
            return Err(anyhow!("Input manifold is empty."));
        }

        let first_row = &grid[0];
        let width = first_row.len();
        let height = grid.len();

        if width == 0 {
            return Err(anyhow!("Manifold rows cannot be empty."));
        }

        let start_col = first_row.iter().position(|&c| c == START).ok_or_else(|| {
            anyhow!(
                "Could not find starting point '{}' in the first row.",
                START
            )
        })?;

        Ok(Manifold {
            grid,
            width,
            height,
            start_col,
        })
    }
}

pub(crate) fn part1(input: &str) -> Result<String> {
    let manifold = Manifold::parse(input)?;

    let mut active_beams: HashSet<usize> = HashSet::new();
    active_beams.insert(manifold.start_col);
    let mut split_count: u64 = 0;

    for r in 1..manifold.height {
        let mut next_beams: HashSet<usize> = HashSet::new();

        if active_beams.is_empty() {
            break;
        }

        for &c in &active_beams {
            match manifold.grid[r][c] {
                SPLITTER => {
                    split_count += 1;

                    if c > 0 {
                        next_beams.insert(c - 1);
                    }
                    if c + 1 < manifold.width {
                        next_beams.insert(c + 1);
                    }
                }
                _ => {
                    next_beams.insert(c);
                }
            }
        }

        active_beams = next_beams;
    }

    Ok(split_count.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let manifold = Manifold::parse(input)?;

    let mut timeline_counts: Vec<u64> = vec![0; manifold.width];

    timeline_counts[manifold.start_col] = 1;

    for r in 1..manifold.height {
        let mut next_counts: Vec<u64> = vec![0; manifold.width];

        for c in 0..manifold.width {
            let count = timeline_counts[c];

            if count == 0 {
                continue;
            }

            match manifold.grid[r][c] {
                SPLITTER => {
                    if c > 0 {
                        next_counts[c - 1] = next_counts[c - 1].saturating_add(count);
                    }
                    if c + 1 < manifold.width {
                        next_counts[c + 1] = next_counts[c + 1].saturating_add(count);
                    }
                }
                _ => {
                    next_counts[c] = next_counts[c].saturating_add(count);
                }
            }
        }

        timeline_counts = next_counts;
    }

    let total_timelines: u64 = timeline_counts.iter().sum();

    Ok(total_timelines.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 21.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 40.to_string());
        Ok(())
    }
}
