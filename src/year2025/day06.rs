use anyhow::{Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    let problems = parse_problems(input)?;
    let total = problems
        .into_iter()
        .map(|problem| problem.op.operate_on_slice(&problem.numbers))
        .sum::<u64>();
    Ok(total.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let total = process_cephalopodly(input)?;
    Ok(total.to_string())
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn operate_on_slice(&self, nums: &[u64]) -> u64 {
        let mut acc = match self {
            Op::Add => 0,
            Op::Mul => 1,
        };

        for num in nums {
            match self {
                Op::Add => acc += num,
                Op::Mul => acc *= num,
            }
        }

        acc
    }

    fn operate(&self, nums: (u64, u64)) -> u64 {
        match self {
            Op::Add => nums.0 + nums.1,
            Op::Mul => nums.0 * nums.1,
        }
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    op: Op,
}

fn parse_problems(input: &str) -> Result<Vec<Problem>> {
    let mut problems = Vec::new();

    let mut lines_rev = input.trim().lines().rev();
    let ops_line = lines_rev.next().expect("Line of operators should exist");

    for op_str in ops_line.split_whitespace() {
        let op = match op_str.trim() {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => bail!("Unhandled operator"),
        };

        problems.push(Problem {
            numbers: Vec::new(),
            op,
        });
    }

    for line in lines_rev {
        for (i, num_str) in line.split_whitespace().enumerate() {
            let value = num_str.parse().expect("Should be an integer");
            problems[i].numbers.push(value);
        }
    }

    Ok(problems)
}

fn process_cephalopodly(input: &str) -> Result<u64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 4277556.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 3263827.to_string());
        Ok(())
    }
}
