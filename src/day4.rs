use super::*;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Assignment {
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>,
}

fn parse_range(input: &str) -> Result<RangeInclusive<i32>> {
    let mut range = input.split('-');
    let start = range.next().ok_or_else(|| eyre!("Missing start of range"))?;
    let end = range.next().ok_or_else(|| eyre!("Missing end of range"))?;
    if range.next().is_some() {
        bail!("Unexpected additional input");
    }

    Ok(RangeInclusive::new(start.parse()?, end.parse()?))
}

fn parse(input: &str) -> Result<Vec<Assignment>> {
    input
        .lines()
        .map(|line| {
            let mut pair = line.split(',');
            let first = pair.next().ok_or_else(|| eyre!("Unexpected end of input"))?;
            let second = pair.next().ok_or_else(|| eyre!("Unexpected end of input"))?;
            if pair.next().is_some() {
                bail!("Unexpected additional input");
            }
            Ok(Assignment {
                first: parse_range(first)?,
                second: parse_range(second)?,
            })
        })
        .collect()
}
fn part2(assignments: &[Assignment]) -> i32 {
    assignments.iter().fold(0, |acc, assignment| {
        let first = &assignment.first;
        let second = &assignment.second;
        if first.contains(second.start()) || first.contains(second.end()) || second.contains(first.start()) || second.contains(first.end()) {
            acc + 1
        } else {
            acc
        }
    })
}

fn part1(assignments: &[Assignment]) -> i32 {
    assignments.iter().fold(0, |acc, assignment| {
        let first = &assignment.first;
        let second = &assignment.second;
        if (first.contains(second.start()) && first.contains(second.end())) || (second.contains(first.start()) && second.contains(first.end())) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn solve(input: &str) -> Result<Solution> {
    let assignments = parse(input)?;
    Ok(Solution {
        first: format!("{}", part1(&assignments)),
        second: format!("{}", part2(&assignments)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        let assignments = parse(input).unwrap();
        assert_eq!(part1(&assignments), 2);
        assert_eq!(part2(&assignments), 4);
    }
}
