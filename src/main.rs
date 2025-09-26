use anyhow::Result;
use clap::{ArgGroup, Parser};

mod aoc;
mod year2022;
mod year2025;

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

    #[clap(short, long, help = "Year")]
    year: String,

    #[clap(short, long, help = "The day to run (1-25)")]
    day: Option<u8>,

    #[clap(short, long, help = "The part to run (1 or 2)")]
    part: Option<u8>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.all {
        aoc::run_all_yearly_solutions(&args.year);
    } else if let Some(day) = args.day {
        aoc::run_single_solution(&args.year, day, args.part)?;
    }

    Ok(())
}
