use super::Solution;
use color_eyre::eyre::{bail, eyre};
use color_eyre::{Report, Result};

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Action {
    Lose,
    Draw,
    Win,
}

impl TryFrom<&str> for Action {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self> {
        Ok(match value {
            "X" => Action::Lose,
            "Y" => Action::Draw,
            "Z" => Action::Win,
            _ => bail!("Unexpected character"),
        })
    }
}

impl Move {
    fn value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn from_action(action: &Action, opponent: &Self) -> Self {
        match (action, opponent) {
            (Action::Lose, Move::Rock) => Move::Scissors,
            (Action::Lose, Move::Paper) => Move::Rock,
            (Action::Lose, Move::Scissors) => Move::Paper,
            (Action::Win, Move::Rock) => Move::Paper,
            (Action::Win, Move::Paper) => Move::Scissors,
            (Action::Win, Move::Scissors) => Move::Rock,
            (Action::Draw, _) => *opponent,
        }
    }

    fn round_score(&self, other: &Self) -> i32 {
        match (self, other) {
            (Move::Rock, Move::Rock) => 3,
            (Move::Rock, Move::Paper) => 0,
            (Move::Rock, Move::Scissors) => 6,
            (Move::Paper, Move::Paper) => 3,
            (Move::Paper, Move::Rock) => 6,
            (Move::Paper, Move::Scissors) => 0,
            (Move::Scissors, Move::Scissors) => 3,
            (Move::Scissors, Move::Paper) => 6,
            (Move::Scissors, Move::Rock) => 0,
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self> {
        Ok(match value {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => bail!("Unknown character"),
        })
    }
}

#[derive(Debug)]
struct Turn {
    opponent: Move,
    me: Move,
    action: Action,
}

fn part1(turns: &Vec<Turn>) -> i32 {
    turns.iter().fold(0, |acc, turn| acc + turn.me.value() + turn.me.round_score(&turn.opponent))
}

fn part2(turns: &Vec<Turn>) -> i32 {
    turns.iter().fold(0, |acc, turn| {
        let my_move = Move::from_action(&turn.action, &turn.opponent);
        acc + my_move.value() + my_move.round_score(&turn.opponent)
    })
}

fn parse(input: &str) -> Result<Vec<Turn>> {
    input
        .lines()
        .map(|line| {
            let mut turn = line.split(' ');
            let opponent = turn.next().ok_or_else(|| eyre!("Expected character"))?;
            let me = turn.next().ok_or_else(|| eyre!("Expected character"))?;
            Ok(Turn {
                opponent: Move::try_from(opponent)?,
                me: Move::try_from(me)?,
                action: Action::try_from(me)?,
            })
        })
        .collect::<Result<Vec<_>>>()
}

pub fn solve(input: &str) -> Result<Solution> {
    let turns = parse(input)?;
    Ok(Solution {
        first: format!("{}", part1(&turns)),
        second: format!("{}", part2(&turns)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = r#"A Y
B X
C Z"#;
        let turns = parse(input).unwrap();
        assert_eq!(part1(&turns), 15);
        assert_eq!(part2(&turns), 12);
    }
}
