use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub trait Solution {
    fn part1(&self, input: &str) -> Result<String>;
    fn part2(&self, input: &str) -> Result<String>;
}

pub fn load_input(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("Could not load input from {path:?}"))
}

pub fn load_input_for_day(day: u8) -> Result<String> {
    const INPUT_DIR: &str = "input";
    let input_path = PathBuf::from(INPUT_DIR).join(format!("day{day}"));
    load_input(&input_path)
}

pub fn get_solution(day: u8) -> Result<Box<dyn Solution>> {
    match day {
        _ => anyhow::bail!("Solution for day {} not available.", day),
    }
}
