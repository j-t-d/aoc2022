use super::*;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use ego_tree::{NodeRef, Tree};
use std::path::PathBuf;

#[derive(Debug)]
struct PathEntry {
    size: i32,
    path: PathBuf,
}

impl PathEntry {
    pub fn new(size: i32, path: PathBuf) -> PathEntry {
        PathEntry { size, path }
    }
}

fn parse(input: &str) -> Result<Tree<PathEntry>> {
    let mut cwd = PathBuf::new();
    let mut rtn = Tree::new(PathEntry::new(0, PathBuf::new()));
    let mut current_node_id = rtn.root().id();
    for line in input.lines() {
        let inputs = line.split(' ').collect::<Vec<_>>();
        match inputs[0] {
            "$" => {
                let mut current_node = rtn.get(current_node_id).ok_or_else(|| eyre!("Could not find node id"))?;
                match inputs[1] {
                    "cd" => match inputs[2] {
                        ".." => {
                            cwd.pop();
                            if let Some(parent) = current_node.parent() {
                                current_node = parent;
                            } else {
                                current_node = rtn.root();
                            }
                        }
                        "/" => {
                            cwd = PathBuf::new();
                            current_node = rtn.root();
                        }
                        dir => {
                            cwd.push(dir);
                            current_node = current_node
                                .children()
                                .find(|entry| entry.value().path == cwd)
                                .ok_or_else(|| eyre!("Could not find path at {:?}", cwd))?;
                        }
                    },
                    "ls" => {}
                    _ => bail!("Unexpected command"),
                }
                current_node_id = current_node.id();
            }
            _ => {
                let mut path = cwd.clone();
                path.push(inputs[1]);
                match inputs[0] {
                    "dir" => {
                        // create new node
                        let mut tree_node = rtn.get_mut(current_node_id).ok_or_else(|| eyre!("Failed to find node"))?;
                        tree_node.append(PathEntry::new(0, path));
                    }
                    _ => {
                        // create new file
                        let mut tree_node = rtn.get_mut(current_node_id).ok_or_else(|| eyre!("Faild to find node"))?;
                        let size: i32 = inputs[0].parse()?;
                        tree_node.append(PathEntry::new(size, path));
                    }
                }
            }
        }
    }
    Ok(rtn)
}

fn calc_size(node: &NodeRef<PathEntry>, dir_sizes: &mut Vec<PathEntry>) -> i32 {
    let size = node.children().fold(0, |acc, entry| {
        if entry.value().size == 0 {
            acc + calc_size(&entry, dir_sizes)
        } else {
            acc + entry.value().size
        }
    });
    dir_sizes.push(PathEntry::new(size, node.value().path.clone()));
    size
}

fn part1(tree: &Tree<PathEntry>) -> i32 {
    let mut dirs = Vec::new();
    let _ = calc_size(&tree.root(), &mut dirs);
    dirs.iter().fold(0, |acc, dir| if dir.size <= 100000 { acc + dir.size } else { acc })
}

fn part2(tree: &Tree<PathEntry>) -> Result<i32> {
    let mut dirs = Vec::new();
    let total = calc_size(&tree.root(), &mut dirs);
    let min_needed = 30000000 - (70000000 - total);
    Ok(dirs
        .iter()
        .filter(|entry| entry.size >= min_needed)
        .min_by(|a, b| a.size.cmp(&b.size))
        .ok_or_else(|| eyre!("Could not find amount that works"))?
        .size)
}

pub fn solve(input: &str) -> Result<Solution> {
    let tree = parse(input)?;

    Ok(Solution {
        first: format!("{}", part1(&tree)),
        second: format!("{}", part2(&tree)?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        let paths = parse(input).unwrap();
        assert_eq!(part1(&paths), 95437);
        assert_eq!(part2(&paths).unwrap(), 24933642);
    }
}
