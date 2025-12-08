use anyhow::{Context, Result};

pub(crate) fn part1(input: &str) -> Result<String> {
    const NUM_PAIRS: usize = 1000;
    connect_boxes(input, NUM_PAIRS)
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let scp = StupidCircuitProblem::init(input)?;
    let (i, j) = scp.find_last_connection();

    let x_i = scp.boxes[i].0;
    let x_j = scp.boxes[j].0;

    Ok((x_i * x_j).to_string())
}

type Point = (i64, i64, i64);
type Edge = (usize, usize, i64);

fn connect_boxes(input: &str, num_pairs: usize) -> Result<String> {
    let scp = StupidCircuitProblem::init(input)?;
    let circuit_sizes = scp.connect(Some(num_pairs));

    Ok(StupidCircuitProblem::three_largest(&circuit_sizes).to_string())
}

struct StupidCircuitProblem {
    boxes: Vec<Point>,
}

impl StupidCircuitProblem {
    fn init(input: &str) -> Result<Self> {
        Ok(Self {
            boxes: parse_points(input)?,
        })
    }
    fn connect(&self, num_pairs: Option<usize>) -> Vec<usize> {
        let num_boxes = self.boxes.len();
        let mut union_find = UnionFind::new(num_boxes);

        let sorted_edges = get_sorted_edges(&self.boxes, num_pairs);

        for (i, j, _distance) in sorted_edges {
            union_find.union(i, j);
        }

        union_find.circuit_sizes()
    }

    fn find_last_connection(&self) -> (usize, usize) {
        let num_boxes = self.boxes.len();
        let mut uf = UnionFind::new(num_boxes);
        let sorted_edges = get_sorted_edges(&self.boxes, None);

        let mut last_connection = (0, 0);

        for (i, j, _d) in sorted_edges {
            if uf.union(i, j) {
                last_connection = (i, j);

                if uf.num_sets == 1 {
                    return last_connection;
                }
            }
        }

        last_connection
    }

    fn three_largest(sorted_circuit_sizes: &[usize]) -> usize {
        sorted_circuit_sizes.iter().take(3).product()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_sets: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_sets: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let mut root_i = self.find(i);
        let mut root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                std::mem::swap(&mut root_i, &mut root_j);
            }

            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
            self.num_sets -= 1;
            true
        } else {
            false
        }
    }

    fn circuit_sizes(&self) -> Vec<usize> {
        let mut final_sizes = Vec::new();
        for i in 0..self.parent.len() {
            if self.parent[i] == i {
                final_sizes.push(self.size[i]);
            }
        }

        final_sizes.sort_unstable_by(|a, b| b.cmp(a));
        final_sizes
    }
}

fn parse_points(input: &str) -> Result<Vec<Point>> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            let parts: Vec<_> = line.split(',').collect();

            if parts.len() != 3 {
                return Err(anyhow::anyhow!(
                    "Expected 3 coordinates, got {}",
                    parts.len()
                ))
                .context(format!("Parsing line {}", line_idx + 1));
            }

            Ok((
                parts[0].parse().context("Parsing x")?,
                parts[1].parse().context("Parsing y")?,
                parts[2].parse().context("Parsing z")?,
            ))
        })
        .collect::<Result<Vec<_>>>()
}

fn get_sorted_edges(boxes: &[Point], num_pairs: Option<usize>) -> Vec<Edge> {
    let mut distances: Vec<Edge> = Vec::new();
    let num_boxes = boxes.len();

    for i in 0..num_boxes {
        for j in (i + 1)..num_boxes {
            let bi = &boxes[i];
            let bj = &boxes[j];

            distances.push((i, j, distance2(bi, bj)));
        }
    }

    distances.sort_unstable_by_key(|&(_bi, _bj, distance)| distance);

    if let Some(n) = num_pairs {
        distances.truncate(n);
    }

    distances
}

#[inline]
fn distance2(p0: &Point, p1: &Point) -> i64 {
    (p1.0 - p0.0).pow(2) + (p1.1 - p0.1).pow(2) + (p1.2 - p0.2).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(connect_boxes(INPUT, 10)?, 40.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 25272.to_string());
        Ok(())
    }
}
