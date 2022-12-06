use super::*;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
enum State {
    Stack,
    Instructions,
}

#[derive(Debug, Clone)]
struct Instruction {
    amount: i32,
    start: i32,
    end: i32,
}

#[derive(Debug, Clone)]
struct Cargo {
    stacks: Vec<VecDeque<char>>,
    instructions: Vec<Instruction>,
}

fn parse(input: &str) -> Result<Cargo> {
    let mut stacks = Vec::new();
    let mut instructions = Vec::new();
    let instruction_regex = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#)?;

    let mut state = State::Stack;
    for line in input.lines() {
        match state {
            State::Stack => {
                let row = line.chars().collect::<Vec<_>>();
                row.chunks(4).enumerate().for_each(|(column, value)| {
                    if stacks.len() < column + 1 {
                        stacks.push(VecDeque::new())
                    }
                    if value[1].is_alphabetic() {
                        stacks[column].push_front(value[1]);
                    } else if value[1].is_numeric() {
                        state = State::Instructions;
                    }
                });
            }
            State::Instructions => {
                if line.is_empty() {
                    continue;
                }
                let caps = instruction_regex.captures(line).ok_or_else(|| eyre!("Failed to find any captures"))?;
                instructions.push(Instruction {
                    amount: caps.get(1).ok_or_else(|| eyre!("Missing amount capture"))?.as_str().parse()?,
                    start: caps.get(2).ok_or_else(|| eyre!("Missing amount capture"))?.as_str().parse()?,
                    end: caps.get(3).ok_or_else(|| eyre!("Missing amount capture"))?.as_str().parse()?,
                });
            }
        }
    }
    Ok(Cargo { instructions, stacks })
}

fn print_stacks(cargo: &Cargo) -> String {
    let mut rtn = String::new();
    for stack in cargo.stacks.iter() {
        if let Some(back) = stack.back() {
            rtn.push(*back);
        }
    }
    rtn
}

fn part1(mut cargo: Cargo) -> Result<String> {
    for instruction in &cargo.instructions {
        for _ in 0..instruction.amount {
            let item = cargo.stacks[instruction.start as usize - 1]
                .pop_back()
                .ok_or_else(|| eyre!("Unexpected end of stack"))?;
            cargo.stacks[instruction.end as usize - 1].push_back(item);
        }
    }

    Ok(print_stacks(&cargo))
}

fn part2(mut cargo: Cargo) -> Result<String> {
    for instruction in &cargo.instructions {
        let items = (0..instruction.amount)
            .map(|_| {
                cargo.stacks[instruction.start as usize - 1]
                    .pop_back()
                    .ok_or_else(|| eyre!("Unexpected end of stack"))
            })
            .collect::<Result<Vec<_>>>()?;
        for item in items.into_iter().rev() {
            cargo.stacks[instruction.end as usize - 1].push_back(item);
        }
    }

    Ok(print_stacks(&cargo))
}

pub fn solve(input: &str) -> Result<Solution> {
    let cargo = parse(input)?;
    Ok(Solution {
        first: part1(cargo.clone())?,
        second: part2(cargo)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let cargo = parse(input).unwrap();
        assert_eq!(part1(cargo.clone()).unwrap(), "CMZ");
        assert_eq!(part2(cargo).unwrap(), "MCD");
    }
}
