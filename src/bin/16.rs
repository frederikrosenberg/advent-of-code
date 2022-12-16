use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug)]
struct Room {
    name: String,
    tunnels: Vec<String>,
    rate: u32,
}

#[derive(Debug)]
struct Tunnel {
    to: String,
    distance: u32,
    reward: u32,
}

type Rooms = HashMap<String, Room>;

impl Room {
    fn from_line(line: &str) -> Room {
        let line = line.trim_start_matches("Valve ");
        let (name, line) = line.split_at(2);
        let line = line.trim_start_matches(" has flow rate=");
        let (rate, line) = line.split_once(';').unwrap();
        let line = line.trim_start_matches(" tunnels lead to valves ");
        let line = line.trim_start_matches(" tunnel leads to valve ");
        let tunnels = line.split(", ").map(|t| t.to_owned()).collect_vec();

        Room {
            name: name.to_owned(),
            tunnels,
            rate: rate.parse().unwrap(),
        }
    }
}

fn parse_rooms(input: &str) -> Rooms {
    let lines = input.lines().sorted().collect_vec();

    lines
        .iter()
        .map(|l| (l[6..8].to_owned(), Room::from_line(l)))
        .collect()
}

fn calculate_rooms_neighbors(rooms: &Rooms) -> HashMap<String, Vec<Tunnel>> {
    rooms
        .iter()
        .map(|(name, room)| (name.to_owned(), calculate_room(rooms, room)))
        .collect()
}

fn calculate_room(rooms: &Rooms, room: &Room) -> Vec<Tunnel> {
    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();
    seen.insert(
        &room.name,
        Tunnel {
            to: room.name.to_owned(),
            reward: room.rate,
            distance: 0,
        },
    );

    queue.push_back((2, &room.name));

    while let Some((count, name)) = queue.pop_front() {
        let room = &rooms[name];

        for tunnel in room.tunnels.iter() {
            if !seen.contains_key(tunnel) {
                seen.insert(
                    tunnel,
                    Tunnel {
                        to: tunnel.to_owned(),
                        distance: count,
                        reward: rooms[tunnel].rate,
                    },
                );
                queue.push_back((count + 1, tunnel))
            }
        }
    }

    seen.into_values()
        .filter(|t| t.distance > 0 && t.reward > 0)
        .collect()
}

fn test(
    rooms: &HashMap<String, Vec<Tunnel>>,
    seen: &mut HashSet<String>,
    rate: u32,
    total: u32,
    time: u32,
    room: &str,
) -> u32 {
    let mut max = 0;

    let tunnels = rooms[room]
        .iter()
        .filter(|r| !seen.contains(r.to.as_str()))
        .collect_vec();
    if tunnels.is_empty() {
        max = total + (30 - time) * rate;
    }
    for tunnel in tunnels {
        if time + tunnel.distance > 30 {
            let result = total + (30 - time) * rate;
            if result > max {
                max = result;
            }

            continue;
        }
        let time = time + tunnel.distance;
        let total = total + rate * tunnel.distance;
        let rate = rate + tunnel.reward;
        let room = &tunnel.to;

        seen.insert(room.to_owned());

        let result = test(rooms, seen, rate, total, time, room);
        if result > max {
            max = result;
        }

        seen.remove(room);
    }

    max
}

fn test_part2_start(rooms: &HashMap<String, Vec<Tunnel>>) -> u32 {
    let mut seen = HashSet::new();

    let tunnels = &rooms["AA"];
    let mut max = 0;

    for (i, tunnel1) in tunnels[..tunnels.len() - 1].iter().enumerate() {
        println!("Outer");
        for tunnel2 in &tunnels[i + 1..] {
            println!("Inner");
            seen.insert(tunnel1.to.to_owned());
            seen.insert(tunnel2.to.to_owned());

            let (array, time, mut reward) = next_array(tunnel1, tunnel2);

            if array[0].distance == array[1].distance {
                reward = array[0].reward + array[1].reward;
            }

            let result = test_part2(rooms, &mut seen, reward, 0, time, array);
            if result > max {
                max = result;
            }

            seen.remove(&tunnel1.to);
            seen.remove(&tunnel2.to);
        }
    }

    max
}

