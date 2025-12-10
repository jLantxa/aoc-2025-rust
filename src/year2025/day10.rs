use std::{io::Write, process::Command};

use anyhow::{Context, Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    Ok(parse_machines(input)?
        .into_iter()
        .map(|machine| machine.optimize_button_presses_p1())
        .sum::<usize>()
        .to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    Ok(parse_machines(input)?
        .into_iter()
        .map(|machine| machine.optimize_button_presses_p2())
        .sum::<usize>()
        .to_string())
}

type IndicatorLights = u32;
type Joltage = u16;

#[derive(Debug)]
struct Machine {
    lights: IndicatorLights, // One bit per light. b0 = '1' -> light 0 ON
    num_lights: usize,
    buttons: Vec<IndicatorLights>, // For each button, the lights (bits) that it toggles
    joltages: Vec<Joltage>,
}

impl Machine {
    /// Create a new machine from its single-line textual description.
    fn new_from_str(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace();

        // Indicator lights
        let light_part = parts.next().context("Missing light diagram")?;

        let light_diagram = light_part.trim_matches(|c| c == '[' || c == ']');
        let num_lights = light_diagram.len();
        let mut target_lights: IndicatorLights = 0;

        for (i, c) in light_diagram.chars().enumerate() {
            match c {
                '#' => target_lights |= 1 << i,
                '.' => {}
                _ => bail!("Invalid character in light diagram: {}", c),
            }
        }

        // Buttons and Joltages
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();

        for part in parts {
            if let Some(content) = part.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
                let mut button_mask: IndicatorLights = 0;

                for index_str in content.split(',') {
                    if index_str.is_empty() {
                        continue;
                    }
                    let index: usize =
                        index_str.parse().context("Invalid light index in button")?;

                    if index >= num_lights {
                        bail!(
                            "Light index {} out of bounds for {} lights",
                            index,
                            num_lights
                        );
                    }
                    button_mask |= 1 << index;
                }
                buttons.push(button_mask);
            } else if let Some(content) = part.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                for joltage_str in content.split(',') {
                    if joltage_str.is_empty() {
                        continue;
                    }
                    let joltage: Joltage = joltage_str.parse().context("Invalid joltage value")?;
                    joltages.push(joltage);
                }
            } else {
                bail!("Unexpected section format: {}", part);
            }
        }

        if buttons.is_empty() {
            bail!("No buttons found in machine line");
        }

        Ok(Machine {
            lights: target_lights,
            num_lights,
            buttons,
            joltages,
        })
    }

    /// Optimize button presses for part 1.
    ///
    /// We need to find the minimum number of button presses that toggle the lights
    /// to a particular state.
    ///
    /// This is the linear equation system `l=Bs` over the Galois Field (GF), where:
    /// - `l` (vector) is the lights toggle status.
    /// - `B` (matrix) contains column vectors for each button and the lights they toggle.
    /// - `s` (vector) represents which buttons are pressed.
    ///
    /// In the GF, addition is performed by `XOR` and multiplication by `AND`.
    ///
    /// Since the system is indetermined, there are many `s` values that are solution.
    /// We need to find the one that minimizes the number of button presses, i.e.,
    /// find the `s` with the minimum L0 norm (number of ones).
    ///
    /// Since the search space is reduced (no more than 16 buttons), we can find
    /// a solution to the equation system by brute force.
    /// I test all posible `s` values, filter the `s` that are solutions and take
    /// the `s` with the lowest L0 norm.
    /// This is be fast enough for this part because the search space is so low.
    fn optimize_button_presses_p1(&self) -> usize {
        let num_buttons = self.buttons.len();
        let max_combinations: IndicatorLights = 1 << num_buttons;

        (0..max_combinations)
            .map(|i| {
                let lights = self
                    .buttons
                    .iter()
                    .enumerate()
                    .filter_map(|(j, button_mask)| {
                        if (i >> j) & 1 == 1 {
                            Some(button_mask)
                        } else {
                            None
                        }
                    })
                    .fold(0 as IndicatorLights, |acc, mask| acc ^ mask);

                let l0 = i.count_ones() as usize;

                (l0, lights)
            })
            .filter(|&(_l0, lights)| lights == self.lights)
            .map(|(l0, _lights)| l0)
            .min()
            .expect("A minimum solution must exist.")
    }

    /// Optimize button presses for part 2.
    ///
    /// Similarly to part 1, this is another linear equation system, but no longer
    /// in the Galois Field. we need to solve `j=Bs` by finding a solution `s` where:
    /// - `j` (vector) is the joltage counter.
    /// - `B` (matrix) contains column vectors for each button and the value added to each counter
    ///   (always one).
    /// - `s` (vector) represents how many times the buttons are pressed.
    ///
    /// The problem now is that we need to find a solution `s` with non-negative
    /// integers and the search space is more indetermined. It is bounded for sure, but much
    /// bigger. There is also no closed solution that I know, since B is not necessarily invertible
    /// (B is not always a square matrix).
    ///
    /// Tried BFS but it was too slow (no solution after 10 minutes).
    ///
    /// Use Z3 by an external Python script.
    fn optimize_button_presses_p2(&self) -> usize {
        const SCRIPT_PATH: &str = "./src/year2025/d10p2_solver.py";

        // Helper struct for serializing data to Python
        #[derive(Debug, serde::Serialize)]
        struct MachineData<'a> {
            buttons: &'a [IndicatorLights],
            joltages: &'a [u16],
            num_counters: usize,
            num_buttons: usize,
        }

        let data = MachineData {
            buttons: &self.buttons,
            joltages: &self.joltages,
            num_counters: self.joltages.len(),
            num_buttons: self.buttons.len(),
        };

        let input_json = serde_json::to_string(&data).expect("Failed to serialize machine data");

        let mut child = Command::new("python")
            .arg(SCRIPT_PATH)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to execute Python script. Ensure 'python' is in your PATH and solve_ilp.py exists.");

        {
            let stdin = child
                .stdin
                .as_mut()
                .expect("Failed to open stdin for Python");
            stdin
                .write_all(input_json.as_bytes())
                .expect("Failed to write to stdin");
        }

        let output = child
            .wait_with_output()
            .expect("Failed to read output from Python script");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("Python script failed with error:\n{}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        stdout
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Python script returned non-integer output: '{}'", stdout))
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lights ({}): {:0width$b} | Buttons: {:?} | Joltages: {:?}",
            self.num_lights,
            self.lights,
            self.buttons,
            self.joltages,
            width = self.num_lights
        )
    }
}

fn parse_machines(input: &str) -> Result<Vec<Machine>> {
    Ok(input
        .trim()
        .lines()
        .map(|line| Machine::new_from_str(line).expect("Should be a valid machine"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 7.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 33.to_string());
        Ok(())
    }
}
