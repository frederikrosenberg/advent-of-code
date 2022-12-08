use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let chars = input.lines().flat_map(|l| l.chars()).collect_vec();
    let mut visable = 0;

    for (index, char) in chars.iter().enumerate() {
        let x = index % width;
        let y = index / width;
        if y == 0 || x == 0 || x == width - 1 || y == height - 1 {
            visable += 1;
            continue;
        }

        let mut is_visable = true;
        for x1 in (0..x).rev() {
            if let Some(value) = chars.get(x1 + (y * width)) {
                if value >= char {
                    is_visable = false;
                    break;
                }
            }
        }
        if is_visable {
            visable += 1;
            continue;
        }

        let mut is_visable = true;
        for x2 in (x + 1)..width {
            if let Some(value) = chars.get(x2 + (y * width)) {
                if value >= char {
                    is_visable = false;
                    break;
                }
            }
        }
        if is_visable {
            visable += 1;
            continue;
        }

        let mut is_visable = true;
        for y1 in (0..y).rev() {
            if let Some(value) = chars.get(x + (y1 * width)) {
                if value >= char {
                    is_visable = false;
                    break;
                }
            }
        }
        if is_visable {
            visable += 1;
            continue;
        }

        let mut is_visable = true;
        for y2 in (y + 1)..height {
            if let Some(value) = chars.get(x + (y2 * width)) {
                if value >= char {
                    is_visable = false;
                    break;
                }
            }
        }
        if is_visable {
            visable += 1;
        }
    }

    Some(visable)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let chars = input.lines().flat_map(|l| l.chars()).collect_vec();
    let mut max = 0;

    for (index, char) in chars.iter().enumerate() {
        let x = index % width;
        let y = index / width;
        if y == 0 || x == 0 || x == width - 1 || y == height - 1 {
            continue;
        }

        let mut total = 1;

        let mut count = 0;
        for x1 in (0..x).rev() {
            if let Some(value) = chars.get(x1 + (y * width)) {
                count += 1;
                if value >= char {
                    break;
                }
            }
        }
        total *= count;

        let mut count = 0;
        for x2 in (x + 1)..width {
            if let Some(value) = chars.get(x2 + (y * width)) {
                count += 1;
                if value >= char {
                    break;
                }
            }
        }
        total *= count;

        let mut count = 0;
        for y1 in (0..y).rev() {
            if let Some(value) = chars.get(x + (y1 * width)) {
                count += 1;
                if value >= char {
                    break;
                }
            }
        }
        total *= count;

        let mut count = 0;
        for y2 in (y + 1)..height {
            if let Some(value) = chars.get(x + (y2 * width)) {
                count += 1;
                if value >= char {
                    break;
                }
            }
        }
        total *= count;

        if total > max {
            max = total;
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
