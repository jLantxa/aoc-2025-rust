use crate::aoc::DailySolutions;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
// mod day10;
// mod day11;
// mod day12;

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
        // 10 => Some((day10::part1, day10::part2)),
        // 11 => Some((day11::part1, day11::part2)),
        // 12 => Some((day12::part1, day12::part2)),
        _ => None,
    }
}
