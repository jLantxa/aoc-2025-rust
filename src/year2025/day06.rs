use anyhow::{Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    let total = process_humanly(input)?;
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
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    op: Op,
}

/// Parse the problems as a normal person would do.
fn process_humanly(input: &str) -> Result<u64> {
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

    Ok(problems
        .into_iter()
        .map(|problem| problem.op.operate_on_slice(&problem.numbers))
        .sum::<u64>())
}

/// Process the math problems from right to left, as cephalopods do.
/// Yes, it is weird.
///
/// Apparently, parsing from right-to-left has equal or better performance.
fn process_cephalopodly(input: &str) -> Result<u64> {
    let lines: Vec<_> = input.lines().collect();
    let line_len = lines[0].len();
    let rev_lines: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().rev().collect::<Vec<char>>())
        .collect();
    let num_lines = rev_lines.len();

    let mut acc: u64 = 0;
    let mut stack = Vec::new();

    for i in 0..line_len {
        let mut chs: Vec<char> = Vec::with_capacity(num_lines);
        let mut is_empty_line = true;

        for rev_line in rev_lines.iter().take(num_lines) {
            let ch = rev_line[i];
            if ch != ' ' {
                is_empty_line = false;
            }
            chs.push(ch);
        }

        if is_empty_line {
            continue;
        }

        let num_str: String = chs[0..num_lines - 1].iter().collect();
        let num: u64 = num_str.trim().parse().expect("Should be a number");
        stack.push(num);

        let op_ch = chs[num_lines - 1];
        match op_ch {
            '+' => {
                acc += Op::Add.operate_on_slice(&stack);
                stack.clear();
            }
            '*' => {
                acc += Op::Mul.operate_on_slice(&stack);
                stack.clear();
            }
            ' ' => (),
            _ => bail!("Unhandled operator"),
        }
    }

    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

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
