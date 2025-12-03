use std::collections::VecDeque;

use anyhow::{Result, bail};
use regex::Regex;

pub(crate) fn part1(input: &str) -> Result<String> {
    let (mut stacks, instructions) = parse(input)?;

    for instruction in instructions {
        instruction.apply_9000(&mut stacks);
    }

    let top_crates: String = stacks
        .iter()
        .map(|stack| *stack.back().expect("Crate should exist"))
        .collect();

    Ok(top_crates)
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let (mut stacks, instructions) = parse(input)?;

    for instruction in instructions {
        instruction.apply_9001(&mut stacks);
    }

    let top_crates: String = stacks
        .iter()
        .map(|stack| *stack.back().expect("Crate should exist"))
        .collect();

    Ok(top_crates)
}

type Stack = VecDeque<char>;

#[derive(Debug)]
struct Instruction {
    size: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn apply_9000(&self, stacks: &mut [Stack]) {
        for _ in 0..self.size {
            let cr = stacks[self.from - 1]
                .pop_back()
                .expect("There should be a crate");
            stacks[self.to - 1].push_back(cr);
        }
    }

    fn apply_9001(&self, stacks: &mut [Stack]) {
        let mut hold = VecDeque::new();

        for _ in 0..self.size {
            let cr = stacks[self.from - 1]
                .pop_back()
                .expect("There should be a crate");
            hold.push_front(cr);
        }

        stacks[self.to - 1].append(&mut hold);
    }
}

fn parse(input: &str) -> Result<(Vec<Stack>, Vec<Instruction>)> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Invalid regex");

    let mut parsing_stack = true;
    for line in input.lines() {
        if line.is_empty() {
            parsing_stack = false;
            continue;
        }

        if parsing_stack {
            if !line.contains("[") {
                continue;
            }

            let mut chars = line.chars();
            let mut stack_index = 0;
            loop {
                let chunk: String = chars.by_ref().take(3).collect();

                if chunk.is_empty() {
                    break;
                } else if chunk.starts_with("[") && chunk.ends_with("]") {
                    let ch = chunk.chars().nth(1).expect("Crate char should exist");
                    while stacks.len() <= stack_index {
                        stacks.push(VecDeque::new());
                    }
                    stacks[stack_index].push_front(ch);
                }

                chars.next();
                stack_index += 1;
            }
        } else if let Some(captures) = re.captures(line) {
            let size = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            instructions.push(Instruction { size, from, to });
        } else {
            bail!("Invalid instruction {line}")
        }
    }

    Ok((stacks, instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "CMZ");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "MCD");

        Ok(())
    }
}
