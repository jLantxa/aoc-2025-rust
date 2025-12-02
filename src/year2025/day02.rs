use std::ops::RangeInclusive;

use anyhow::Result;

pub(crate) fn part1(input: &str) -> Result<String> {
    // Part 1 checks for a sequence repeated exactly twice (e.g., 1212)
    let filter = |id: i64| -> bool {
        assert!(id <= 9_999_999_999, "ID {id} is too big to be handled");

        match id {
            // Check for 'AA' (len 2) - Divisible by 11
            10..=99 => id % 11 != 0,
            // Check for 'ABAB' (len 4) - Divisible by 101
            1_000..=9_999 => id % 101 != 0,
            // Check for 'ABCABC' (len 6) - Divisible by 1_001
            100_000..=999_999 => id % 1_001 != 0,
            // Check for 'ABCDABCD' (len 8) - Divisible by 10_001
            10_000_000..=99_999_999 => id % 10_001 != 0,
            // Check for 'ABCDEABCDE' (len 10) - Divisible by 100_001
            1_000_000_000..=9_999_999_999 => id % 100_001 != 0,

            // All other ranges are either valid or simply unhandled
            _ => true,
        }
    };

    Ok(accumulate_invalid_ids(input.trim(), filter).to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    // Part 2 checks for any repeating sequence (e.g., 111, 121212, 123123)
    let filter = |id: i64| -> bool {
        assert!(id <= 9_999_999_999, "ID {id} is too big to be handled");

        match id {
            // Len 2: AA (divisible by 11)
            10..=99 => id % 11 != 0,

            // Len 3: AAA (divisible by 111)
            100..=999 => id % 111 != 0,

            // Len 4: ABAB (divisible by 101) or AAAA (divisible by 1111)
            1_000..=9_999 => id % 101 != 0 && id % 1_111 != 0,

            // Len 5: AAAAA (divisible by 11111). No smaller repeating unit.
            10_000..=99_999 => id % 11_111 != 0,

            // Len 6: ABCABC (divisible by 1001), ABABAB (divisible by 10101), or AAAAAA (divisible by 111111)
            100_000..=999_999 => id % 1_001 != 0 && id % 10_101 != 0 && id % 111_111 != 0,

            // Len 7: AAAAAAA (divisible by 1111111). No smaller repeating unit.
            1_000_000..=9_999_999 => id % 1_111_111 != 0,

            // Len 8: ABCDABCD (10001), ABABABAB (1010101), or AAAAAAAA (11111111)
            10_000_000..=99_999_999 => {
                id % 10_001 != 0 && id % 1_010_101 != 0 && id % 11_111_111 != 0
            }

            // Len 9: ABCABCABC (1001001), or AAAAAAAAA (111111111)
            100_000_000..=999_999_999 => id % 1_001_001 != 0 && id % 111_111_111 != 0,

            // Len 10: ABCDEABCDE (100001), ABABABABAB (101010101), or AAAAAAAAAA (1111111111)
            1_000_000_000..=9_999_999_999 => {
                id % 100_001 != 0 && id % 101_010_101 != 0 && id % 1_111_111_111 != 0
            }

            // All other ranges are either valid or simply unhandled
            _ => true,
        }
    };

    Ok(accumulate_invalid_ids(input.trim(), filter).to_string())
}

/// Read an inclusive range from a string.

fn to_range(s: &str) -> RangeInclusive<i64> {
    let (start, end) = s
        .split_once("-")
        .expect("Range should have the '-' separator");

    start.parse().expect("First ID should be an integer")
        ..=end.parse().expect("Last ID should be an integer")
}

/// Finds all invalid IDs from the ranges in an input string.
/// Uses `f` as a validation function.
fn accumulate_invalid_ids<F: Fn(i64) -> bool>(input: &str, f: F) -> i64 {
    input
        .split(',')
        .map(|s| to_range(s))
        .flat_map(|range| range)
        .filter(|&id| !f(id))
        .sum::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
446443-446449,38593856-38593862,565653-565659,824824821-824824827,\
2121212118-2121212124";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 1227775554_i64.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 4174379265_i64.to_string());
        Ok(())
    }
}
