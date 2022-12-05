/*
 * Strings where first char is the top of the stack
 */

use itertools::Itertools;

fn parse_initial<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in lines {
        let mut iter = line.chars();
        let mut index = 0;
        while let Some((_, c, _)) = iter.next_tuple() {
            // skip the space
            iter.next();

            if c.is_ascii_digit() {
                break;
            }

            if !c.is_whitespace() {
                if let Some(s) = result.get_mut(index) {
                    s.push(c);
                } else {
                    result.push(c.to_string())
                }
            } else if let None = result.get(index) {
                    result.push(String::new());
            }

            index += 1;
        }
    }
    result
}


pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let mut initial = parse_initial(lines.take_while_ref(|line| !line.is_empty()));

    // skip empty line
    lines.next();

    for line in lines {
        if let Some((_, count, _, from, _, to)) = line.split_ascii_whitespace().next_tuple() {
            let from_stack = initial.get_mut(from.parse::<usize>().unwrap() - 1).unwrap();
            let mut to_move = String::new();

            for _ in 0..count.parse().unwrap() {
                to_move.insert(0, from_stack.remove(0));
            }
        
            let to_stack = initial.get_mut(to.parse::<usize>().unwrap() - 1).unwrap();
            to_stack.insert_str(0, &to_move);
        }
    }

    let result = initial.iter().map(|s| s.chars().nth(0).unwrap()).join("");

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let mut initial = parse_initial(lines.take_while_ref(|line| !line.is_empty()));

    // skip empty line
    lines.next();

    for line in lines {
        if let Some((_, count, _, from, _, to)) = line.split_ascii_whitespace().next_tuple() {
            let from_stack = initial.get_mut(from.parse::<usize>().unwrap() - 1).unwrap();
            let mut to_move = String::new();

            for _ in 0..count.parse().unwrap() {
                to_move.push(from_stack.remove(0));
            }
        
            let to_stack = initial.get_mut(to.parse::<usize>().unwrap() - 1).unwrap();
            to_stack.insert_str(0, &to_move);
        }
    }

    let result = initial.iter().map(|s| s.chars().nth(0).unwrap()).join("");

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial() {
        let input = "    [D]    \n\
                     [N] [C]    \n\
                     [Z] [M] [P]\n\
                      1   2   3 ";
        assert_eq!(parse_initial(input.lines()), vec!["NZ", "DCM", "P"])
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
