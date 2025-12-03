use crate::aoc::DailySolutions;

mod day01;
mod day02;
mod day03;

pub fn get_solution(day: u8) -> Option<DailySolutions> {
    match day {
        1 => Some((day01::part1, day01::part2)),
        2 => Some((day02::part1, day02::part2)),
        3 => Some((day03::part1, day03::part2)),

        _ => None,
    }
}
