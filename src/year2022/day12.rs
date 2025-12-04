use std::collections::{HashSet, VecDeque};

use anyhow::{Result, bail};

use crate::utils::grid::Grid;

type Height = i8;
type Point = (usize, usize);

pub(crate) fn part1(input: &str) -> Result<String> {
    let (grid, start, end) = parse_input(input)?;
    let starting_points = vec![start];
    let distance = find_shortest_path_multipoint(&grid, &starting_points, end)
        .expect("There should be a path");
    Ok(distance.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let (grid, _start, end) = parse_input(input)?;
    let mut starting_points = Vec::new();

    for j in 0..grid.height() {
        for i in 0..grid.width() {
            if *grid.get(i, j) == 0 {
                starting_points.push((i, j));
            }
        }
    }

    let distance = find_shortest_path_multipoint(&grid, &starting_points, end)
        .expect("There should be a path");
    Ok(distance.to_string())
}

fn find_shortest_path_multipoint(
    grid: &Grid<Height>,
    start_points: &[Point],
    end: Point,
) -> Option<usize> {
    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();

    for &start in start_points {
        queue.push_back((start, 0));
        visited.insert(start);
    }

    while let Some(((x, y), distance)) = queue.pop_front() {
        let current_height = *grid.get(x, y);

        if (x, y) == end {
            return Some(distance);
        }

        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in directions.iter() {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if grid.is_within_bounds(nx as isize, ny as isize) {
                let neighbor_point = (nx, ny);
                let neighbor_height = *grid.get(nx, ny);

                if neighbor_height <= current_height + 1 && visited.insert(neighbor_point) {
                    queue.push_back((neighbor_point, distance + 1));
                }
            }
        }
    }

    None
}

fn parse_input(input: &str) -> Result<(Grid<Height>, Point, Point)> {
    let letter_grid = Grid::from_input(input)?;
    let start = find_point(&letter_grid, 'S').expect("Start exist");
    let end = find_point(&letter_grid, 'E').expect("End exist");

    let grid = Grid::from_input_with(input, letter_to_height)?;

    Ok((grid, start, end))
}

fn find_point(grid: &Grid<char>, ch: char) -> Result<Point> {
    for j in 0..grid.height() {
        for i in 0..grid.width() {
            if *grid.get(i, j) == ch {
                return Ok((i, j));
            }
        }
    }

    bail!("Point not found")
}

fn letter_to_height(ch: char) -> Height {
    if ch == 'S' {
        0
    } else if ch == 'E' {
        26
    } else {
        ch as Height - 'a' as Height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "31");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "29");

        Ok(())
    }
}
