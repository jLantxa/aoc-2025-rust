use crate::aoc::DailySolutions;

pub(crate) mod day01;
pub(crate) mod day02;
pub(crate) mod day03;
pub(crate) mod day04;
pub(crate) mod day05;
pub(crate) mod day06;
pub(crate) mod day07;
pub(crate) mod day08;
pub(crate) mod day09;
pub(crate) mod day10;
pub(crate) mod day11;

pub fn get_solution(day: u8) -> Option<DailySolutions> {
    match day {
        1 => Some((day01::part1, day01::part2)),
        2 => Some((day02::part1, day02::part2)),
        3 => Some((day03::part1, day03::part2)),
        4 => Some((day04::part1, day04::part2)),
        5 => Some((day05::part1, day05::part2)),
        6 => Some((day06::part1, day06::part2)),
        7 => Some((day07::part1, day07::part2)),
        8 => Some((day08::part1, day08::part2)),
        9 => Some((day09::part1, day09::part2)),
        10 => Some((day10::part1, day10::part2)),
        11 => Some((day11::part1, day11::part2)),
        _ => None,
    }
}
