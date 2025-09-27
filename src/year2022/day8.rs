use anyhow::Result;

use crate::utils::grid::Grid;

pub(crate) fn part1(input: &str) -> Result<String> {
    let grid = Grid::from_input_with(input, |ch| ch.to_digit(10).expect("Char should be a digit"))?;

    let mut num_visible_trees = 0;
    for i in 0..grid.width() {
        for j in 0..grid.height() {
            if is_tree_visible(&grid, i, j) {
                num_visible_trees += 1;
            }
        }
    }

    Ok(num_visible_trees.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let grid = Grid::from_input_with(input, |ch| ch.to_digit(10).expect("Char should be a digit"))?;

    let mut scores = Vec::new();

    for i in 0..grid.width() {
        for j in 0..grid.height() {
            scores.push(scenic_score(&grid, i, j));
        }
    }

    let max_score = scores.iter().max().expect("There should be a max value");

    Ok(max_score.to_string())
}

fn is_tree_visible(grid: &Grid<u32>, i: usize, j: usize) -> bool {
    let width = grid.width();
    let height = grid.height();

    if i == 0 || i == width - 1 || j == 0 || j == height - 1 {
        return true;
    }

    let tree_height = grid.get(i, j);

    // Left side
    let mut left = true;
    for d in 0..i {
        if grid.get(d, j) >= tree_height {
            left = false;
            break;
        }
    }

    // Right side
    let mut right = true;
    for d in i + 1..width {
        if grid.get(d, j) >= tree_height {
            right = false;
            break;
        }
    }

    let mut top = true;
    for d in 0..j {
        if grid.get(i, d) >= tree_height {
            top = false;
            break;
        }
    }

    let mut bottom = true;
    for d in j + 1..height {
        if grid.get(i, d) >= tree_height {
            bottom = false;
            break;
        }
    }

    left || right || top || bottom
}

fn scenic_score(grid: &Grid<u32>, i: usize, j: usize) -> u32 {
    let width = grid.width();
    let height = grid.height();

    let tree_height = grid.get(i, j);

    // Left side
    let mut left = 0;
    for d in 1..=i {
        left += 1;
        if grid.get(i - d, j) >= tree_height {
            break;
        }
    }

    // Right side
    let mut right = 0;
    for d in (i + 1)..width {
        right += 1;
        if grid.get(d, j) >= tree_height {
            break;
        }
    }

    // Top side
    let mut top = 0;
    for d in 1..=j {
        top += 1;
        if grid.get(i, j - d) >= tree_height {
            break;
        }
    }

    // Bottom side
    let mut bottom = 0;
    for d in (j + 1)..height {
        bottom += 1;
        if grid.get(i, d) >= tree_height {
            break;
        }
    }

    left * right * top * bottom
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "21");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "8");

        Ok(())
    }
}
