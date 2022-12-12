use std::collections::VecDeque;

use itertools::Itertools;

struct Map {
    width: usize,
    height: usize,
    map: Vec<char>,
    start: usize,
    end: usize,
}

impl Map {
    fn from(input: &str) -> Map {
        let map: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
        let width = input.lines().next().unwrap().chars().count();
        let start = map.iter().position(|&c| c == 'S').unwrap();
        let end = map.iter().position(|&c| c == 'E').unwrap();

        Map {
            map,
            width,
            height: input.lines().count(),
            start,
            end,
        }
    }

    fn height_index(&self, index: usize) -> usize {
        let char = self.map[index];
        match char {
            'E' => (b'z' - b'a') as usize,
            'S' => 0,
            _ => (char as u8 - b'a') as usize,
        }
    }

    fn height(&self, x: usize, y: usize) -> usize {
        self.height_index(self.index(x, y))
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x as usize + y as usize * self.width
    }

    fn position(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;

        (x, y)
    }

    fn adjacent_edges(&self, index: usize) -> Vec<usize> {
        let (x, y) = self.position(index);
        let height = self.height(x, y);
        let mut result = Vec::with_capacity(4);

        if y < self.height - 1 {
            let up = self.index(x, y + 1);
            let up_heigth = self.height_index(up);
            if height.abs_diff(up_heigth) <= 1 || height > up_heigth {
                result.push(up);
            }
        }

        if y > 0 {
            let down = self.index(x, y - 1);
            let down_heigth = self.height_index(down);
            if height.abs_diff(down_heigth) <= 1 || height > down_heigth {
                result.push(down);
            }
        }

        if x < self.width - 1 {
            let right = self.index(x + 1, y);
            let right_heigth = self.height_index(right);
            if height.abs_diff(right_heigth) <= 1 || height > right_heigth {
                result.push(right);
            }
        }

        if x > 0 {
            let left = self.index(x - 1, y);
            let left_heigth = self.height_index(left);
            if height.abs_diff(left_heigth) <= 1 || height > left_heigth {
                result.push(left);
            }
        }

        result
    }
}

fn path(map: &Map, start: usize, end: usize) -> u32 {
    let mut queue = VecDeque::new();
    let mut parent: Vec<Option<usize>> = vec![None; map.map.len()];
    queue.push_back(start);

    let mut count = 0;

    while let Some(index) = queue.pop_front() {
        if end == index {
            let mut next = index;
            while let Some(p) = parent[next] {
                next = p;
                count += 1;
                if next == start {
                    break;
                }
            }
            break;
        }

        for edge in map.adjacent_edges(index) {
            if parent[edge].is_none() {
                parent[edge] = Some(index);
                queue.push_back(edge);
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input);
    let count = path(&map, map.start, map.end);

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from(input);
    let min = map
        .map
        .iter()
        .positions(|&c| c == 'a' || c == 'S')
        .map(|p| path(&map, p, map.end))
        .filter(|&p| p > 0)
        .min();

    min

    /*
    // This gets the correct answer only for the input for but not for the example. Some work is still
    // needed but is a lot faster.
    let map = Map::from(input);
    let mut parent: Vec<Option<(usize, usize)>> = vec![None; map.map.len()];

    for start in map.map.iter().positions(|&c| c == 'a' || c == 'S') {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        let mut to_break = false;

        while let Some(index) = queue.pop_front() {
            if map.end == index.0 {
                break;
            }

            for edge in map.adjacent_edges(index.0) {
                if parent[edge].is_none() {
                    parent[edge] = Some(index);
                    queue.push_back((edge, index.1 + 1));
                } else if parent[edge].unwrap().1 > index.1 {
                    parent[edge] = Some(index);
                    to_break = true;
                }
            }

            if to_break {
                break;
            }
        }
    }

    let mut count = 0;

    let mut next = map.end;
    while let Some(p) = parent[next] {
        next = p.0;
        count += 1;
        if map.map[next] == 'a' {
            break;
        }
    }

    Some(count)
    */
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
