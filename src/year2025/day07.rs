use anyhow::{Result, anyhow};
use std::collections::HashSet;

use crate::utils::grid::Grid;

const SPLITTER: char = '^';
const START: char = 'S';

struct Manifold {
    grid: Grid<char>,
    start_col: usize,
}

impl Manifold {
    fn parse(input: &str) -> Result<Self> {
        let grid = Grid::from_input(input)?;
        debug_assert!(!grid.is_empty());

        let first_row = grid.row(0);

        let start_col = first_row.iter().position(|&c| c == START).ok_or_else(|| {
            anyhow!(
                "Could not find starting point '{}' in the first row.",
                START
            )
        })?;

        Ok(Manifold { grid, start_col })
    }
}

pub(crate) fn part1(input: &str) -> Result<String> {
    let manifold = Manifold::parse(input)?;

    let mut active_beams: HashSet<usize> = HashSet::new();
    active_beams.insert(manifold.start_col);
    let mut split_count: u64 = 0;

    for row in 1..manifold.grid.height() {
        let mut next_beams: HashSet<usize> = HashSet::new();

        if active_beams.is_empty() {
            break;
        }

        for &col in &active_beams {
            match manifold.grid[(col, row)] {
                SPLITTER => {
                    split_count += 1;

                    if col > 0 {
                        next_beams.insert(col - 1);
                    }
                    if col + 1 < manifold.grid.width() {
                        next_beams.insert(col + 1);
                    }
                }
                _ => {
                    next_beams.insert(col);
                }
            }
        }

        active_beams = next_beams;
    }

    Ok(split_count.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let manifold = Manifold::parse(input)?;

    let mut timeline_counts: Vec<u64> = vec![0; manifold.grid.width()];
    timeline_counts[manifold.start_col] = 1;

    for row in 1..manifold.grid.height() {
        let mut next_counts: Vec<u64> = vec![0; manifold.grid.width()];

        for col in 0..manifold.grid.width() {
            let count = timeline_counts[col];

            if count == 0 {
                continue;
            }

            match manifold.grid[(col, row)] {
                SPLITTER => {
                    if col > 0 {
                        next_counts[col - 1] = next_counts[col - 1].saturating_add(count);
                    }
                    if col + 1 < manifold.grid.width() {
                        next_counts[col + 1] = next_counts[col + 1].saturating_add(count);
                    }
                }
                _ => {
                    next_counts[col] = next_counts[col].saturating_add(count);
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
