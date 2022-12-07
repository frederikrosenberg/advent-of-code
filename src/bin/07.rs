
pub fn part_one(input: &str) -> Option<u32> {
    let mut stack: Vec<u32> = Vec::new();
    let mut total: u32 = 0;
    let mut current: u32 = 0; 

    for line in input.lines().skip(1) {
        match line {
            "$ ls" => {}
            "$ cd .." => {
                if current < 100000 {
                    total += current;
                }
                current += stack.pop().unwrap();
            }
            s if s.starts_with("$ cd ") => {
                stack.push(current);
                current = 0;
            }
            s if s.starts_with("dir") => {}
            _ => {
                current += line.split_whitespace().next().unwrap().parse::<u32>().unwrap();
            }
        }
    }

    while !stack.is_empty() {
        if current < 100000 {
            total += current;
        }
        current += stack.pop().unwrap();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stack: Vec<u32> = Vec::new();
    let mut sizes: Vec<u32> = Vec::new();
    let mut current: u32 = 0; 

    for line in input.lines().skip(1) {
        match line {
            "$ ls" => {}
            "$ cd .." => {
                sizes.push(current);
                current += stack.pop().unwrap();
            }
            s if s.starts_with("$ cd ") => {
                stack.push(current);
                current = 0;
            }
            s if s.starts_with("dir") => {}
            _ => {
                current += line.split_whitespace().next().unwrap().parse::<u32>().unwrap();
            }
        }
    }

    while !stack.is_empty() {
        sizes.push(current);
        current += stack.pop().unwrap();
    }

    let missing = 30_000_000 - (70_000_000 - current);

    sizes.into_iter().filter(|s| s > &missing).min_by_key(|s| s.abs_diff(missing))
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
