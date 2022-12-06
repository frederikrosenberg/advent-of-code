use itertools::Itertools;

fn _find_next(input: &str, distinct: usize) -> u32 {
    let mut position = 0;
    for (index, value) in input.chars().collect_vec().windows(distinct).enumerate() {
        if value.iter().unique().count() == distinct {
            position = index + distinct;
            break;
        }
    }
    position as u32
}

fn find_next_faster(input: &str, distinct: usize) -> u32 {
    let mut current = String::new();

    let mut index = 0;

    for char in input.chars() {
        if let Some(i) = current.find(char) {
            for _ in 0..(i + 1) {
                current.remove(0);
            }
        }

        current.push(char);
        index += 1;

        if current.len() == distinct {
            break;
        }
    }

    index as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_next_faster(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_next_faster(input, 14))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
