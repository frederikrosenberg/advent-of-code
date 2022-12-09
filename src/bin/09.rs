use itertools::Itertools;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Move {
    direction: Direction,
    times: i32,
}

struct Rope {
    knots: Vec<Position>,
    size: usize
}

impl Rope {
    fn new(size: usize) -> Rope {
        Rope {
            knots: vec![Position::new(); size],
            size,
        }
    }

    fn tail(&self) -> &Position {
        &self.knots[self.size - 1]
    }

    fn move_dir(&mut self, dir: &Direction) -> bool {
        self.knots[0].move_dir(dir);

        for i in 1..self.size {
            let head = self.knots[i - 1];
            let tail = &mut self.knots[i];

            if !tail.move_to(&head) {
                break;
            }
            
            if i == self.size - 1 {
                return true;
            }
        }

        false
    }
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn over_one_away(&self, other: &Position) -> bool {
        self.x.abs_diff(other.x) > 1 || self.y.abs_diff(other.y) > 1
    }

    fn move_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn move_to(&mut self, target: &Position) -> bool {
        if !self.over_one_away(target) {
            return false;
        }

        if self.x == target.x {
            self.y += if self.y > target.y { -1 } else { 1 };
        } else if self.y == target.y {
            self.x += if self.x > target.x { -1 } else { 1 };
        } else {
            self.x += if self.x > target.x { -1 } else { 1 };
            self.y += if self.y > target.y { -1 } else { 1 };
        }

        true
    }

    fn hash(&self) -> i32 {
        self.x * 10_000 + self.y
    }
}

impl Move {
    fn from_line(line: &str) -> Move {
        let (dir, num) = line.split_at(1);
        let num = num.trim().parse().unwrap();

        let direction = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right, 
            _ => unreachable!()
        };

        Move {
            direction,
            times: num,
        }
    }
}

fn solve(input: &str, size: usize) -> Option<u32> {
    let mut rope = Rope::new(size);
    let mut set = Vec::new();
    set.push(rope.tail().hash());

    for line in input.lines() {
        let m = Move::from_line(line);

        for _ in 0..m.times {
            if rope.move_dir(&m.direction) {
                set.push(rope.tail().hash());
            }
        }
    }

    Some(set.into_iter().unique().count() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
