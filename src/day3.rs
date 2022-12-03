use super::Solution;
use ahash::HashSet;
use color_eyre::eyre::bail;
use color_eyre::Result;

#[derive(Debug)]
struct Compartment(HashSet<char>);

#[derive(Debug)]
struct Rucksack {
    compartment1: Compartment,
    compartment2: Compartment,
    all: HashSet<char>,
}

impl Rucksack {
    fn mispacked(&self) -> Result<char> {
        let intersections = self.compartment1.0.intersection(&self.compartment2.0).collect::<Vec<_>>();
        if intersections.len() != 1 {
            println!("{:?} {:?}", intersections, self);
            bail!("Expected 1 mispacked item");
        }
        Ok(*intersections[0])
    }
}

fn priority(item: char) -> i32 {
    match item.is_ascii_lowercase() {
        true => item as i32 - 'a' as i32 + 1,
        false => item as i32 - 'A' as i32 + 27,
    }
}

fn part1(rucksacks: &[Rucksack]) -> Result<i32> {
    rucksacks.iter().try_fold(0, |acc, sack| Ok(acc + priority(sack.mispacked()?)))
}

fn part2(rucksacks: &[Rucksack]) -> Result<i32> {
    if rucksacks.len() % 3 != 0 {
        bail!("Expected groups of 3");
    }
    rucksacks.chunks(3).try_fold(0, |acc, chunk| {
        let first_intersection = chunk[0].all.intersection(&chunk[1].all).copied().collect::<HashSet<_>>();
        let second_intersection = first_intersection.intersection(&chunk[2].all).copied().collect::<Vec<_>>();
        if second_intersection.len() != 1 {
            bail!("Expected 1 intersection in group")
        }
        Ok(acc + priority(second_intersection[0]))
    })
}

fn parse_compartment(input: &str) -> Result<Compartment> {
    Ok(Compartment(
        input
            .chars()
            .map(|item| {
                if !item.is_ascii_alphabetic() {
                    bail!("Unexpected character")
                }
                Ok(item)
            })
            .collect::<Result<HashSet<char>>>()?,
    ))
}

fn parse(input: &str) -> Result<Vec<Rucksack>> {
    input
        .lines()
        .map(|line| {
            if line.len() % 2 != 0 {
                bail!("Uneven compartment");
            }
            let all = line.chars().collect();
            let (compartment1, compartment2) = line.split_at(line.len() / 2);
            Ok(Rucksack {
                compartment1: parse_compartment(compartment1)?,
                compartment2: parse_compartment(compartment2)?,
                all,
            })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<Solution> {
    let sacks = parse(input)?;
    Ok(Solution {
        first: format!("{}", part1(&sacks)?),
        second: format!("{}", part2(&sacks)?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_data() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let sacks = parse(input).unwrap();
        assert_eq!(part1(&sacks).unwrap(), 157);
        assert_eq!(part2(&sacks).unwrap(), 70);
    }
}
