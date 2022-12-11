use std::{collections::VecDeque, slice::from_raw_parts_mut};

use itertools::Itertools;

struct Monkey {
    inspected: u64,
    items: VecDeque<u64>,
    operation: fn(u64, u64) -> u64,
    operation_const: Option<u64>,
    test_div_by: u64,
    test_true: usize,
    test_false: usize,
}

fn parse_starting_items(items: &str) -> VecDeque<u64> {
    let (_, items) = items.split_at("  Starting items: ".len());
    let items = items.split(", ");
    items.filter_map(|i| i.parse().ok()).collect()
}

fn parse_operation(op: &str) -> (fn(u64, u64) -> u64, Option<u64>) {
    let (_, op) = op.split_at("  Operation: new = old ".len());
    let (op, val) = op.split_once(' ').expect("Invalid operation!");
    let val = val.parse::<u64>().ok();
    let operation = match op {
        "+" => |old, val| old + val,
        "-" => |old, val| old - val,
        "*" => |old, val| old * val,
        "/" => |old, val| old / val,
        _ => unreachable!("Expected a operation, got {}", op),
    };

    (operation, val)
}

fn parse_div(div: &str) -> u64 {
    let (_, div) = div.split_at("  Test: divisible by ".len());
    div.parse().expect("Expected a number!")
}

fn parse_condition(con: &str) -> usize {
    con.chars()
        .last()
        .unwrap()
        .to_digit(10)
        .expect("Expected a number!") as usize
}

impl Monkey {
    fn parse<'a>(mut lines: impl Iterator<Item = &'a str>) -> Monkey {
        lines.next();
        let items = parse_starting_items(lines.next().unwrap());
        let (operation, operation_const) = parse_operation(lines.next().unwrap());
        let test_div_by = parse_div(lines.next().unwrap());
        let test_true = parse_condition(lines.next().unwrap());
        let test_false = parse_condition(lines.next().unwrap());

        Monkey {
            inspected: 0,
            items,
            operation,
            operation_const,
            test_div_by,
            test_true,
            test_false,
        }
    }

    fn round(&mut self, t_monkey: &mut Monkey, f_monkey: &mut Monkey, modulus: Option<u64>) {
        while let Some(mut item) = self.items.pop_front() {
            self.inspected += 1;
            if let Some(val) = self.operation_const {
                item = (self.operation)(item, val);
            } else {
                item = (self.operation)(item, item);
            }

            if let Some(modolus) = modulus {
                item %= modolus;
            } else {
                item /= 3;
            }

            if item % self.test_div_by == 0 {
                t_monkey.items.push_back(item);
            } else {
                f_monkey.items.push_back(item);
            }
        }
    }
}

#[inline]
fn get_monkeys(
    monkeyes: &mut Vec<Monkey>,
    index: usize,
) -> (&mut Monkey, &mut Monkey, &mut Monkey) {
    unsafe {
        assert!(index < monkeyes.len());

        let monkey = &mut from_raw_parts_mut(monkeyes.as_mut_ptr().add(index), 1)[0];
        assert!(monkey.test_true < monkeyes.len());
        assert!(monkey.test_false < monkeyes.len());
        assert!(monkey.test_true != index);
        assert!(monkey.test_false != index);
        assert!(monkey.test_true != monkey.test_false);
        let t_monkey = &mut from_raw_parts_mut(monkeyes.as_mut_ptr().add(monkey.test_true), 1)[0];
        let f_monkey = &mut from_raw_parts_mut(monkeyes.as_mut_ptr().add(monkey.test_false), 1)[0];

        (monkey, t_monkey, f_monkey)
    }
}

fn solve(input: &str, rounds: u64, is_part_two: bool) -> Option<u64> {
    let mut monkeyes: Vec<_> = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(Monkey::parse)
        .collect();
    let modulus = if is_part_two {
        Some(monkeyes.iter().map(|m| m.test_div_by).unique().product())
    } else {
        None
    };

    for _round in 0..rounds {
        for i in 0..monkeyes.len() {
            let (monkey, t_monkey, f_monkey) = get_monkeys(&mut monkeyes, i);

            monkey.round(t_monkey, f_monkey, modulus);
        }
    }

    let (one, two) = monkeyes
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .next_tuple()
        .unwrap();

    Some(one * two)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 20, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 10_000, true)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2_713_310_158));
    }
}
