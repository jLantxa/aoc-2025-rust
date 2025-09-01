use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};

mod aoc;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("selection")
        .required(true)
        .args(&["all", "day"]),
))]
struct Args {
    #[clap(short, long, help = "Run all solutions")]
    all: bool,

    #[clap(short, long, help = "The day to run (1-25)")]
    day: Option<u8>,

    #[clap(short, long, help = "The part to run (1 or 2)")]
    part: Option<u8>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.all {
        run_all_solutions();
    } else if let Some(day) = args.day {
        run_single_solution(day, args.part)?;
    }

    Ok(())
}

fn run_all_solutions() {
    for day in 1..=25 {
        let solution = match aoc::get_solution(day) {
            Ok(sol) => sol,
            Err(_) => {
                continue;
            }
        };

        let input = match aoc::load_input_for_day(day) {
            Ok(inp) => inp,
            Err(e) => {
                eprintln!("Error loading input for day {day}: {e}. Skipping...");
                continue;
            }
        };

        println!("\n--- Day {day} ---");
        match solution.part1(&input) {
            Ok(res) => println!("Part 1: {res}"),
            Err(e) => eprintln!("Error running Day {day} Part 1: {e}"),
        }
        match solution.part2(&input) {
            Ok(res) => println!("Part 2: {res}"),
            Err(e) => eprintln!("Error running Day {day} Part 2: {e}"),
        }
        println!();
    }
}

fn run_single_solution(day: u8, part: Option<u8>) -> Result<()> {
    let solution =
        aoc::get_solution(day).with_context(|| format!("Solution for day {day} not found"))?;
    let input =
        aoc::load_input_for_day(day).with_context(|| format!("Input for day {day} not found"))?;

    println!("\n--- Day {day} ---");
    if part.is_none() || part == Some(1) {
        let result = solution.part1(&input)?;
        println!("Part 1: {result}");
    }
    if part.is_none() || part == Some(2) {
        let result = solution.part2(&input)?;
        println!("Part 2: {result}");
    }

    Ok(())
}
