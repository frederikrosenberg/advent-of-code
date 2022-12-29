use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn go(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }
}

struct Crater {
    elves: HashSet<(i32, i32)>,
}

impl Display for Crater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.get_min_max();

        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                if self.elves.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

impl Crater {
    fn from_input(input: &str) -> Crater {
        let mut elves = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, point) in line.chars().enumerate() {
                if point == '#' {
                    elves.insert((x as i32, y as i32));
                }
            }
        }

        Crater { elves }
    }

    fn can_move(&self, (x, y): (i32, i32), dir: &Direction) -> bool {
        match dir {
            Direction::North => {
                !self.elves.contains(&(x - 1, y - 1))
                    && !self.elves.contains(&(x, y - 1))
                    && !self.elves.contains(&(x + 1, y - 1))
            }
            Direction::South => {
                !self.elves.contains(&(x - 1, y + 1))
                    && !self.elves.contains(&(x, y + 1))
                    && !self.elves.contains(&(x + 1, y + 1))
            }
            Direction::West => {
                !self.elves.contains(&(x - 1, y - 1))
                    && !self.elves.contains(&(x - 1, y))
                    && !self.elves.contains(&(x - 1, y + 1))
            }
            Direction::East => {
                !self.elves.contains(&(x + 1, y - 1))
                    && !self.elves.contains(&(x + 1, y))
                    && !self.elves.contains(&(x + 1, y + 1))
            }
        }
    }

    fn none_around(&self, (x, y): (i32, i32)) -> bool {
        !self.elves.contains(&(x - 1, y - 1))
            && !self.elves.contains(&(x - 1, y))
            && !self.elves.contains(&(x - 1, y + 1))
            && !self.elves.contains(&(x, y - 1))
            && !self.elves.contains(&(x, y + 1))
            && !self.elves.contains(&(x + 1, y - 1))
            && !self.elves.contains(&(x + 1, y))
            && !self.elves.contains(&(x + 1, y + 1))
    }

    fn get_area(&self) -> i32 {
        let (min, max) = self.get_min_max();

        let w = max.0.abs_diff(min.0) + 1;
        let h = max.1.abs_diff(min.1) + 1;

        (w * h) as i32
    }

    fn get_min_max(&self) -> ((i32, i32), (i32, i32)) {
        let mut min = (100, 100);
        let mut max = (0, 0);

        for (x, y) in self.elves.iter() {
            if x < &min.0 {
                min.0 = *x;
            }
            if x > &max.0 {
                max.0 = *x;
            }
            if y < &min.1 {
                min.1 = *y;
            }
            if y > &max.1 {
                max.1 = *y;
            }
        }

        (min, max)
    }
}

fn do_round(crater: &mut Crater, order: &mut [Direction]) -> bool {
    let mut move_map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    for elf in crater.elves.iter() {
        if crater.none_around(*elf) {
            continue;
        }
        for dir in order.iter() {
            if crater.can_move(*elf, dir) {
                let entry = move_map.entry(dir.go(*elf)).or_default();
                entry.push(*elf);

                break;
            }
        }
    }

    for (new_pos, elves) in move_map.iter().filter(|(_, v)| v.len() == 1) {
        let elf = elves[0];
        crater.elves.remove(&elf);
        crater.elves.insert(*new_pos);
    }

    order.rotate_left(1);

    move_map.is_empty()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut crater = Crater::from_input(input);
    let mut order = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        do_round(&mut crater, &mut order);
    }

    Some(crater.get_area() as u32 - crater.elves.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut crater = Crater::from_input(input);
    let mut order = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut round = 1;

    while !do_round(&mut crater, &mut order) {
        round += 1;
    }

    Some(round)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
