use itertools::{FoldWhile, Itertools};

struct Map {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut map = Map {
            rows: Vec::new(),
            cols: Vec::new(),
        };

        for line in input.lines() {
            let mut row = Vec::new();
            for (index, char) in line.chars().enumerate() {
                if let Some(col) = map.cols.get_mut(index) {
                    col.push(char);
                } else {
                    map.cols.push(vec![char]);
                }

                row.push(char);
            }
            map.rows.push(row);
        }

        map
    }

    fn width(&self) -> usize {
        self.cols.len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn get_dirs_at(&self, x: usize, y: usize) -> [(bool, &[char]); 4] {
        let (up, down) = self.cols.get(x).unwrap().split_at(y);
        let (left, right) = self.rows.get(y).unwrap().split_at(x);

        [
            (true, up),
            (true, left),
            (false, &down[1..]),
            (false, &right[1..]),
        ]
    }

    fn get(&self, x: usize, y: usize) -> &char {
        self.rows.get(y).unwrap().get(x).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::parse(input);
    let width = map.width();
    let height = map.height();

    let mut count = width * 2 + (height - 2) * 2;

    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let value = map.get(x, y);
            if map
                .get_dirs_at(x, y)
                .into_iter()
                .any(|dir| dir.1.iter().all(|&c| c < *value))
            {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::parse(input);
    let width = map.width();
    let height = map.height();

    let mut max = 0;

    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let value = map.get(x, y);
            let v = map.get_dirs_at(x, y).into_iter().fold(1, |acc, dir| {
                acc * match dir.0 {
                    true => dir
                        .1
                        .iter()
                        .rev()
                        .fold_while(1, |acc, c| {
                            if c >= value {
                                FoldWhile::Done(acc)
                            } else {
                                FoldWhile::Continue(acc + 1)
                            }
                        })
                        .into_inner(),
                    false => dir
                        .1
                        .iter()
                        .fold_while(1, |acc, c| {
                            if c >= value {
                                FoldWhile::Done(acc)
                            } else {
                                FoldWhile::Continue(acc + 1)
                            }
                        })
                        .into_inner(),
                }
            });
            if max < v {
                max = v;
            }
        }
    }

    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
