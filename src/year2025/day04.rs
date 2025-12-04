use std::collections::{HashSet, VecDeque};

use anyhow::Result;

use crate::utils::grid::{DIRECTIONS, Grid};

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
    let mut count = 0;
    for (di, dj) in DIRECTIONS {
        let ni = i as isize + di;
        let nj = j as isize + dj;

        if grid.is_within_bounds(ni, nj) && *grid.get(ni as usize, nj as usize) == ROLL {
            count += 1;
        }
    }
    count
}

fn get_accessible_rolls(grid: &Grid<char>) -> Vec<(usize, usize)> {
    let mut rolls = Vec::new();

    for j in 0..grid.height() {
        for i in 0..grid.width() {
            if *grid.get(i, j) == EMPTY {
                continue;
            }

            if count_surrounding_rolls(grid, i, j) < 4 {
                rolls.push((i, j));
            }
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
        *grid.get_mut(i, j) = EMPTY;
        num_removed_rolls += 1;

        for (di, dj) in DIRECTIONS {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if grid.is_within_bounds(ni, nj) {
                let ni_u = ni as usize;
                let nj_u = nj as usize;

                if *grid.get(ni_u, nj_u) == ROLL && !scheduled.contains(&(ni_u, nj_u)) {
                    let new_count = count_surrounding_rolls(grid, ni_u, nj_u);

                    if new_count < 4 {
                        to_remove.push_back((ni_u, nj_u));
                        scheduled.insert((ni_u, nj_u));
                    }
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
