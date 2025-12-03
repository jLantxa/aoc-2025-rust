use std::collections::VecDeque;

use anyhow::{Result, anyhow, bail};

type OpFn = Box<dyn Fn(i64) -> i64>;

struct Monkey {
    items: VecDeque<i64>,
    operation: OpFn,
    test: (i64, usize, usize),
    examined_items_count: usize,
}

impl Monkey {}

fn parse_monkeys(input: &str) -> Result<Vec<Monkey>> {
    let blocks = input.trim().split("\n\n");

    let mut monkeys = Vec::new();

    for block in blocks {
        match parse_single_monkey(block) {
            Ok(monkey) => monkeys.push(monkey),
            Err(e) => bail!(format!("Error parsing monkey block: {}", e)),
        }
    }

    Ok(monkeys)
}

fn parse_single_monkey(block: &str) -> Result<Monkey> {
    let mut lines = block.lines().map(|l| l.trim());

    let _id_line = lines.next().ok_or(anyhow!("Missing ID line"))?;

    let items_line = lines.next().ok_or(anyhow!("Missing items line"))?;
    let items_str = items_line
        .strip_prefix("Starting items: ")
        .ok_or(anyhow!("Bad items format"))?;

    let items: VecDeque<i64> = items_str
        .split(", ")
        .map(|s| s.parse::<i64>().expect("Value should be a number"))
        .collect::<VecDeque<i64>>();

    let op_line = lines.next().ok_or(anyhow!("Missing operation line"))?;
    let op_str = op_line
        .strip_prefix("Operation: new = ")
        .ok_or(anyhow!("Bad operation format"))?;
    let operation = parse_operation(op_str)?;

    let test_line = lines.next().ok_or(anyhow!("Missing test line"))?;

    let test_divisor = test_line
        .split("by ")
        .nth(1)
        .ok_or(anyhow!("Bad divisor format"))?
        .parse::<i64>()
        .expect("Value should be a number");

    let true_line = lines.next().ok_or(anyhow!("Missing true line"))?;
    let true_target = true_line
        .split("monkey ")
        .nth(1)
        .ok_or(anyhow!("Bad true target format"))?
        .parse::<usize>()
        .expect("Value should be a number");

    let false_line = lines.next().ok_or(anyhow!("Missing false line"))?;
    let false_target = false_line
        .split("monkey ")
        .nth(1)
        .ok_or(anyhow!("Bad false target format"))?
        .parse::<usize>()
        .expect("Value should be a number");

    let test = (test_divisor, true_target, false_target);

    Ok(Monkey {
        items,
        operation,
        test,
        examined_items_count: 0,
    })
}

fn parse_operation(s: &str) -> Result<OpFn> {
    let parts: Vec<&str> = s.split_whitespace().collect();

    if parts.len() == 3 && parts[0] == "old" && parts[2] == "old" && parts[1] == "*" {
        return Ok(Box::new(|old| old * old));
    }

    if parts.len() == 3 && parts[0] == "old" {
        let op = parts[1];

        let val = parts[2].parse::<i64>().expect("Value should be a number");

        match op {
            "*" => return Ok(Box::new(move |old| old * val)),
            "+" => return Ok(Box::new(move |old| old + val)),
            _ => bail!("Unknown operator: {}", op),
        }
    }

    bail!("Malformed operation string: {}", s)
}

fn round(monkeys: &mut [Monkey], worry_divider: i64, modulus: i64) {
    let num_monkeys = monkeys.len();

    for i in 0..num_monkeys {
        monkeys[i].examined_items_count += monkeys[i].items.len();

        let old_items = std::mem::take(&mut monkeys[i].items);

        for old in old_items {
            let mut new = (monkeys[i].operation)(old);

            new /= worry_divider;
            new %= modulus;

            let test_params = monkeys[i].test;

            let to_monkey: usize = if new % test_params.0 == 0 {
                test_params.1
            } else {
                test_params.2
            };

            monkeys[to_monkey].items.push_back(new);
        }
    }
}

fn simulate(monkeys: &mut [Monkey], worry_divider: i64, rounds: usize) -> usize {
    let modulus = monkeys.iter().map(|monkey| monkey.test.0).product();

    for _ in 0..rounds {
        round(monkeys, worry_divider, modulus);
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.examined_items_count);
    let monkey_business: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.examined_items_count)
        .product();

    monkey_business
}

pub(crate) fn part1(input: &str) -> Result<String> {
    let mut monkeys = parse_monkeys(input)?;
    let monkey_business = simulate(&mut monkeys, 3, 20);
    Ok(monkey_business.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let mut monkeys = parse_monkeys(input)?;
    let monkey_business = simulate(&mut monkeys, 1, 10000);
    Ok(monkey_business.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "10605");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "2713310158");

        Ok(())
    }
}
