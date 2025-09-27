use anyhow::Result;
use clap::Parser;

mod aoc;
mod year2022;
mod year2025;

mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help = "Year")]
    year: String,

    #[clap(short, long, help = "The day to run (1-25)")]
    day: Option<u8>,

    #[clap(short, long, help = "The part to run (1 or 2)")]
    part: Option<u8>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(day) = args.day {
        aoc::run_single_solution(&args.year, day, args.part)?;
    } else {
        aoc::run_all_yearly_solutions(&args.year);
    }

    Ok(())
}
