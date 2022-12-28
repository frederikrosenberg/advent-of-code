use itertools::Itertools;

struct Position {
    col: usize,
    row: usize,
    facing: usize,
}

struct Map {
    rows: Vec<Row>,
}

struct Row {
    data: Vec<Tile>,
    offset: usize,
}

enum Tile {
    Wall,
    Open
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(Direction),
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let groups = line.trim().chars().group_by(|c| c.is_ascii_digit());

    let mut instructions = Vec::new();

    for mut group in &groups {
        if group.0 {
            instructions.push(Instruction::Move(group.1.join("").parse().unwrap()));
        } else {
            let dir = match group.1.collect_vec()[0] {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => unreachable!(),
            };
            instructions.push(Instruction::Turn(dir));
        }
    }

    instructions
}

impl Row {
    fn from_line(line: &str) -> Row {
        let row = line.trim_start();
        let offset = line.len() - row.len();

        let data = row.chars().map(|c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            _ => unreachable!()
        }).collect_vec();

        Row { data, offset }
    }
}

impl Map {
    fn from_lines(map: &str) -> Map {
        let rows = map.lines().map(Row::from_line).collect_vec();

        Map { rows }
    }

    fn next(&self, pos: &Position) -> (usize, usize, &Tile) {
        let mut row = pos.row;
        let mut col = pos.col;

        match pos.facing {
            0 => col = (col + 1) % self.rows[row].data.len(),
            1 => {
                let global = col + self.rows[row].offset;
                row = (row + 1) % self.rows.len();

                while self.rows[row].offset > global || self.rows[row].offset + self.rows[row].data.len() - 1 < global {
                    row = (row + 1) % self.rows.len();
                }

                col = global - self.rows[row].offset;
            },
            2 => {
                if col == 0 {
                    col = self.rows[row].data.len() - 1;
                } else {
                    col -= 1;
                }
            },
            3 => {
                let global = col + self.rows[row].offset;
                if row == 0 {
                    row = self.rows.len() - 1;
                } else {
                    row = (row - 1) % self.rows.len();
                }

                while self.rows[row].offset > global || self.rows[row].offset + self.rows[row].data.len() - 1 < global {
                    if row == 0 {
                        row = self.rows.len() - 1;
                    } else {
                        row = (row - 1) % self.rows.len();
                    }
                }

                col = global - self.rows[row].offset;
            },
            _ => unreachable!()
        }

        (row, col, &self.rows[row].data[col])
    }
}

impl Position {
    fn score(&self) -> usize {
        (self.row + 1) * 1000 + (self.col + 1) * 4 + self.facing
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let map = Map::from_lines(map);
    let instructions = parse_instructions(instructions);
    let mut position = Position { col: 0, row: 0, facing: 0 };

    for instruction in instructions {
        match instruction {
            Instruction::Move(moves) => {
                for _ in 0..moves {
                    let (row, col, tile) = map.next(&position);
                    if matches!(tile, Tile::Wall) {
                        break;
                    } 
                    position.col = col;
                    position.row = row;
                }

            },
            Instruction::Turn(dir) => {
                match dir {
                    Direction::Left => {
                        if position.facing == 0 {
                            position.facing = 3;
                        } else {
                            position.facing -= 1;
                        }
                    },
                    Direction::Right => position.facing = (position.facing + 1) % 4,
                }
            },
        }
    }
    Some(position.score())
}

pub fn part_two(_input: &str) -> Option<u32> {
    //let (map, instructions) = input.split_once("\n\n").unwrap();
    //let instructions = parse_instructions(instructions);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
