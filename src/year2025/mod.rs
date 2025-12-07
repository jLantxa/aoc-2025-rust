use crate::aoc::DailySolutions;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub fn get_solution(day: u8) -> Option<DailySolutions> {
    match day {
        1 => Some((day01::part1, day01::part2)),
        2 => Some((day02::part1, day02::part2)),
        3 => Some((day03::part1, day03::part2)),
        4 => Some((day04::part1, day04::part2)),
        5 => Some((day05::part1, day05::part2)),
        6 => Some((day06::part1, day06::part2)),
        7 => Some((day07::part1, day07::part2)),
        _ => None,
    }
}
