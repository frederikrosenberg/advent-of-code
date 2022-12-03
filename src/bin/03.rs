use itertools::Itertools;

fn to_number(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 97 + 1
    } else {
        *c as u32 - 65 + 27
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| {
            let count = line.len();
            let first = line[..count / 2].chars().collect_vec();
            let second = line[count / 2..].chars().collect_vec();
            let result: &char = first
                .iter()
                .filter(|c| second.contains(c))
                .next()
                .expect("Expected at least one char");
            to_number(result)
        })
        .sum::<u32>();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let first = group
                .next()
                .expect("Expected first elf")
                .chars()
                .collect_vec();
            let second = group
                .next()
                .expect("Expected second elf")
                .chars()
                .collect_vec();
            let third = group
                .next()
                .expect("Expected third elf")
                .chars()
                .collect_vec();

            let common: &char = first
                .iter()
                .filter(|c| second.contains(c))
                .filter(|c| third.contains(c))
                .next()
                .expect("Expected at least one char");

            to_number(common)
        })
        .sum::<u32>();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
