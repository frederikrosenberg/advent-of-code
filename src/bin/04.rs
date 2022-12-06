struct Section {
    lower: u32,
    higher: u32,
}

impl Section {
    fn new(lower: u32, higher: u32) -> Section {
        Section { lower, higher }
    }

    fn contains(&self, other: &Section) -> bool {
        self.lower <= other.lower && self.higher >= other.higher ||
        self.lower >= other.lower && self.higher <= other.higher
    }

    fn overlaps(&self, other: &Section) -> bool {
        self.lower <= other.lower && other.lower <= self.higher ||
        other.lower <= self.lower && self.lower <= other.higher 
    }

    fn parse(line: &str) -> (Section, Section) {
        let sections: Vec<&str> = line.split(',').collect();
        let first: Vec<_> = sections[0].split('-').collect();
        let second: Vec<_> = sections[1].split('-').collect();

        (
            Section::new(
                first[0].parse().expect("Should be a number"),
                first[1].parse().expect("Should be a number"),
            ),
            Section::new(
                second[0].parse().expect("Should be a number"),
                second[1].parse().expect("Should be a number"),
            ),
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(Section::parse)
        .filter(|s| s.0.contains(&s.1))
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(Section::parse)
        .filter(|s| s.0.overlaps(&s.1))
        .count();

    Some(count as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
