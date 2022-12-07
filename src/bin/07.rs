use itertools::Itertools;
use std::str::Lines;

struct FileSystemIter<'a> {
    lines: Lines<'a>,
    stack: Vec<u32>,
    current: u32,
}

impl<'a> FileSystemIter<'a> {
    fn new(lines: Lines<'a>) -> FileSystemIter {
        FileSystemIter {
            lines,
            stack: Vec::new(),
            current: 0,
        }
    }
}

impl Iterator for FileSystemIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        while let Some(line) = self.lines.next() {
            match line {
                "$ ls" => {}
                "$ cd .." => {
                    result = Some(self.current);
                    self.current += self.stack.pop().unwrap();
                    break;
                }
                s if s.starts_with("$ cd ") => {
                    self.stack.push(self.current);
                    self.current = 0;
                }
                s if s.starts_with("dir") => {}
                _ => {
                    self.current += line
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                }
            }
        }

        // No need to return the last current since the stack 
        // contains an 0 from the root folder
        if result.is_none() && !self.stack.is_empty() {
            result = Some(self.current);
            self.current += self.stack.pop().unwrap();
        }

        result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = FileSystemIter::new(input.lines())
        .filter(|s| *s < 100_000)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let fs = FileSystemIter::new(input.lines()).collect_vec();
    let total = fs.last().unwrap();
    let missing = 30_000_000 - (70_000_000 - total);

    fs.into_iter()
        .filter(|s| s > &missing)
        .min_by_key(|s| s.abs_diff(missing))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
