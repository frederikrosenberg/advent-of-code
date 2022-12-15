use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Rock {
    points: Vec<Point>,
}

#[derive(Clone)]
enum Type {
    None,
    Rock,
    Sand,
}

struct Map {
    x_offset: usize,
    y_max: usize,

    width: usize,
    height: usize,

    map: Vec<Type>,
}

const SAND_DROP: Point = Point { x: 500, y: 0 };

impl Map {
    fn from_rocks(rocks: &[Rock], is_part_one: bool) -> Map {
        let xs = rocks
            .iter()
            .flat_map(|rs| rs.points.iter().map(|r| r.x))
            .collect_vec();
        let ys = rocks.iter().flat_map(|rs| rs.points.iter().map(|r| r.y));
        let x_min = xs.iter().min().unwrap();
        let x_max = xs.iter().max().unwrap();
        let y_max = ys.max().unwrap();

        let mut width = x_max - x_min;
        let mut x_offset = *x_min - 1;
        if !is_part_one {
            width += 500;
            x_offset -= 250;
        } else {
            width += 2;
            x_offset -= 1;
        }

        let height = y_max + 3;

        let mut map = vec![Type::None; (width * height) as usize];

        let index = |x, y| ((x - x_offset) + y * width) as usize;

        for rock in rocks.iter() {
            for (start, end) in rock.points.iter().tuple_windows() {
                if start.x == end.x {
                    let s = start.y.min(end.y);
                    let e = start.y.max(end.y);
                    for y in s..=e {
                        let i = index(start.x, y);
                        map[i] = Type::Rock;
                    }
                } else {
                    let s = start.x.min(end.x);
                    let e = start.x.max(end.x);
                    for x in s..=e {
                        let i = index(x, start.y);
                        map[i] = Type::Rock;
                    }
                }
            }
        }

        if !is_part_one {
            let start = (height - 1) * width;
            for item in map.iter_mut().skip(start) {
                *item = Type::Rock;
            }
        }

        Map {
            x_offset,
            width,
            height,
            map,
            y_max,
        }
    }

    fn place_sand_part1(&mut self, start: usize) -> bool {
        let mut current = start;
        while let Some(new) = self.next_point(current) {
            current = new;
            if current + self.width >= (self.y_max + 1) * self.width {
                return false;
            }
        }

        self.map[current] = Type::Sand;

        true
    }

    fn place_sand_part2(&mut self, start: usize) -> bool {
        let mut current = start;
        while let Some(new) = self.next_point(current) {
            current = new;
        }

        self.map[current] = Type::Sand;

        current != start
    }

    fn next_point(&self, sand: usize) -> Option<usize> {
        let start = sand + self.width;
        if let Type::None = self.map[start] {
            return Some(start);
        }

        if let Type::None = self.map[start - 1] {
            return Some(start - 1);
        }

        if let Type::None = self.map[start + 1] {
            return Some(start + 1);
        }

        None
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = |x, y| (x + y * self.width) as usize;

        writeln!(f)?;

        for y in 0..self.height {
            for x in 0..self.width {
                if y == SAND_DROP.y && x + self.x_offset == SAND_DROP.x {
                    write!(f, "+")?;
                } else {
                    let i = index(x, y);
                    match self.map[i] {
                        Type::None => write!(f, ".")?,
                        Type::Rock => write!(f, "#")?,
                        Type::Sand => write!(f, "o")?,
                    };
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Point {
    fn from_str(point: &str) -> Point {
        let (x, y) = point.split_once(',').expect("Invalid point");
        let (x, y) = (x.parse().expect("Number"), y.parse().expect("Number"));

        Point { x, y }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rocks = input
        .lines()
        .map(|l| Rock {
            points: l.split(" -> ").map(Point::from_str).collect_vec(),
        })
        .collect_vec();

    let mut map = Map::from_rocks(rocks.as_slice(), true);

    let mut count = 0;
    let start = SAND_DROP.x - map.x_offset;

    while map.place_sand_part1(start) {
        count += 1;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rocks = input
        .lines()
        .map(|l| Rock {
            points: l.split(" -> ").map(Point::from_str).collect_vec(),
        })
        .collect_vec();

    let mut map = Map::from_rocks(rocks.as_slice(), false);

    let mut count = 1;
    let start = SAND_DROP.x - map.x_offset;

    while map.place_sand_part2(start) {
        count += 1;
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
