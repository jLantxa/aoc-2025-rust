use std::collections::HashSet;

use anyhow::{Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    let movements = parse_movements(input)?;
    let positions = simulate(&movements, 2);

    Ok(positions.len().to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let movements = parse_movements(input)?;
    let positions = simulate(&movements, 10);

    Ok(positions.len().to_string())
}

type PositionValue = i32;
type Position = (PositionValue, PositionValue);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Movement {
    direction: Direction,
    value: PositionValue,
}

fn parse_movements(input: &str) -> Result<Vec<Movement>> {
    let mut movements = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let movement: Movement = match (parts.next(), parts.next(), parts.next()) {
            (Some(ch), Some(num), None) => {
                let value = num.parse::<i32>().expect("Should be a number");
                let direction = match ch {
                    "U" => Direction::Up,
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    _ => bail!("Invalid direction"),
                };

                Movement { direction, value }
            }
            _ => bail!("Invalid line"),
        };

        movements.push(movement);
    }

    Ok(movements)
}

fn simulate(movements: &[Movement], num_knots: usize) -> HashSet<Position> {
    let mut positions = HashSet::new();
    let mut knots = vec![(0, 0); num_knots];

    positions.insert(*knots.last().expect("Tail should exist"));

    for movement in movements {
        for _ in 0..movement.value {
            // Move head
            match movement.direction {
                Direction::Up => knots[0].1 += 1,
                Direction::Right => knots[0].0 += 1,
                Direction::Down => knots[0].1 -= 1,
                Direction::Left => knots[0].0 -= 1,
            }

            // Propagate to next knot
            for i in 1..num_knots {
                knots[i] = move_tail(knots[i - 1], knots[i]);
            }

            // Update tail position
            positions.insert(*knots.last().expect("Tail should exist"));
        }
    }

    positions
}

fn move_tail(head: Position, mut tail: Position) -> Position {
    let (hx, hy) = head;
    let (tx, ty) = tail;

    let dx = hx - tx;
    let dy = hy - ty;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return tail;
    }

    if dx.abs() >= 1 {
        tail.0 += dx.signum();
    }
    if dy.abs() >= 1 {
        tail.1 += dy.signum();
    }

    tail
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT1)?;
        assert_eq!(solution, "13");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT2)?;
        assert_eq!(solution, "36");

        Ok(())
    }
}
