use anyhow::{Context, Result};

pub(crate) fn part1(input: &str) -> Result<String> {
    let calories = parse_calories(input)?;

    let max_calories = calories
        .iter()
        .max()
        .expect("There should be at least one elf");

    Ok(max_calories.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let mut calories = parse_calories(input)?;
    calories.sort_unstable();

    // We assume there are at least three elves
    assert!(calories.len() >= 3);

    let total_top_three_calories: u64 = calories.iter().rev().take(3).sum();

    Ok(total_top_three_calories.to_string())
}

fn parse_calories(input: &str) -> Result<Vec<u64>> {
    let mut calories = Vec::new();

    let mut acc = 0;
    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            calories.push(acc);
            acc = 0;
        } else {
            acc += line
                .parse::<u64>()
                .with_context(|| "Expected a number when parsing input")?;
        }
    }

    calories.push(acc);

    Ok(calories)
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "24000");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "45000");

        Ok(())
    }
}
