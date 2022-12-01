use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            if max < current {
                max = current;
            }
            current = 0;
            continue;
        }

        if let Ok(number) = line.parse::<u32>() {
            current += number;
        }
    }

    if max < current {
        max = current;
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let max = input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|group| {
            group
                .1
                .filter_map(|line| line.parse::<u32>().ok())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<u32>();

    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
