use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

#[derive(Clone, Debug)]
enum Wind {
    Left,
    Right,
}

impl Wind {
    fn from_char(char: char) -> Wind {
        match char {
            '>' => Wind::Right,
            '<' => Wind::Left,
            _ => unreachable!("Invalid char"),
        }
    }
}

struct Shape {
    shape: Vec<Vec<bool>>,
}

impl Shape {
    fn get_shapes() -> [Self; 5] {
        let line = Shape {
            shape: vec![vec![true, true, true, true]],
        };
        let plus = Shape {
            shape: vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
        };
        let l = Shape {
            shape: vec![
                vec![false, false, true],
                vec![false, false, true],
                vec![true, true, true],
            ],
        };
        let vertical = Shape {
            shape: vec![vec![true], vec![true], vec![true], vec![true]],
        };
        let sqaure = Shape {
            shape: vec![vec![true, true], vec![true, true]],
        };

        [line, plus, l, vertical, sqaure]
    }

    fn can_move<const WIDTH: usize>(&self, new_pos: &Position, board: &Board<WIDTH>) -> bool {
        if self.is_outside_board::<WIDTH>(new_pos.x) {
            return false;
        }

        for row in (0..self.shape.len()).rev() {
            if new_pos.y < row {
                return false;
            }
            let y = new_pos.y - row;
            if y > board.max_shape_position.unwrap_or(0) {
                break;
            }
            let index = y % HEIGHT;
            for col in 0..self.shape[row].len() {
                if !self.shape[row][col] {
                    continue;
                }

                if board.board[index][new_pos.x + col] {
                    return false;
                }
            }
        }

        true
    }

    fn is_outside_board<const WIDTH: usize>(&self, x: usize) -> bool {
        x + self.shape[0].len() > WIDTH
    }

    fn place_in_board<const WIDTH: usize>(&self, pos: &Position, board: &mut Board<WIDTH>) {
        for row in 0..self.shape.len() {
            let y = (pos.y - row) % HEIGHT;
            for col in 0..self.shape[row].len() {
                if !self.shape[row][col] {
                    continue;
                }

                board.board[y][pos.x + col] = true;
            }
        }
    }
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

const HEIGHT: usize = 10000;

struct Board<const WIDTH: usize> {
    board: [[bool; WIDTH]; HEIGHT],
    max_shape_position: Option<usize>,
}

impl<const WIDTH: usize> Display for Board<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.max_shape_position.is_none() {
            return writeln!(f, "Board is empty");
        }
        let max = self.max_shape_position.unwrap();
        writeln!(f, "Board: ")?;
        for h in (0..=max).rev() {
            write!(f, "|")?;
            let row = self.board[h % HEIGHT];
            for entry in row.iter() {
                if *entry {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "| {:0>4}", h)?;
        }

        write!(f, "+")?;
        for _ in 0..WIDTH {
            write!(f, "-")?;
        }
        writeln!(f, "+")
    }
}

impl<const WIDTH: usize> Board<WIDTH> {
    fn new() -> Self {
        let vec = [[false; WIDTH]; HEIGHT];
        Board {
            board: vec,
            max_shape_position: None,
        }
    }

    fn place_shape(&mut self, shape: &Shape, winds: &mut impl Iterator<Item = Wind>) {
        let height = shape.shape.len();
        let y = if let Some(max) = self.max_shape_position {
            max + height
        } else {
            0
        };

        let mut x = 2;

        for _ in 0..3 {
            let wind = winds.next().unwrap();
            let new_x = match wind {
                Wind::Left => {
                    if x == 0 {
                        0
                    } else {
                        x - 1
                    }
                }
                Wind::Right => x + 1,
            };

            if !shape.is_outside_board::<WIDTH>(new_x) {
                x = new_x;
            }
        }

        for index in self.max_shape_position.unwrap_or(0) + 1..=y {
            self.board[index % HEIGHT] = [false; WIDTH]
        }

        let mut position = Position::new(x, y);

        loop {
            // wind
            let wind = winds.next().unwrap();
            let new_x = match wind {
                Wind::Left => {
                    if position.x == 0 {
                        0
                    } else {
                        position.x - 1
                    }
                }
                Wind::Right => position.x + 1,
            };
            if shape.can_move(&Position::new(new_x, position.y), self) {
                position.x = new_x;
            }

            if position.y == 0 {
                break;
            }

            let new_pos = Position::new(position.x, position.y - 1);
            if !shape.can_move(&new_pos, self) {
                break;
            }
            position = new_pos;
        }

        shape.place_in_board(&position, self);

        if let Some(max) = self.max_shape_position {
            if max < position.y {
                self.max_shape_position = Some(position.y);
            }
        } else {
            self.max_shape_position = Some(position.y);
        }
    }

    fn height(&self) -> usize {
        if let Some(max) = self.max_shape_position {
            max + 1
        } else {
            0
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let winds = input
        .lines()
        .join("")
        .chars()
        .map(Wind::from_char)
        .collect_vec();
    let mut winds_iter = winds.into_iter().cycle();
    let shapes = Shape::get_shapes();
    let mut shapes_iter = shapes.iter().cycle();

    let mut board = Board::<7>::new();
    for _ in 0..2022 {
        let shape = shapes_iter.next().unwrap();
        board.place_shape(shape, &mut winds_iter);
    }

    Some(board.height())
}

pub fn part_two(input: &str) -> Option<usize> {
    let to = 1_000_000_000_000_usize;
    let winds = input
        .lines()
        .join("")
        .chars()
        .map(Wind::from_char)
        .collect_vec();
    let mut winds_iter = winds.into_iter().cycle();
    let mut map = HashMap::new();

    let shapes = Shape::get_shapes();
    let mut shapes_iter = shapes.iter().cycle();

    let mut board = Board::<7>::new();

    for i in 0..to {
        let shape = shapes_iter.next().unwrap();
        board.place_shape(shape, &mut winds_iter);
        if let Some(max) = board.max_shape_position {
            if max > HEIGHT - 10 {
                break;
            }

            if i % 5 != 0 {
                continue;
            }
            map.entry(max).or_insert(i);
        }
    }

    let mut repeating: Option<(usize, usize)> = None;

    // Finding repeating
    for start in 10..HEIGHT / 2 {
        for i in start + 10..HEIGHT / 2 {
            if board.board[start] == board.board[i] {
                let mut found = true;
                for offset in 1..=i - start {
                    if board.board[start + offset] != board.board[i + offset] {
                        found = false;
                        break;
                    }
                }
                if found {
                    repeating = Some((start, i));
                    break;
                }
            }
        }
        if repeating.is_some() {
            break;
        }
    }

    if let Some((start, end)) = &repeating {
        let start_i = map[start];
        let end_i = map[end];
        let diff = end_i - start_i;

        let to = to - start_i;
        let mut total = (to / diff) * (end - start) + start;

        let missing = to % diff;
        let missing_index = start_i + missing;

        let height = map.iter().find(|(_, &v)| v == missing_index).unwrap().0;
        total += height - start;

        return Some(total);
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1_514_285_714_288));
    }
}
