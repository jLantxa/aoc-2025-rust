use anyhow::{Result, bail};

const SAFE_MAX: i32 = 100;
const SAFE_INIT_DIAL: i32 = 50;

pub(crate) fn part1(input: &str) -> Result<String> {
    let mut safe = Safe::new(SAFE_MAX, SAFE_INIT_DIAL);
    let mut count_zeros = 0;

    for line in input.lines() {
        let (dial, _zeros) = safe.rotate(line)?;
        if dial == 0 {
            count_zeros += 1;
        }
    }

    Ok(count_zeros.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let mut safe = Safe::new(SAFE_MAX, SAFE_INIT_DIAL);
    let mut count_zeros = 0;

    for line in input.lines() {
        let (_dial, zeros) = safe.rotate(line)?;
        count_zeros += zeros;
    }

    Ok(count_zeros.to_string())
}

struct Safe {
    max: i32,
    dial: i32,
}

impl Safe {
    pub fn new(max: i32, dial: i32) -> Self {
        Self { max, dial }
    }

    /// Rotate the safe
    ///
    /// This function takes a rotation instruction like
    /// L50, R18, etc.
    ///
    /// Returns a tuple (dial, zero_crossings)
    pub fn rotate(&mut self, rotation: &str) -> Result<(i32, i32)> {
        let val: i32 = rotation[1..]
            .parse()
            .expect("Rotation value should be an integer");

        let unwrapped_dial = match rotation.chars().nth(0).expect("Should have rotation") {
            'L' => self.dial - val,
            'R' => self.dial + val,
            _ => bail!("Invalid rotation"),
        };

        let zeros = num_zero_crossings(self.dial, unwrapped_dial, self.max);

        // Calculate a non negative modulo (negative rotations also wrap around)
        // This is the same as ((a % n) + n) % n
        self.dial = unwrapped_dial.rem_euclid(self.max);

        Ok((self.dial, zeros))
    }
}

/// Calculate the number of times the dial crosses the '0' with a rotation
fn num_zero_crossings(dial: i32, unwrapped: i32, n: i32) -> i32 {
    let mut zeros = unwrapped.abs() / n;

    if (unwrapped < 0 && dial > 0) || (unwrapped == 0 && unwrapped < dial) {
        zeros += 1;
    }

    zeros
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 3.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 6.to_string());
        Ok(())
    }

    #[test]
    fn test_wrap() {
        assert_eq!(num_zero_crossings(5, 25, 100), 0);
        assert_eq!(num_zero_crossings(5, 100, 100), 1);
        assert_eq!(num_zero_crossings(5, 175, 100), 1);
        assert_eq!(num_zero_crossings(5, 200, 100), 2);
        assert_eq!(num_zero_crossings(5, 250, 100), 2);
        assert_eq!(num_zero_crossings(0, 100, 100), 1);
        assert_eq!(num_zero_crossings(0, 101, 100), 1);

        assert_eq!(num_zero_crossings(0, -1, 100), 0);
        assert_eq!(num_zero_crossings(5, -1, 100), 1);
        assert_eq!(num_zero_crossings(5, -88, 100), 1);
        assert_eq!(num_zero_crossings(5, -100, 100), 2);
        assert_eq!(num_zero_crossings(5, -112, 100), 2);
        assert_eq!(num_zero_crossings(0, -200, 100), 2);
        assert_eq!(num_zero_crossings(5, -200, 100), 3);
        assert_eq!(num_zero_crossings(5, -250, 100), 3);
    }
}
