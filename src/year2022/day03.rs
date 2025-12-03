use std::collections::HashSet;

use anyhow::{Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    let bags = parse_bags(input);

    let sum_of_duplicates_priorities: u64 = bags
        .iter()
        .map(|bag| find_duplicate_in_both_compartments(bag))
        .map(|result| result.and_then(get_item_priority))
        .collect::<Result<Vec<u64>>>()?
        .iter()
        .sum();

    Ok(sum_of_duplicates_priorities.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let bags = parse_bags(input);
    assert!(bags.len().is_multiple_of(3));

    let mut sum_badge_priorities = 0;
    for group in bags.as_slice().chunks(3) {
        let badge =
            find_badge(&group[0], &group[1], &group[2]).expect("The group should have a badge");

        let badge_priority = get_item_priority(badge)?;
        sum_badge_priorities += badge_priority;
    }

    Ok(sum_badge_priorities.to_string())
}

fn parse_bags(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

fn find_duplicate_in_both_compartments(bag: &str) -> Result<char> {
    let len = bag.len();
    let (c0, c1) = bag.split_at(len / 2);

    for item in c0.chars() {
        if c1.contains(item) {
            return Ok(item);
        }
    }

    bail!("No duplicate items found in different compartments")
}

fn get_item_priority(item: char) -> Result<u64> {
    if item.is_lowercase() {
        Ok(1 + (item as u64) - ('a' as u64))
    } else if item.is_uppercase() {
        Ok(27 + (item as u64) - ('A' as u64))
    } else {
        bail!("Invalid item {}", item)
    }
}

fn find_badge(b0: &str, b1: &str, b2: &str) -> Option<char> {
    let set0: HashSet<char> = b0.chars().collect();
    let set12: HashSet<char> = b1.chars().filter(|c| set0.contains(c)).collect();
    b2.chars().find(|c| set12.contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "157");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "70");

        Ok(())
    }
}
