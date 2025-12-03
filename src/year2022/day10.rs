use anyhow::{Result, bail};

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from(line: &str) -> Result<Instruction> {
        let args: Vec<&str> = line.split_whitespace().collect();

        let instruction = match args[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(args[1].parse::<i32>().expect("Value should be i32")),
            _ => bail!("Invalid instruction {line}"),
        };

        Ok(instruction)
    }
}

struct Cpu {
    x: Vec<i32>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self { x: vec![1] }
    }
}

impl Cpu {
    fn instr(&mut self, instr: &Instruction) {
        let last_x = *self.x.last().expect("X should have a value");

        match instr {
            Instruction::Noop => {
                self.x.push(last_x);
            }
            Instruction::Addx(val) => {
                self.x.push(last_x);
                self.x.push(last_x + val);
            }
        }
    }

    #[allow(dead_code)]
    fn get_x_at_cycle(&self, cycle: usize) -> Option<i32> {
        self.x.get(cycle).copied()
    }

    #[allow(dead_code)]
    fn cycles(&self) -> usize {
        self.x.len()
    }

    #[allow(dead_code)]
    fn x(&self) -> i32 {
        *self.x.last().expect("X should have a value")
    }

    fn all_x(&self) -> &Vec<i32> {
        self.x.as_ref()
    }
}

fn draw_screen(x: &[i32]) -> String {
    let mut screen = String::new();

    const SCREEN_WIDTH: usize = 40;
    const SCREEN_HEIGHT: usize = 6;

    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {
            let cycle = SCREEN_WIDTH * j + i;
            let val = x[cycle];

            if (i as i32 - val).abs() <= 1 {
                screen.push('#');
            } else {
                screen.push('.');
            }
        }

        if j < (SCREEN_HEIGHT - 1) {
            screen.push('\n');
        }
    }

    screen
}

pub(crate) fn part1(input: &str) -> Result<String> {
    let mut cpu = Cpu::default();

    for line in input.lines() {
        let instr = Instruction::from(line)?;
        cpu.instr(&instr);
    }

    let mut power = 0;
    for c in [20, 60, 100, 140, 180, 220] {
        power += (c as i32)
            * cpu
                .get_x_at_cycle(c - 1) // First cycle has index 0
                .expect("Value at cycle {c} should exist");
    }

    Ok(power.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let mut cpu = Cpu::default();

    for line in input.lines() {
        let instr = Instruction::from(line)?;
        cpu.instr(&instr);
    }

    let screen = draw_screen(cpu.all_x());
    Ok(format!("\n{screen}"))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_cpu() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "13140");

        Ok(())
    }
}
