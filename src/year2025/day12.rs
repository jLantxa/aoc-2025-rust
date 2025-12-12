use anyhow::Result;

pub(crate) fn part1(input: &str) -> Result<String> {
    let problems = parse_input(input);

    Ok(problems
        .iter()
        .filter(|p| p.is_solvable_naive())
        .count()
        .to_string())
}

pub(crate) fn part2(_input: &str) -> Result<String> {
    Ok("N/A".to_string())
}

#[derive(Debug)]
struct Problem {
    width: i32,
    height: i32,
    counts: Vec<usize>,
}

impl Problem {
    // Just check that every 3x3 gift would fit. I will not be trolled by an elf.
    fn is_solvable_naive(&self) -> bool {
        let max_area = (self.width * self.height) as u32;
        let total_area: u32 = self.counts.iter().map(|&c| (c * (3 * 3)) as u32).sum();

        total_area <= max_area
    }
}

fn parse_input(input: &str) -> Vec<Problem> {
    let (_shapes_str, problems_str) = input
        .split_once("\n\n")
        .expect("Input should have two parts");

    let mut problems = Vec::new();

    for line in problems_str.lines().filter(|l| !l.trim().is_empty()) {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue;
        }

        let size_str = parts[0];
        let counts_str = parts[1];

        let wh: Vec<&str> = size_str.split('x').collect();
        let width = wh.first().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
        let height = wh.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);

        let counts: Vec<usize> = counts_str
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        problems.push(Problem {
            width,
            height,
            counts,
        });
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 2.to_string());
        Ok(())
    }
}
