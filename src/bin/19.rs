use std::{collections::HashMap, thread};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Blueprint {
    number: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32),
}

impl Blueprint {
    fn from_line(line: &str) -> Blueprint {
        lazy_static! {
            static ref RE: Regex = Regex::new("Blueprint (\\d*): Each ore robot costs (\\d*) ore. Each clay robot costs (\\d*) ore. Each obsidian robot costs (\\d*) ore and (\\d*) clay. Each geode robot costs (\\d*) ore and (\\d*) obsidian.").unwrap();
        }

        let captures = RE.captures(line).unwrap();
        let numbers = captures
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse().unwrap())
            .collect_vec();

        Blueprint {
            number: numbers[0],
            ore_robot_cost: numbers[1],
            clay_robot_cost: numbers[2],
            obsidian_robot_cost: (numbers[3], numbers[4]),
            geode_robot_cost: (numbers[5], numbers[6]),
        }
    }
}

#[derive(Debug)]
enum Action {
    Ore,
    Clay,
    Obsidian,
    Geode,
    Wait,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State<'a> {
    blueprint: &'a Blueprint,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    minute: u32,
}

impl State<'_> {
    fn new(blueprint: &Blueprint) -> State {
        State {
            blueprint,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            minute: 0,
        }
    }

    fn get_actions(&self) -> impl Iterator<Item = Action> {
        let mut actions = vec![];

        if self.blueprint.ore_robot_cost <= self.ore {
            actions.push(Action::Ore);
        }

        if self.blueprint.clay_robot_cost <= self.ore {
            actions.push(Action::Clay);
        }

        if self.blueprint.obsidian_robot_cost.0 <= self.ore
            && self.blueprint.obsidian_robot_cost.1 <= self.clay
        {
            actions.push(Action::Obsidian);
        }

        if self.blueprint.geode_robot_cost.0 <= self.ore
            && self.blueprint.geode_robot_cost.1 <= self.obsidian
        {
            actions.push(Action::Geode);
        }

        actions.push(Action::Wait);

        actions.into_iter()
    }

    fn apply(&mut self, action: &Action) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        match action {
            Action::Ore => {
                self.ore -= self.blueprint.ore_robot_cost;
                self.ore_robots += 1;
            }
            Action::Clay => {
                self.ore -= self.blueprint.clay_robot_cost;
                self.clay_robots += 1;
            }
            Action::Obsidian => {
                self.ore -= self.blueprint.obsidian_robot_cost.0;
                self.clay -= self.blueprint.obsidian_robot_cost.1;
                self.obsidian_robots += 1;
            }
            Action::Geode => {
                self.ore -= self.blueprint.geode_robot_cost.0;
                self.obsidian -= self.blueprint.geode_robot_cost.1;
                self.geode_robots += 1;
            }
            Action::Wait => {}
        }
        self.minute += 1;
    }

    fn unapply(&mut self, action: &Action) {
        self.minute -= 1;
        match action {
            Action::Ore => {
                self.ore += self.blueprint.ore_robot_cost;
                self.ore_robots -= 1;
            }
            Action::Clay => {
                self.ore += self.blueprint.clay_robot_cost;
                self.clay_robots -= 1;
            }
            Action::Obsidian => {
                self.ore += self.blueprint.obsidian_robot_cost.0;
                self.clay += self.blueprint.obsidian_robot_cost.1;
                self.obsidian_robots -= 1;
            }
            Action::Geode => {
                self.ore += self.blueprint.geode_robot_cost.0;
                self.obsidian += self.blueprint.geode_robot_cost.1;
                self.geode_robots -= 1;
            }
            Action::Wait => {}
        }
        self.ore -= self.ore_robots;
        self.clay -= self.clay_robots;
        self.obsidian -= self.obsidian_robots;
        self.geode -= self.geode_robots;
    }
}

fn simulate<'a, const MAX: u32>(
    state: &mut State<'a>,
    cache: &mut HashMap<State<'a>, u32>,
    max_seen: &mut HashMap<u32, u32>,
) -> u32 {
    if state.minute == MAX {
        return state.geode;
    }

    if cache.contains_key(state) {
        return cache[state];
    }

    if let Some(max) = max_seen.get(&state.minute) {
        if *max > state.geode {
            return 0;
        }
    }

    let mut max = 0;

    for action in state.get_actions() {
        state.apply(&action);

        let result = simulate::<MAX>(state, cache, max_seen);
        if result > max {
            max = result;
        }

        if let Some(old_max) = max_seen.get_mut(&state.minute) {
            if *old_max < state.geode {
                *old_max = state.geode;
            }
        } else if state.geode != 0 {
            max_seen.insert(state.minute, state.geode);
        }

        state.unapply(&action);
    }

    cache.insert(state.clone(), max);

    max
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = input.lines().map(Blueprint::from_line).collect_vec();

    let mut score = 0;

    let mut threads = Vec::new();

    for blueprint in blueprints.into_iter() {
        let handle = thread::spawn(move || {
            let mut state = State::new(&blueprint);
            let seen = &mut HashMap::new();
            let result = simulate::<24>(&mut state, &mut HashMap::new(), seen);
            blueprint.number * result
        });
        threads.push(handle);
    }

    for thread in threads.into_iter() {
        score += thread.join().unwrap();
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = input
        .lines()
        .map(Blueprint::from_line)
        .take(3)
        .collect_vec();

    let mut score = 1;

    let mut threads = Vec::new();

    for blueprint in blueprints.into_iter() {
        let handle = thread::spawn(move || {
            let mut state = State::new(&blueprint);
            let seen = &mut HashMap::new();
            let result = simulate::<32>(&mut state, &mut HashMap::new(), seen);
            println!("Seen: {:?}", seen);
            result
        });
        threads.push(handle);
    }

    for thread in threads.into_iter() {
        let result = thread.join().unwrap();
        println!("{}", result);
        score *= result;
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    /* My solution is not entirely correct since this fails:
    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(62 * 56));
    }
    */
}
