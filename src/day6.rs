use super::*;
use ahash::HashSet;
use color_eyre::eyre::eyre;
use color_eyre::Result;

fn find_sequence(input: &str, len: usize) -> Option<u32> {
    let chars = input.chars().collect::<Vec<_>>();

    for start in 0..chars.len() {
        let end = std::cmp::min(start + len, chars.len() - 1);
        let set = chars[start..end].iter().collect::<HashSet<_>>();
        if set.len() == len {
            return Some((start + len) as u32);
        }
    }
    None
}

fn part1(input: &str) -> Option<u32> {
    find_sequence(input, 4)
}

fn part2(input: &str) -> Option<u32> {
    find_sequence(input, 14)
}

pub fn solve(input: &str) -> Result<Solution> {
    Ok(Solution {
        first: format!("{}", part1(input).ok_or_else(|| eyre!("Failed to find start"))?),
        second: format!("{}", part2(input).ok_or_else(|| eyre!("Failed to find start"))?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));

        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
