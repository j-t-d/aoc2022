use super::*;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::fmt;

#[derive(Debug, Clone)]
struct Tree {
    height: u8,
    visible: bool,
    score: i32,
}

#[derive(Clone)]
struct Forest {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

impl Forest {
    pub fn new(height: usize, width: usize, trees: Vec<Tree>) -> Self {
        Self { trees, width, height }
    }

    pub fn get_height(&self, x: usize, y: usize) -> u8 {
        self.trees[y * self.height + x].height
    }

    pub fn set_visible(&mut self, x: usize, y: usize) {
        self.trees[y * self.height + x].visible = true;
    }

    pub fn set_score(&mut self, x: usize, y: usize, score: i32) {
        self.trees[y * self.height + x].score = score;
    }

    pub fn count_visible(&self) -> i32 {
        self.trees.iter().fold(0, |acc, tree| if tree.visible { acc + 1 } else { acc })
    }
}

impl fmt::Debug for Forest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, tree) in self.trees.iter().enumerate() {
            let visible = if tree.visible { "t" } else { "f" };
            write!(f, "{}:{} ", tree.height, visible)?;
            if (index + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        writeln!(f)
    }
}

fn score_left(forest: &Forest, x: usize, y: usize) -> i32 {
    let height = forest.get_height(x, y);
    let mut score = 0;
    for x in (0..x).rev() {
        score += 1;
        if forest.get_height(x, y) >= height {
            break;
        }
    }
    score
}

fn score_right(forest: &Forest, x: usize, y: usize) -> i32 {
    let height = forest.get_height(x, y);
    let mut score = 0;
    for x in x + 1..forest.width {
        score += 1;
        if forest.get_height(x, y) >= height {
            break;
        }
    }
    score
}

fn score_up(forest: &Forest, x: usize, y: usize) -> i32 {
    let height = forest.get_height(x, y);
    let mut score = 0;
    for y in (0..y).rev() {
        score += 1;
        if forest.get_height(x, y) >= height {
            break;
        }
    }
    score
}

fn score_down(forest: &Forest, x: usize, y: usize) -> i32 {
    let height = forest.get_height(x, y);
    let mut score = 0;
    for y in y + 1..forest.height {
        score += 1;
        if forest.get_height(x, y) >= height {
            break;
        }
    }
    score
}

fn part2(forest: &mut Forest) -> Result<i32> {
    for x in 0..forest.width {
        for y in 0..forest.height {
            let score = { score_left(forest, x, y) * score_right(forest, x, y) * score_up(forest, x, y) * score_down(forest, x, y) };
            forest.set_score(x, y, score);
        }
    }

    Ok(forest
        .trees
        .iter()
        .max_by_key(|tree| tree.score)
        .ok_or_else(|| eyre!("Failed to find highest score"))?
        .score)
}

fn part1(forest: &mut Forest) -> i32 {
    for x in 0..forest.width {
        let mut last_highest = forest.get_height(x, 0);
        forest.set_visible(x, 0);
        for y in 0..forest.height {
            if forest.get_height(x, y) > last_highest {
                last_highest = forest.get_height(x, y);
                forest.set_visible(x, y);
            }
        }
    }

    for x in 0..forest.width {
        let mut last_highest = forest.get_height(x, forest.height - 1);
        forest.set_visible(x, forest.height - 1);
        for y in (0..forest.height).rev() {
            if forest.get_height(x, y) > last_highest {
                last_highest = forest.get_height(x, y);
                forest.set_visible(x, y);
            }
        }
    }

    for y in 0..forest.height {
        let mut last_highest = forest.get_height(0, y);
        forest.set_visible(0, y);
        for x in 0..forest.width {
            if forest.get_height(x, y) > last_highest {
                last_highest = forest.get_height(x, y);
                forest.set_visible(x, y);
            }
        }
    }

    for y in 0..forest.height {
        let mut last_highest = forest.get_height(forest.width - 1, y);
        forest.set_visible(forest.width - 1, y);
        for x in (0..forest.width).rev() {
            if forest.get_height(x, y) > last_highest {
                last_highest = forest.get_height(x, y);
                forest.set_visible(x, y);
            }
        }
    }

    forest.count_visible()
}

fn parse(input: &str) -> Result<Forest> {
    let mut trees = Vec::new();
    let mut width = 0;
    for line in input.lines() {
        let mut curr_width = 0;
        for char in line.chars() {
            trees.push(Tree {
                visible: false,
                height: char.to_digit(10).ok_or_else(|| eyre!("Failed to convert to digit"))? as u8,
                score: 0,
            });
            curr_width += 1;
        }
        if width == 0 {
            width = curr_width;
        } else if width != curr_width {
            bail!("Unexpected line width")
        }
    }

    Ok(Forest::new(trees.len() / width, width, trees))
}

pub fn solve(input: &str) -> Result<Solution> {
    let mut forest = parse(input)?;

    Ok(Solution {
        first: format!("{}", part1(&mut forest)),
        second: format!("{}", part2(&mut forest)?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = r#"30373
25512
65332
33549
35390"#;

        let mut forest = parse(input).unwrap();
        assert_eq!(part1(&mut forest), 21);

        assert_eq!(score_left(&mut forest, 2, 1), 1);
        assert_eq!(score_right(&mut forest, 2, 1), 2);
        assert_eq!(score_up(&mut forest, 2, 1), 1);
        assert_eq!(score_down(&mut forest, 2, 1), 2);
        assert_eq!(score_left(&mut forest, 2, 3), 2);
        assert_eq!(score_right(&mut forest, 2, 3), 2);
        assert_eq!(score_up(&mut forest, 2, 3), 2);
        assert_eq!(score_down(&mut forest, 2, 3), 1);

        assert_eq!(part2(&mut forest).unwrap(), 8);
    }
}
