use std::cmp::Ordering;

use anyhow::Result;

pub(crate) fn part1(input: &str) -> Result<String> {
    let (sorted_ranges, ids) = parse_input(input);
    Ok(ids
        .into_iter()
        .filter(|&id| ranges_contain(&sorted_ranges, id))
        .count()
        .to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let (sorted_ranges, _) = parse_input(input);
    let num_fresh_items = sorted_ranges.len()
        + sorted_ranges
            .into_iter()
            .map(|range| (range.1 - range.0) as usize)
            .sum::<usize>();
    Ok(num_fresh_items.to_string())
}

type ID = u64;

fn parse_input(input: &str) -> (Vec<(ID, ID)>, Vec<ID>) {
    let (ranges_str, ids_str) = input
        .split_once("\n\n")
        .expect("Input should be separated by an empty line");

    let mut ranges: Vec<(ID, ID)> = ranges_str
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            line.split_once("-").map(|(a, b)| {
                (
                    a.parse().expect("Should be an integer"),
                    b.parse().expect("Should be an integer"),
                )
            })
        })
        .collect();

    let ids: Vec<ID> = ids_str
        .lines()
        .filter_map(|line| line.trim().parse::<ID>().ok())
        .collect();

    merge_ranges(&mut ranges);

    (ranges, ids)
}

fn merge_ranges(ranges: &mut Vec<(ID, ID)>) {
    ranges.sort_unstable_by_key(|(start, _end)| *start);

    let mut i = 0; // Index of the last merged range
    let mut j = 1; // Index of the current range being processed

    while j < ranges.len() {
        let (current_start, current_end) = ranges[j];
        let (_merged_start, merged_end) = ranges[i];

        if current_start <= merged_end {
            ranges[i].1 = merged_end.max(current_end);
        } else {
            i += 1;
            ranges[i] = ranges[j];
        }

        j += 1;
    }

    ranges.truncate(i + 1);
}

fn ranges_contain(sorted_ranges: &[(ID, ID)], id: ID) -> bool {
    match sorted_ranges.binary_search_by(|&(start, _end)| {
        if start > id {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }) {
        Ok(idx) | Err(idx) => {
            if idx > 0 {
                let (_start, end) = sorted_ranges[idx - 1];
                id <= end
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 3.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 14.to_string());
        Ok(())
    }
}
