use itertools::Itertools;

fn from_char(digit: char) -> i64 {
    match digit {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn to_char(digit: i64) -> char {
    match digit {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '=',
        4 => '-',
        _ => unreachable!(),
    }
}

fn to_snafu(number: i64) -> String {
    let mut number = number;

    let mut result = String::new();

    while number != 0 {
        let digit = number % 5;
        number /= 5;

        if digit > 2 {
            number += 1;
        }

        result.push(to_char(digit));
    }

    result.chars().rev().join("")
}

fn from_snafu(number: &str) -> i64 {
    let mut start = 5_i64.pow(number.len() as u32 - 1);
    let mut result = 0;

    for digit in number.chars() {
        let value = from_char(digit);
        result += value * start;
        start /= 5;
    }

    result
}

pub fn part_one(input: &str) -> Option<String> {
    let sum = input.lines().map(from_snafu).sum();

    let snafu = to_snafu(sum);

    Some(snafu)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test_case("1" => 1)]
    #[test_case("2" => 2)]
    #[test_case("1=" => 3)]
    #[test_case("1-" => 4)]
    #[test_case("10" => 5)]
    #[test_case("11" => 6)]
    #[test_case("12" => 7)]
    #[test_case("2=" => 8)]
    #[test_case("2-" => 9)]
    #[test_case("20" => 10)]
    #[test_case("1=0" => 15)]
    #[test_case("1-0" => 20)]
    #[test_case("1=11-2" => 2022)]
    #[test_case("1-0---0" => 12345)]
    #[test_case("1121-1110-1=0" => 314159265)]
    fn from_snafu_test(snafu: &str) -> i64 {
        from_snafu(snafu)
    }

    #[test_case(1 => "1")]
    #[test_case(2 => "2")]
    #[test_case(3 => "1=")]
    #[test_case(4 => "1-")]
    #[test_case(5 => "10")]
    #[test_case(6 => "11")]
    #[test_case(7 => "12")]
    #[test_case(8 => "2=")]
    #[test_case(9 => "2-")]
    #[test_case(10 => "20")]
    #[test_case(15 => "1=0")]
    #[test_case(20 => "1-0")]
    #[test_case(2022 => "1=11-2")]
    #[test_case(12345 => "1-0---0")]
    #[test_case(314159265 => "1121-1110-1=0")]
    fn to_snafu_test(number: i64) -> String {
        to_snafu(number)
    }
}
