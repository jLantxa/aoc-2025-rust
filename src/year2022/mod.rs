use crate::aoc::DailySolutions;

pub(crate) mod day1;

pub fn get_solution(day: u8) -> Option<DailySolutions> {
    match day {
        1 => Some((day1::part1, day1::part2)),
        _ => None,
    }
}
