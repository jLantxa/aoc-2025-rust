use crate::aoc::DailySolutions;

pub(crate) mod day1;
pub(crate) mod day2;
pub(crate) mod day3;

pub fn get_solution(day: u8) -> Option<DailySolutions> {
    match day {
        1 => Some((day1::part1, day1::part2)),
        2 => Some((day2::part1, day2::part2)),
        3 => Some((day3::part1, day3::part2)),
        _ => None,
    }
}
