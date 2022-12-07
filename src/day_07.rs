use crate::utils::LineIterator;
use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::path::PathBuf;

type Output = usize;

#[aoc(day7, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let path_to_size = input_to_path_sizes(input);

    path_to_size.values().filter(|size| size <= &&100_000).sum()
}

#[aoc(day7, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let path_to_size = input_to_path_sizes(input);
    let to_free = 30_000_000 - (70_000_000 - *path_to_size.get(&PathBuf::from("/")).unwrap());

    *path_to_size
        .values()
        .filter(|size| size >= &&to_free)
        .min()
        .unwrap()
}

fn input_to_path_sizes(input: &str) -> HashMap<PathBuf, usize> {
    let mut current_directory = PathBuf::from("/");

    // TODO: There has to be a better way without hashmaps
    // Why do all my solutions start out with hashmaps
    let mut path_to_size: HashMap<PathBuf, usize> = HashMap::default();

    for line in LineIterator::from(input) {
        let split = line.split_whitespace().collect::<Vec<_>>();

        match split[0] {
            "$" => match split[1] {
                "cd" => match split[2] {
                    "/" => current_directory = PathBuf::from("/"),
                    ".." => {
                        current_directory.pop();
                    }
                    _ => current_directory = current_directory.join(split[2]),
                },
                "ls" => { /* We parse command output in the _ arm */ }
                _ => unreachable!("unexpected command: {:?}", split[1]),
            },
            "dir" => { /* We just find sizes and add them to current dir */ }
            _ => {
                let size: usize = split[0].parse().unwrap();
                let mut current_directory_clone = current_directory.clone();

                loop {
                    let entry = path_to_size
                        .entry(current_directory_clone.clone())
                        .or_default();
                    *entry += size;

                    // Reached end
                    if !current_directory_clone.pop() {
                        break;
                    }
                }
            }
        }
    }

    path_to_size
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
$ cd /
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
7214296 k
    "#;

    #[test]
    fn test_part_1() {
        let expected = 95437;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 24933642;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
