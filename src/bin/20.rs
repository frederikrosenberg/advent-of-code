use itertools::Itertools;

fn move_new(file: &mut Vec<(usize, i64)>, (index, number): (usize, i64)) {
    if number == 0 {
        return;
    }

    let len = file.len() as i64;
    let rotate = (number / len) % (len - 1);
    let mut actual_index = file.iter().position(|&n| n.0 == index).unwrap() as i64;

    if rotate != 0 {
        let element = file.remove(actual_index as usize);

        if rotate < 0 {
            file.rotate_right(rotate.unsigned_abs() as usize);
        } else {
            file.rotate_left(rotate as usize);
        }

        file.insert(actual_index as usize, element);
    }

    let new_position = actual_index + number % len;

    if new_position < actual_index {
        while actual_index != new_position {
            file.swap(
                ((actual_index - 1).rem_euclid(len)) as usize,
                (actual_index.rem_euclid(len)) as usize,
            );
            actual_index -= 1;
        }
    } else {
        while actual_index != new_position {
            file.swap(
                ((actual_index).rem_euclid(len)) as usize,
                ((actual_index + 1).rem_euclid(len)) as usize,
            );
            actual_index += 1;
        }
    }
}

fn _move_number(file: &mut Vec<(usize, i64)>, (index, number): (usize, i64)) {
    if number == 0 {
        return;
    }

    let len = file.len() as i64;

    let mut actual_index = file.iter().position(|&n| n.0 == index).unwrap() as i64;

    let new_position = actual_index + number;

    if new_position < actual_index {
        while actual_index != new_position {
            file.swap(
                ((actual_index - 1).rem_euclid(len)) as usize,
                (actual_index.rem_euclid(len)) as usize,
            );
            actual_index -= 1;
        }
    } else {
        while actual_index != new_position {
            file.swap(
                ((actual_index).rem_euclid(len)) as usize,
                ((actual_index + 1).rem_euclid(len)) as usize,
            );
            actual_index += 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let order: Vec<i64> = input.lines().filter_map(|l| l.parse().ok()).collect_vec();
    let mut file = order.clone().into_iter().enumerate().collect_vec();

    for pair in order.into_iter().enumerate() {
        move_new(&mut file, pair);
    }

    let index = file.iter().position(|&n| n.1 == 0).unwrap();

    let one = file[(index + 1000).rem_euclid(file.len())].1;
    let two = file[(index + 2000).rem_euclid(file.len())].1;
    let three = file[(index + 3000).rem_euclid(file.len())].1;

    println!("Len {}", file.len());

    println!("1000th {}", one);
    println!("2000th {}", two);
    println!("3000th {}", three);

    Some(one + two + three)
}

pub fn part_two(input: &str) -> Option<i64> {
    let order: Vec<i64> = input
        .lines()
        .filter_map(|l| l.parse().ok())
        .map(|n: i64| n * 811589153)
        .collect_vec();
    let mut file = order.clone().into_iter().enumerate().collect_vec();

    for _round in 0..10 {
        for (index, number) in order.iter().enumerate() {
            move_new(&mut file, (index, *number));
        }
    }

    let index = file.iter().position(|&n| n.1 == 0).unwrap();

    let one = file[(index + 1000).rem_euclid(file.len())].1;
    let two = file[(index + 2000).rem_euclid(file.len())].1;
    let three = file[(index + 3000).rem_euclid(file.len())].1;

    println!("Len {}", file.len());

    println!("1000th {}", one);
    println!("2000th {}", two);
    println!("3000th {}", three);

    Some(one + two + three)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn move_number_is_equal_to_faster() {
        let input = advent_of_code::read_file("examples", 20);
        let order: Vec<i64> = input.lines().filter_map(|l| l.parse().ok()).collect_vec();
        let mut file = order.clone().into_iter().enumerate().collect_vec();
        let mut expected = file.clone();

        for pair in order.into_iter().enumerate() {
            _move_number(&mut expected, pair);
            move_new(&mut file, pair);

            assert_eq!(file, expected, "After {} array should be:", pair.1);
        }
    }

    #[test]
    fn move_number_test_1() {
        let mut file = vec![(0, 1), (1, 2), (2, 4), (3, 3)];
        let mut actual = file.clone();

        _move_number(&mut file, (2, 4));
        move_new(&mut actual, (2, 4));
        assert_eq!(file, actual);
    }

    #[test]
    fn move_test() {
        let mut file = vec![
            (0, 0),
            (1, 5),
            (2, -18),
            (3, 100),
            (4, -310),
            (5, 2),
            (6, 7),
        ];
        let order = file.iter().clone().map(|f| f.1).enumerate().collect_vec();
        let mut file_actual = file.clone();
        for round in 0..49 {
            for (index, num) in order.iter() {
                println!("Moving {:?}", (index, num));
                _move_number(&mut file, (*index, *num));
                move_new(&mut file_actual, (*index, *num));
                assert_eq!(
                    file,
                    file_actual,
                    "Failed after round {} after moving {:?}",
                    round,
                    (index, num)
                );
            }
        }
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
