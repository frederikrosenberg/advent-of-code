use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn parse(input: &str) -> HashSet<(i32, i32, i32)> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn get_sides((x, y, z): (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

pub fn part_one(input: &str) -> Option<i32> {
    let lava = parse(input);

    let mut sides = 0;

    for block in lava.iter() {
        for side in get_sides(*block).iter() {
            if !lava.contains(side) {
                sides += 1;
            }
        }
    }

    Some(sides)
}

type Position = (i32, i32, i32);

fn is_air(
    lava: &HashSet<Position>,
    known_air: &mut HashSet<Position>,
    known_enclosed: &mut HashSet<Position>,
    position: Position,
    target: Position,
) -> bool {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut done = HashSet::new();

    queue.push_back(position);
    seen.insert(position);

    let mut is_air = false;
    let mut should_break = false;

    while let Some(pos) = queue.pop_front() {
        done.insert(pos);

        if pos == target {
            is_air = true;
            break;
        }

        for side in get_sides(pos) {
            if seen.contains(&side) || lava.contains(&side) {
                continue;
            }
            if known_air.contains(&side) {
                is_air = true;
                should_break = true;
                break;
            }

            if known_enclosed.contains(&side) {
                should_break = true;
                break;
            }

            seen.insert(side);
            queue.push_back(side);
        }

        if should_break {
            break;
        }
    }

    if is_air {
        known_air.extend(done);
    } else {
        known_enclosed.extend(done);
    }

    is_air
}

pub fn part_two(input: &str) -> Option<i32> {
    let lava = parse(input);
    let mut sides = 0;
    // Could set a more optimized target pos, but this works.
    let target = (-1, -1, -1);

    let mut known_air = HashSet::new();
    let mut known_enclosed = HashSet::new();

    for block in lava.iter() {
        for side in get_sides(*block) {
            if !lava.contains(&side)
                && is_air(&lava, &mut known_air, &mut known_enclosed, side, target)
            {
                sides += 1;
            }
        }
    }

    Some(sides)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
