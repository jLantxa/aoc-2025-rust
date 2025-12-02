use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::{year2022, year2025};

pub(crate) type SolutionFn = fn(&str) -> Result<String>;
pub(crate) type DailySolutions = (SolutionFn, SolutionFn);

fn load_input(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("Could not load input from {path:?}"))
}

fn load_input_for_day(year: &str, day: u8) -> Result<String> {
    const INPUT_DIR: &str = "input";
    let input_path = PathBuf::from(INPUT_DIR)
        .join(year)
        .join(format!("day{day:02}.txt"));
    load_input(&input_path)
}

fn get_solution(year: &str, day: u8) -> Option<DailySolutions> {
    match year {
        "2022" => year2022::get_solution(day),
        "2025" => year2025::get_solution(day),
        _ => None,
    }
}

/// Executes a solution part, times it, and prints the result.
fn run_solution_result(part: u8, func: impl FnOnce() -> Result<String>) -> Duration {
    let start = Instant::now();
    let result = func();
    let elapsed = start.elapsed();

    match result {
        Ok(res) => println!(
            "{} {} {res}",
            format!("Part {part}:").bold().yellow(),
            format!("({elapsed:?})").dimmed().italic()
        ),
        Err(e) => {
            println!(
                "{} ({elapsed:?}) Error: {e}",
                format!("Part {part}:").bold().yellow(),
            );
        }
    }

    elapsed
}

pub(crate) fn run_all_yearly_solutions(year: &str) {
    let mut elapsed = Duration::ZERO;

    for day in 1..=25 {
        let solutions = match get_solution(year, day) {
            Some(sols) => sols,
            None => continue, // Skip if no solution for the day
        };

        let input = match load_input_for_day(year, day) {
            Ok(inp) => inp,
            Err(e) => {
                eprintln!("Error loading input for day {day}: {e}. Skipping...");
                continue; // Skip if input can't be loaded
            }
        };

        println!("{}", format!("-- {year} :: day {day} --").bold());
        elapsed += run_solution_result(1, || (solutions.0)(&input)); // Part 1
        elapsed += run_solution_result(2, || (solutions.1)(&input)); // Part 2
        println!();
    }

    println!("Total time: {}", format!("{elapsed:?}").bold().cyan());
}

pub(crate) fn run_single_solution(year: &str, day: u8, part: Option<u8>) -> Result<()> {
    let solution =
        get_solution(year, day).with_context(|| format!("Solution for day {day} not found"))?;
    let input =
        load_input_for_day(year, day).with_context(|| format!("Input for day {day} not found"))?;

    println!("{}", format!("-- {year} :: day {day} --").bold());

    if part.is_none() || part == Some(1) {
        let _ = run_solution_result(1, || (solution.0)(&input));
    }

    if part.is_none() || part == Some(2) {
        let _ = run_solution_result(2, || (solution.1)(&input));
    }

    Ok(())
}