fn next_array(tunnel1: &Tunnel, tunnel2: &Tunnel) -> ([Tunnel; 2], u32, u32) {
    let dis1: u32;
    let dis2: u32;
    let time: u32;
    let reward: u32;

    if tunnel1.distance < tunnel2.distance {
        dis1 = 0;
        dis2 = tunnel2.distance - tunnel1.distance;
        time = tunnel1.distance;
        reward = tunnel1.reward;
    } else {
        dis1 = tunnel1.distance - tunnel2.distance;
        dis2 = 0;
        time = tunnel2.distance;
        reward = tunnel2.reward;
    }

    (
        [
            Tunnel {
                distance: dis1,
                to: tunnel1.to.to_owned(),
                reward: tunnel1.reward,
            },
            Tunnel {
                distance: dis2,
                to: tunnel2.to.to_owned(),
                reward: tunnel2.reward,
            },
        ],
        time,
        reward,
    )
}

fn test_part2(
    rooms: &HashMap<String, Vec<Tunnel>>,
    seen: &mut HashSet<String>,
    rate: u32,
    total: u32,
    time: u32,
    next: [Tunnel; 2],
) -> u32 {
    let mut max = 0;

    if next[0].distance == next[1].distance {
        let room0 = next[0].to.as_str();
        let room1 = next[1].to.as_str();
        let tunnels0 = rooms[room0]
            .iter()
            .filter(|r| !seen.contains(r.to.as_str()))
            .collect_vec();
        let tunnels1 = rooms[room1]
            .iter()
            .filter(|r| !seen.contains(r.to.as_str()))
            .collect_vec();

        for tunnel1 in &tunnels0 {
            for tunnel2 in &tunnels1 {
                if tunnel1.to == tunnel2.to {
                    continue;
                }

                let (array, offset, mut reward) = next_array(tunnel1, tunnel2);
                if time + offset > 26 {
                    let result = total + (26 - time) * rate;
                    if result > max {
                        max = result;
                    }

                    continue;
                }

                if array[0].distance == array[1].distance {
                    reward = array[0].reward + array[1].reward;
                }
                let time = time + offset;
                let total = total + rate * offset;
                let rate = rate + reward;

                seen.insert(tunnel1.to.to_owned());
                seen.insert(tunnel2.to.to_owned());

                let result = test_part2(rooms, seen, rate, total, time, array);
                if result > max {
                    max = result;
                }

                seen.remove(&tunnel1.to);
                seen.remove(&tunnel2.to);
            }
        }
    } else {
        let room: &str;
        let other: &Tunnel;
        if next[0].distance == 0 {
            room = next[0].to.as_str();
            other = &next[1];
        } else {
            room = next[1].to.as_str();
            other = &next[0];
        }
        let tunnels = rooms[room]
            .iter()
            .filter(|r| !seen.contains(r.to.as_str()))
            .collect_vec();
        if tunnels.is_empty() {
            let total = total + rate * other.distance;
            let rate = rate + other.reward;
            let time = time + other.distance;

            return total + (26 - time) * rate;
        }
        for tunnel in tunnels {
            let (array, offset, mut reward) = next_array(tunnel, other);
            if time + offset > 26 {
                let result = total + (26 - time) * rate;
                if result > max {
                    max = result;
                }

                continue;
            }
            if array[0].distance == array[1].distance {
                reward = array[0].reward + array[1].reward;
            }
            seen.insert(tunnel.to.to_owned());

            let time = time + offset;
            let total = total + rate * offset;
            let rate = rate + reward;

            let result = test_part2(rooms, seen, rate, total, time, array);
            if result > max {
                max = result;
            }

            seen.remove(&tunnel.to);
        }
    }

    max
}

pub fn part_one(input: &str) -> Option<u32> {
    let rooms = parse_rooms(input);
    let rooms_n = calculate_rooms_neighbors(&rooms);

    let mut seen = HashSet::new();
    let max = test(&rooms_n, &mut seen, 0, 0, 0, "AA");

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rooms = parse_rooms(input);
    let rooms_n = calculate_rooms_neighbors(&rooms);

    let max = test_part2_start(&rooms_n);

    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
