use anyhow::Result;

pub(crate) fn part1(input: &str) -> Result<String> {
    let index = detect_marker::<4>(input).expect("Expected a solution");
    Ok(index.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let index = detect_marker::<14>(input).expect("Expected a solution");
    Ok(index.to_string())
}

fn detect_marker<const MARKER_LEN: usize>(input: &str) -> Option<usize> {
    let input_len = input.len();
    if input_len < MARKER_LEN {
        return None;
    }

    for i in 0..input_len - MARKER_LEN {
        let mut substr: Vec<char> = input[i..i + MARKER_LEN].chars().collect();
        substr.sort_unstable();
        substr.dedup();

        if substr.len() == MARKER_LEN {
            return Some(i + MARKER_LEN);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_part1(#[case] input: &str, #[case] expected: usize) -> Result<()> {
        let solution = part1(input)?;
        assert_eq!(solution, expected.to_string());

        Ok(())
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_part2(#[case] input: &str, #[case] expected: usize) -> Result<()> {
        let solution = part2(input)?;
        assert_eq!(solution, expected.to_string());

        Ok(())
    }
}
