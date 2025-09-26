use crate::aoc::DailySolutions;

pub(crate) mod day1;
pub(crate) mod day2;
pub(crate) mod day3;
pub(crate) mod day4;
pub(crate) mod day5;
pub(crate) mod day6;

pub fn get_solution(day: u8) -> Option<DailySolutions> {
    match day {
        1 => Some((day1::part1, day1::part2)),
        2 => Some((day2::part1, day2::part2)),
        3 => Some((day3::part1, day3::part2)),
        4 => Some((day4::part1, day4::part2)),
        5 => Some((day5::part1, day5::part2)),
        6 => Some((day6::part1, day6::part2)),
        _ => None,
    }
}
