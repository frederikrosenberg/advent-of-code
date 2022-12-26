use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Minus,
    Plus,
    Mult,
    Division,
}

impl Operation {
    fn from_char(operation: &str) -> Operation {
        match operation {
            "+" => Operation::Plus,
            "-" => Operation::Minus,
            "*" => Operation::Mult,
            "/" => Operation::Division,
            _ => unreachable!(),
        }
    }

    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Operation::Minus => left - right,
            Operation::Plus => left + right,
            Operation::Mult => left * right,
            Operation::Division => left / right,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Minus => write!(f, "-"),
            Operation::Plus => write!(f, "+"),
            Operation::Mult => write!(f, "*"),
            Operation::Division => write!(f, "/"),
        }
    }
}

#[derive(Clone)]
enum Expression {
    Literal(usize),
    Expression(String, String, Operation),
}

#[derive(Debug)]
enum ExpressionTree {
    Literal(usize),
    Variable,
    Expression(Box<ExpressionTree>, Box<ExpressionTree>, Operation),
}

#[derive(Clone)]
struct Monkey {
    name: String,
    expression: Expression,
}

impl Expression {
    fn from_str(line: &str) -> (String, Monkey) {
        let (ident, expression) = line.split_once(": ").unwrap();

        let expression = if expression.len() == 11 {
            let (left, operation, right) = expression.split(' ').collect_tuple().unwrap();
            Expression::Expression(
                left.to_owned(),
                right.to_owned(),
                Operation::from_char(operation),
            )
        } else {
            Expression::Literal(expression.parse().unwrap())
        };

        (
            ident.to_owned(),
            Monkey {
                name: ident.to_owned(),
                expression,
            },
        )
    }
}

const START: &str = "root";
const ME: &str = "humn";

impl ExpressionTree {
    fn build_expression(name: &str, map: &HashMap<String, Monkey>) -> Self {
        if name == ME {
            return ExpressionTree::Variable;
        }

        let monkey = map.get(name).unwrap();

        if let Expression::Literal(value) = &monkey.expression {
            return ExpressionTree::Literal(*value);
        }

        if let Expression::Expression(left, right, operation) = &monkey.expression {
            let left = ExpressionTree::build_expression(left, map);
            let right = ExpressionTree::build_expression(right, map);

            if let (ExpressionTree::Literal(left), ExpressionTree::Literal(right)) = (&left, &right)
            {
                return ExpressionTree::Literal(operation.apply(*left, *right));
            }

            return ExpressionTree::Expression(Box::new(left), Box::new(right), *operation);
        }

        unreachable!();
    }

    fn is_variable(&self) -> bool {
        matches!(self, ExpressionTree::Variable)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = input
        .lines()
        .map(Expression::from_str)
        .collect::<HashMap<_, _>>();

    let mut stack = Vec::new();
    stack.push(map[START].name.clone());

    while let Some(name) = stack.pop() {
        let monkey = map.get(&name).unwrap();
        if let Expression::Expression(left, right, operation) = &monkey.expression {
            if let (Expression::Literal(left), Expression::Literal(right)) =
                (&map[left].expression, &map[right].expression)
            {
                let result = operation.apply(*left, *right);
                map.get_mut(&name).unwrap().expression = Expression::Literal(result);
            } else {
                stack.push(name);
                stack.push(left.to_string());
                stack.push(right.to_string());
            }
        }
    }

    if let Expression::Literal(result) = &map[START].expression {
        Some(*result)
    } else {
        None
    }
}

fn unapply(result: usize, expression: ExpressionTree) -> (usize, ExpressionTree) {
    if expression.is_variable() {
        return (result, expression);
    }

    if let ExpressionTree::Expression(left, right, op) = expression {
        let result = match (left.as_ref(), right.as_ref()) {
            (ExpressionTree::Literal(value), _) => {
                let result = match op {
                    Operation::Minus => value - result,
                    Operation::Plus => result - value,
                    Operation::Mult => result / value,
                    Operation::Division => value / result,
                };
                (result, *right)
            }
            (_, ExpressionTree::Literal(value)) => {
                let result = match op {
                    Operation::Minus => value + result,
                    Operation::Plus => result - value,
                    Operation::Mult => result / value,
                    Operation::Division => result * value,
                };
                (result, *left)
            }
            _ => unreachable!(),
        };

        return result;
    }

    unreachable!();
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(Expression::from_str)
        .collect::<HashMap<_, _>>();

    if let Expression::Expression(left, right, _) = &map[START].expression {
        let left = ExpressionTree::build_expression(left, &map);
        let right = ExpressionTree::build_expression(right, &map);

        let (mut result, mut expression) = if let ExpressionTree::Literal(l) = left {
            (l, right)
        } else if let ExpressionTree::Literal(r) = right {
            (r, left)
        } else {
            unreachable!()
        };

        while !expression.is_variable() {
            (result, expression) = unapply(result, expression);
        }

        return Some(result);
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
