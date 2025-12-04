use std::collections::{HashSet, VecDeque};

use anyhow::Result;

use crate::utils::grid::Grid;

pub(crate) fn part1(input: &str) -> Result<String> {
    let grid = Grid::from_input(input)?;
    let accessible_rolls = get_accessible_rolls(&grid);
    Ok(accessible_rolls.len().to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let mut grid = Grid::from_input(input)?;
    let num_removed_rolls = remove_accessible_rolls(&mut grid);
    Ok(num_removed_rolls.to_string())
}

const ROLL: char = '@';
const EMPTY: char = '.';

fn count_surrounding_rolls(grid: &Grid<char>, i: usize, j: usize) -> u8 {
    grid.neighbors_8(i, j)
        .filter(|&((_, _), ch)| *ch == ROLL)
        .count() as u8
}

fn get_accessible_rolls(grid: &Grid<char>) -> Vec<(usize, usize)> {
    let mut rolls = Vec::new();

    for ((i, j), &ch) in grid.iter() {
        if ch == EMPTY {
            continue;
        }

        if count_surrounding_rolls(grid, i, j) < 4 {
            rolls.push((i, j));
        }
    }

    rolls
}

fn remove_accessible_rolls(grid: &mut Grid<char>) -> usize {
    let initial_rolls = get_accessible_rolls(grid);
    let mut to_remove: VecDeque<(usize, usize)> = initial_rolls.into();
    let mut scheduled: HashSet<(usize, usize)> = to_remove.iter().cloned().collect();
    let mut num_removed_rolls = 0;

    while let Some((i, j)) = to_remove.pop_front() {
        grid[(i, j)] = EMPTY;
        num_removed_rolls += 1;

        for ((ni, nj), &ch) in grid.neighbors_8(i, j) {
            if ch == ROLL && !scheduled.contains(&(ni, nj)) {
                let new_count = count_surrounding_rolls(grid, ni, nj);

                if new_count < 4 {
                    to_remove.push_back((ni, nj));
                    scheduled.insert((ni, nj));
                }
            }
        }
    }

    num_removed_rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 13.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 43.to_string());
        Ok(())
    }
}
