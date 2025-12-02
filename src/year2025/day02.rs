//! This implementation uses a brute force approach. There is a faster way
//! to solve this using some modulo magic to check for repeating pattenrs.
//! However, for the size of the input and number of IDs to check,the brute
//! force implementation is still good.

use std::ops::RangeInclusive;

use anyhow::Result;

pub(crate) fn part1(input: &str) -> Result<String> {
    Ok(accumulate_invalid_ids(input, process_id_repeating_halves).to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    Ok(accumulate_invalid_ids(input, process_id_repeating_any).to_string())
}

/// Returns false if an ID has a repeating sequence of numbers (twice).
/// 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
///
/// This is a validation function, so it returns true if the ID is valid and false otherwise.
fn process_id_repeating_halves(id: i64) -> bool {
    let text = id.to_string();
    let len = text.len();

    if !len.is_multiple_of(2) {
        // If the length is uneven, it cannot have two equal halves and this is a valid ID.
        return true;
    }

    text[0..len / 2] != text[len / 2..]
}

/// Returns false if an ID has a repeating sequence of numbers (any number of times).
/// 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times),
/// and 1111111 (1 seven times) are all invalid IDs.
///
/// This is a validation function, so it returns true if the ID is valid and false otherwise.
fn process_id_repeating_any(id: i64) -> bool {
    let id_text = id.to_string();
    let len = id_text.len();

    let is_repeating = |n: usize| -> bool {
        if !len.is_multiple_of(n) {
            // The ID cannot have n repeating numbers if len is not a multilple of n.
            return false;
        };

        // Compare with all other substrings
        let substr = &id_text[0..n];
        for i in 1..(len / n) {
            if &id_text[i * n..i * n + n] != substr {
                return false;
            }
        }

        true
    };

    // Check for substrings of length 1 and increasing.
    for n in 1..=len / 2 {
        if is_repeating(n) {
            return false;
        }
    }

    true
}

/// Process a range of IDs using `f` as a validation function.
fn process_range<F: Fn(i64) -> bool>(range: RangeInclusive<i64>, f: F) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    for id in range {
        if !f(id) {
            invalid_ids.push(id);
        }
    }

    invalid_ids
}

/// Read an inclusive range from a string.
fn read_range(s: &str) -> RangeInclusive<i64> {
    let vals: Vec<&str> = s.trim().split("-").collect();
    assert_eq!(vals.len(), 2);

    vals.first()
        .expect("Should have first ID")
        .parse()
        .expect("First ID should be an integer")
        ..=vals
            .last()
            .expect("Should have last ID")
            .parse()
            .expect("Last ID should be an integer")
}

/// Finds all invalid IDs from the ranges in an input string.
/// Uses `f` as a validation function.
fn accumulate_invalid_ids<F: Fn(i64) -> bool>(input: &str, f: F) -> i64 {
    let mut acc: i64 = 0;
    for s in input.split(",") {
        let range = read_range(s);
        let invalid_ids = process_range(range, &f);
        for id in invalid_ids {
            acc += id;
        }
    }

    acc
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
