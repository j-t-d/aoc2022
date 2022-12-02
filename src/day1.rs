use super::Solution;
use color_eyre::eyre::eyre;
use color_eyre::Result;

#[derive(Default, Debug)]
struct Elf {
    calories: Vec<isize>,
    sum: isize,
}

fn parse(input: &str) -> Result<Vec<Elf>> {
    let mut elves = Vec::new();
    let mut current_elf = Elf::default();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = Elf::default();
        } else {
            let calories: isize = line.parse()?;
            current_elf.calories.push(calories);
            current_elf.sum += calories;
        }
    }
    elves.push(current_elf);
    Ok(elves)
}

fn part1(elves: &Vec<Elf>) -> Result<isize> {
    Ok(elves.last().ok_or_else(|| eyre!("No max elf"))?.sum)
}

fn part2(elves: &Vec<Elf>) -> Result<isize> {
    Ok(elves[elves.len() - 3..].iter().fold(0, |sum, elf| sum + elf.sum))
}

pub fn solve(input: &str) -> Result<Solution> {
    let mut elves = parse(input)?;
    elves.sort_by(|a, b| a.sum.cmp(&b.sum));

    Ok(Solution {
        first: format!("Calories {}", part1(&elves)?),
        second: format!("Three Total Calories {}", part2(&elves)?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let mut elves = parse(input).unwrap();
        elves.sort_by(|a, b| a.sum.cmp(&b.sum));
        assert_eq!(part1(&elves).unwrap(), 24000);
        assert_eq!(part2(&elves).unwrap(), 45000);
    }
}
