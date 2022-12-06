fn score(a: char, b: char) -> u32 {
    let choice = match b {
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => panic!("Unexpected input {}", b),
    };

    let outcome = match (a, b) {
        ('a', 'x') => 3,
        ('a', 'y') => 6,
        ('a', 'z') => 0,
        ('b', 'x') => 0,
        ('b', 'y') => 3,
        ('b', 'z') => 6,
        ('c', 'x') => 6,
        ('c', 'y') => 0,
        ('c', 'z') => 3,
        _ => panic!("Unexpected combination ({}, {})", a, b),
    };

    choice + outcome
}

fn choose(a: char, b: char) -> char {
    match (a, b) {
        ('a', 'x') => 'z',
        ('a', 'y') => 'x',
        ('a', 'z') => 'y',
        ('b', 'x') => 'x',
        ('b', 'y') => 'y',
        ('b', 'z') => 'z',
        ('c', 'x') => 'y',
        ('c', 'y') => 'z',
        ('c', 'z') => 'x',
        _ => panic!("Unexpected combination ({}, {})", a, b),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .to_lowercase()
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let a = chars.next().expect("Expected a char");
            let b = chars.nth(1).expect("Expected a char at position 1");
            score(a, b)
        })
        .sum::<u32>();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input
        .to_lowercase()
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let a = chars.next().expect("Expected a char");
            let b = chars.nth(1).expect("Expected a char at position 1");
            score(a, choose(a, b))
        })
        .sum::<u32>();

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
