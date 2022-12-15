use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Scan {
    sensor: Point,
    beacon: Point,
    distance: i32,
}

impl Point {
    fn from(str: &str) -> Point {
        let (x, y) = str.split_once(',').unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

        Point { x, y }
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }
}

impl Scan {
    fn from_line(line: &str) -> Scan {
        let line = line
            .matches(|c: char| c.is_ascii_digit() || c == ',' || c == ':' || c == '-')
            .join("");
        let (sensor, beacon) = line.split_once(':').unwrap();
        let (sensor, beacon) = (Point::from(sensor), Point::from(beacon));

        let distance = sensor.manhattan_distance(&beacon);

        Scan {
            sensor,
            beacon,
            distance,
        }
    }

    fn is_within(&self, point: &Point) -> bool {
        self.sensor.manhattan_distance(point) <= self.distance
    }

    fn is_outside(&self, point: &Point) -> bool {
        self.sensor.manhattan_distance(point) > self.distance
    }

    fn edge_points(&self, min: &Point, max: &Point) -> Vec<Point> {
        let start_y = self.sensor.y - self.distance - 1;
        let end_y = self.sensor.y + self.distance + 1;

        let o_start_y = start_y.max(min.y);
        let o_end_y = end_y.min(max.y);
        let mut offset = 0;

        if start_y != o_start_y {
            if o_start_y <= self.sensor.y {
                offset = o_start_y - start_y + 1;
            } else {
                offset = o_end_y - o_start_y;
            }
        }

        let mut points = vec![];

        for y in start_y..=end_y {
            if min.y <= y && max.y >= y {
                if offset == 0 {
                    if min.x <= self.sensor.x && max.x >= self.sensor.x {
                        points.push(Point {
                            x: self.sensor.x,
                            y,
                        });
                    }
                } else {
                    let x_1 = self.sensor.x + offset;
                    let x_2 = self.sensor.x - offset;
                    if min.x <= x_1 && max.x >= x_1 {
                        points.push(Point { x: x_1, y });
                    }
                    if min.x <= x_2 && max.x >= x_2 {
                        points.push(Point { x: x_2, y });
                    }
                }
            }

            if y < self.sensor.y {
                offset += 1;
            } else {
                offset -= 1;
            }
        }

        points
    }
}

fn solve_part_one(input: &str, y: i32) -> Option<usize> {
    let scans = input.lines().map(Scan::from_line).collect_vec();

    let min_x = scans
        .iter()
        .map(|s| s.beacon.x.min(s.sensor.x - s.distance))
        .min()
        .unwrap();
    let max_x = scans
        .iter()
        .map(|s| s.beacon.x.max(s.sensor.x + s.distance))
        .max()
        .unwrap();

    let mut count = (min_x..=max_x)
        .map(|x| Point { x, y })
        .filter(|p| scans.iter().any(|s| s.is_within(p)))
        .count();

    count -= scans
        .iter()
        .map(|s| &s.beacon)
        .filter(|b| b.y == y)
        .unique()
        .count();

    Some(count)
}

fn solve_part_two(input: &str, max_size: u32) -> Option<usize> {
    const WIDTH: usize = 4000000;
    let scans = input.lines().map(Scan::from_line).collect_vec();

    let min = Point { x: 0, y: 0 };
    let max = Point {
        x: max_size as i32,
        y: max_size as i32,
    };

    let mut points = scans
        .iter()
        .flat_map(|s| s.edge_points(&min, &max));

    let point = points
        .find(|p| scans.iter().all(|s| s.is_outside(p))).unwrap();

    Some(point.x as usize * WIDTH + point.y as usize)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_part_one(input, 2000000)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_part_two(input, 4000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(solve_part_one(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(solve_part_two(&input, 20), Some(56000011));
    }
}
