use crate::aoc::DailySolutions;

mod day01;

pub fn get_solution(day: u8) -> Option<DailySolutions> {
    match day {
        1 => Some((day01::part1, day01::part2)),
        _ => None,
    }
}
