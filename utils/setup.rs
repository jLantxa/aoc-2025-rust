use std::path::PathBuf;

use clap::Parser;

const TEXT: &str = r#"
use anyhow::{Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    bail!("Unimplemented");
}

pub(crate) fn part2(input: &str) -> Result<String> {
    bail!("Unimplemented");
}

#[cfg(test)]
mod tests {
    use super::*;
}
"#;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    year: String,

    #[clap(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    let year = format!("year{:02}", args.year);
    let day = format!("day{:02}", args.day);

    let src = PathBuf::from("src");
    let year_dir = src.join(year);
    let file_path = year_dir.join(day).with_extension("rs");

    if file_path.exists() {
        eprintln!("Error: File {file_path:?} exists already");
    } else {
        let _ = std::fs::create_dir_all(year_dir);

        match std::fs::write(&file_path, TEXT) {
            Ok(_) => println!("Created file {file_path:?}"),
            Err(e) => eprintln!("Error: Could not create file {file_path:?}: {e:?}"),
        }
    }
}
