use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug)]
enum Signal {
    List { items: Vec<Signal> },
    Number { value: u32 },
}

impl Signal {
    fn from_line(line: &mut impl Iterator<Item = char>) -> Signal {
        let mut content = Vec::new();

        let mut should_break = false;
        let mut parsing_number = false;
        let mut number = 0;
        while let Some(char) = line.next() {
            let s = match char {
                '[' => Signal::from_line(line),
                ']' => {
                    if !parsing_number {
                        break;
                    }

                    parsing_number = false;
                    should_break = true;
                    Signal::Number { value: number }
                }
                c if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap();
                    if !parsing_number {
                        parsing_number = true;
                        number = digit;
                        continue;
                    }

                    number *= 10;
                    number += digit;

                    continue;
                }
                _ => {
                    if !parsing_number {
                        continue;
                    }

                    parsing_number = false;
                    Signal::Number { value: number }
                }
            };

            content.push(s);
            if should_break {
                break;
            }
        }

        Signal::List { items: content }
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        let com = self.partial_cmp(other);
        if let Some(result) = com {
            return result == Ordering::Equal;
        }
        false
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Signal::List { items: self_items }, Signal::List { items: other_items }) => {
                let mut self_iter = self_items.iter();
                let mut other_iter = other_items.iter();
                while let (Some(s), Some(o)) = (self_iter.next(), other_iter.next()) {
                    if let Some(result) = s.partial_cmp(o) {
                        if result != Ordering::Equal {
                            return Some(result);
                        }
                    }
                }

                self_items.len().partial_cmp(&other_items.len())
            }
            (Signal::List { .. }, Signal::Number { value }) => self.partial_cmp(&Signal::List {
                items: vec![Signal::Number { value: *value }],
            }),
            (Signal::Number { value }, Signal::List { .. }) => Signal::List {
                items: vec![Signal::Number { value: *value }],
            }
            .partial_cmp(other),
            (Signal::Number { value: self_value }, Signal::Number { value: other_value }) => {
                self_value.partial_cmp(other_value)
            }
        }
    }
}

impl Eq for Signal {}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let signals = input
        .split("\n\n")
        .filter_map(|s| s.split_once('\n'))
        .map(|(left, right)| {
            (
                Signal::from_line(&mut left.chars().skip(1)),
                Signal::from_line(&mut right.chars().skip(1)),
            )
        });

    println!("Parsed input");

    let result = signals
        .filter_map(|(left, right)| left.partial_cmp(&right))
        .positions(|o| o == Ordering::Less)
        .map(|p| p + 1)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let div_1 = Signal::from_line(&mut "[[2]]".chars().skip(1));
    let div_2 = Signal::from_line(&mut "[[6]]".chars().skip(1));

    let mut signals = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Signal::from_line(&mut l.chars().skip(1)))
        .collect_vec();
    signals.push(div_1);
    signals.push(div_2);

    signals.sort_unstable();

    let div_1 = Signal::from_line(&mut "[[2]]".chars().skip(1));
    let div_2 = Signal::from_line(&mut "[[6]]".chars().skip(1));

    let result = signals
        .iter()
        .positions(|s| *s == div_1 || *s == div_2)
        .map(|p| p + 1)
        .product();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
