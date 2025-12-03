use anyhow::{Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    let score = simulate_strategy_guide_1(input)?;

    Ok(score.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    let score = simulate_strategy_guide_2(input)?;

    Ok(score.to_string())
}

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn move_from_code(code: &str) -> Result<Move> {
    match code {
        "A" => Ok(Move::Rock),
        "X" => Ok(Move::Rock),
        "B" => Ok(Move::Paper),
        "Y" => Ok(Move::Paper),
        "C" => Ok(Move::Scissors),
        "Z" => Ok(Move::Scissors),
        _ => bail!("Unknown move {}", code),
    }
}

fn outcome_from_code(code: &str) -> Result<Outcome> {
    match code {
        "X" => Ok(Outcome::Lose),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Win),
        _ => bail!("Unknown outcome {}", code),
    }
}

fn move_score(m: &Move) -> u64 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn simulate_round(me: &Move, op: &Move) -> u64 {
    let outcome_score = match (me, op) {
        (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper)
        | (Move::Scissors, Move::Scissors) => 3,

        (Move::Rock, Move::Scissors)
        | (Move::Paper, Move::Rock)
        | (Move::Scissors, Move::Paper) => 6,

        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => 0,
    };

    outcome_score + move_score(me)
}

fn move_from_outcome(op: &Move, outcome: &Outcome) -> Move {
    match (op, outcome) {
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Draw) => Move::Rock,
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Draw) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Draw) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
    }
}

fn simulate_strategy_guide_1(input: &str) -> Result<u64> {
    let mut total_score = 0;

    for line in input.lines() {
        let mut codes = line.split_whitespace();

        let (op_code, my_code) = match (codes.next(), codes.next(), codes.next()) {
            (Some(op), Some(me), None) => (op, me),
            _ => bail!("Invalid line format: {}", line),
        };

        let op_move = move_from_code(op_code)?;
        let me_move = move_from_code(my_code)?;

        let round_score = simulate_round(&me_move, &op_move);
        total_score += round_score;
    }

    Ok(total_score)
}

fn simulate_strategy_guide_2(input: &str) -> Result<u64> {
    let mut total_score = 0;

    for line in input.lines() {
        let mut codes = line.split_whitespace();

        let (op_code, outcome_code) = match (codes.next(), codes.next(), codes.next()) {
            (Some(op), Some(me), None) => (op, me),
            _ => bail!("Invalid line format: {}", line),
        };

        let op_move = move_from_code(op_code)?;
        let outcome = outcome_from_code(outcome_code)?;
        let me_move = move_from_outcome(&op_move, &outcome);

        let round_score = simulate_round(&me_move, &op_move);
        total_score += round_score;
    }

    Ok(total_score)
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "15");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "12");

        Ok(())
    }
}
