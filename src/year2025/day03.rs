use anyhow::Result;

pub(crate) fn part1(input: &str) -> Result<String> {
    // Part 1 calculates the joltage with 2 batteries

    let banks = read_banks(input);
    Ok(banks
        .iter()
        .map(|bank| max_joltage(bank, 2))
        .sum::<Joltage>()
        .to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    // Part 2 calculates the joltage with 12 batteries

    let banks = read_banks(input);
    Ok(banks
        .iter()
        .map(|bank| max_joltage(bank, 12))
        .sum::<Joltage>()
        .to_string())
}

type Digit = u8;
type Joltage = u64;
type Bank = Vec<Digit>;

fn read_banks(input: &str) -> Vec<Bank> {
    input.lines().map(|l| parse_bank(l)).collect()
}

fn parse_bank(s: &str) -> Bank {
    s.chars()
        .map(|c| c.to_digit(10).expect("Should be a digit") as Digit)
        .collect()
}

fn max_joltage(bank: &Bank, size: usize) -> Joltage {
    let bank_len = bank.len();
    assert!(
        bank_len >= size,
        "This bank is smaller than the size ({} < {})",
        bank_len,
        size
    );

    let mut joltage: Joltage = 0;
    let mut n = 0;
    let mut start: usize = 0;

    while n < size {
        let end = (bank_len - size) + (n + 1);
        let (relative_index, best_digit) = &bank[start..end]
            .iter()
            .enumerate()
            .max_by(|&(ai, av), &(bi, bv)| match av.cmp(bv) {
                std::cmp::Ordering::Equal => ai.cmp(&bi).reverse(),
                ordering => ordering,
            })
            .map(|(index, &value)| (index, value))
            .expect("Slice should not be empty");

        joltage = (joltage * 10) + (*best_digit as Joltage);
        start += relative_index + 1;
        n += 1;
    }

    joltage
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 357_usize.to_string());

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 3121910778619_u64.to_string());

        Ok(())
    }
}
