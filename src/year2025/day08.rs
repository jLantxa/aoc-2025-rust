use anyhow::Result;

pub(crate) fn part1(input: &str) -> Result<String> {
    connect_part1(input, 1000)
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let scp = StupidCircuitProblem::init(input)?;
    let (i, j) = scp.find_last_connection();

    let x_i = scp.boxes[i].0;
    let x_j = scp.boxes[j].0;

    Ok((x_i * x_j).to_string())
}

type Box = (i64, i64, i64);

fn connect_part1(input: &str, num_pairs: usize) -> Result<String> {
    let mut scp = StupidCircuitProblem::init(input)?;
    let circuit_sizes = scp.conectilear(Some(num_pairs));

    let mut sorted_sizes = circuit_sizes;
    sorted_sizes.sort_unstable_by(|a, b| b.cmp(a));

    Ok(StupidCircuitProblem::three_largest(&sorted_sizes).to_string())
}

struct StupidCircuitProblem {
    boxes: Vec<Box>,
}

impl StupidCircuitProblem {
    fn init(input: &str) -> Result<Self> {
        Ok(Self {
            boxes: parse_boxes(input)?,
        })
    }

    fn conectilear(&mut self, num_pairs: Option<usize>) -> Vec<usize> {
        let num_boxes = self.boxes.len();

        let sorted_distances = sorted_distances_indexed(&self.boxes, num_pairs);

        let mut parent: Vec<usize> = (0..num_boxes).collect();
        let mut circuit_size: Vec<usize> = vec![1; num_boxes];

        fn find_root(parent: &mut [usize], i: usize) -> usize {
            if parent[i] == i {
                return i;
            }
            parent[i] = find_root(parent, parent[i]);
            parent[i]
        }

        for (i, j, _d) in sorted_distances.into_iter().rev() {
            let root_i = find_root(&mut parent, i);
            let root_j = find_root(&mut parent, j);

            if root_i != root_j {
                if circuit_size[root_i] < circuit_size[root_j] {
                    parent[root_i] = root_j;
                    circuit_size[root_j] += circuit_size[root_i];
                } else {
                    parent[root_j] = root_i;
                    circuit_size[root_i] += circuit_size[root_j];
                }
            }
        }

        let mut final_sizes = Vec::new();
        for i in 0..num_boxes {
            if parent[i] == i {
                final_sizes.push(circuit_size[i]);
            }
        }

        final_sizes
    }

    fn find_last_connection(&self) -> (usize, usize) {
        let num_boxes = self.boxes.len();

        let sorted_distances = sorted_distances_indexed(&self.boxes, None);

        let mut parent: Vec<usize> = (0..num_boxes).collect();
        let mut num_circuits = num_boxes;
        let mut last_connection = (0, 0);

        fn find_root(parent: &mut [usize], i: usize) -> usize {
            if parent[i] == i {
                return i;
            }
            parent[i] = find_root(parent, parent[i]);
            parent[i]
        }

        for (i, j, _d) in sorted_distances.into_iter() {
            let root_i = find_root(&mut parent, i);
            let root_j = find_root(&mut parent, j);

            if root_i != root_j {
                parent[root_i] = root_j;

                last_connection = (i, j);
                num_circuits -= 1;

                if num_circuits == 1 {
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

fn parse_boxes(input: &str) -> Result<Vec<Box>> {
    Ok(input
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(",").collect();
            (
                parts[0].parse().expect("Should have x"),
                parts[1].parse().expect("Should have y"),
                parts[2].parse().expect("Should have z"),
            )
        })
        .collect::<Vec<_>>())
}

fn sorted_distances_indexed(boxes: &[Box], num_pairs: Option<usize>) -> Vec<(usize, usize, i64)> {
    let mut distances = Vec::new();
    let num_boxes = boxes.len();

    for i in 0..num_boxes {
        for j in (i + 1)..num_boxes {
            let bi = &boxes[i];
            let bj = &boxes[j];

            distances.push((i, j, distance2(bi, bj)));
        }
    }

    distances.sort_unstable_by(|&(_, _, d0), &(_, _, d1)| {
        d0.partial_cmp(&d1).expect("Should be comparable")
    });

    if let Some(n) = num_pairs {
        distances.truncate(n);
    }

    distances
}

#[inline]
fn distance2(b0: &Box, b1: &Box) -> i64 {
    (b1.0 - b0.0).pow(2) + (b1.1 - b0.1).pow(2) + (b1.2 - b0.2).pow(2)
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
        assert_eq!(connect_part1(INPUT, 10)?, 40.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 25272.to_string());
        Ok(())
    }
}
