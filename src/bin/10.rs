use std::collections::VecDeque;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        let mut split = line.split(' ');
        match split.next().unwrap() {
            "addx" => Self::Addx(
                split
                    .next()
                    .expect("Should contain a number")
                    .parse()
                    .expect("Invalid number"),
            ),
            "noop" => Self::Noop,
            _ => Self::Noop,
        }
    }
}

struct Cpu {
    cycle: u32,
    instructions: VecDeque<Instruction>,
    current: Option<Instruction>,
    current_cycle: Option<u32>,
    register_x: i32,
    instruction_finished: bool,
}

impl Cpu {
    fn new(instructions: VecDeque<Instruction>) -> Cpu {
        Cpu {
            cycle: 0,
            instructions,
            current: None,
            current_cycle: None,
            register_x: 1,
            instruction_finished: false,
        }
    }

    fn tick(&mut self) {
        if self.instruction_finished && self.current.is_some() {
            match self.current.as_ref().unwrap() {
                Instruction::Noop => {}
                Instruction::Addx(x) => self.register_x += x,
            }
            self.current_cycle = None;
            self.current = None;
            self.instruction_finished = false;
        }

        self.cycle += 1;
        if self.current.is_none() {
            self.current = self.instructions.pop_front();
        }

        if let Some(ins) = &self.current {
            match ins {
                Instruction::Noop => self.instruction_finished = true,
                Instruction::Addx(_) => {
                    if self.current_cycle.is_none() {
                        self.current_cycle = Some(2);
                    }
                    if let Some(c) = &mut self.current_cycle {
                        *c -= 1;
                        if *c == 0 {
                            self.instruction_finished = true
                        }
                    }
                }
            }
        }
    }
}

struct Crt {
    screen: String,
}

impl Crt {
    fn new() -> Crt {
        Crt {
            screen: String::with_capacity(40 * 6 + 6),
        }
    }

    fn tick(&mut self, register_x: i32, cycle: u32) {
        let x = (cycle - 1) as i32 % 40;
        if x == register_x - 1 || x == register_x || x == register_x + 1 {
            self.screen.push('#')
        } else {
            self.screen.push('.')
        }

        if cycle % 40 == 0 {
            self.screen.push('\n')
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = input.lines().map(Instruction::from_line).collect();
    let mut cpu = Cpu::new(instructions);
    let mut sum = 0;

    while cpu.cycle < 220 {
        cpu.tick();
        if (cpu.cycle + 20) % 40 == 0 {
            let result = cpu.register_x * cpu.cycle as i32;
            sum += result;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = input.lines().map(Instruction::from_line).collect();
    let mut cpu = Cpu::new(instructions);
    let mut crt = Crt::new();

    while cpu.cycle < 240 {
        cpu.tick();
        crt.tick(cpu.register_x, cpu.cycle);
    }

    Some(crt.screen)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n";

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let part_two = part_two(&input);
        if let Some(str) = part_two {
            assert_eq!(str, RESULT);
        }
    }
}
