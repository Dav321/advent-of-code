use crate::day::Day;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
}

impl Machine {
    pub fn solve0(&self) -> u32 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([(0, 0)]);
        while let Some((num, presses)) = queue.pop_front() {
            let presses = presses + 1;
            for button in &self.buttons {
                let num = num ^ button;
                if visited.contains(&num) {
                    continue;
                }
                if num == self.lights {
                    return presses;
                }
                visited.insert(num);
                queue.push_back((num, presses));
            }
        }
        unreachable!()
    }
}

pub struct Day10 {
    machines: Vec<Machine>,
}

impl Day for Day10 {
    fn new(input: &str) -> Self {
        let machines = input
            .lines()
            .map(|l| {
                let (lights, rest) = l.split_once(' ').unwrap();
                let (buttons, joltage) = rest.rsplit_once(' ').unwrap();

                let lights =
                    lights
                        .trim_matches(&['[', ']'])
                        .chars()
                        .rev()
                        .fold(0, |acc: u32, c| {
                            let mut acc = acc << 1;
                            if c == '#' {
                                acc |= 1;
                            }
                            acc
                        });

                let buttons_indices: Vec<Vec<usize>> = buttons
                    .split(' ')
                    .map(|s| {
                        s.trim_matches(&['(', ')'])
                            .split(',')
                            .map(|s| usize::from_str(s).unwrap())
                            .collect()
                    })
                    .collect();

                let buttons = buttons_indices.iter()
                    .map(|n| n.iter().fold(0, |acc, i| acc | (1u32 << i)))
                    .collect();


                let joltage = joltage
                    .trim_matches(&['{', '}'])
                    .split(',')
                    .map(|s| u32::from_str(s).unwrap())
                    .collect();

                Machine {
                    lights,
                    buttons,
                    joltage,
                }
            })
            .collect::<Vec<Machine>>();

        Self { machines }
    }

    fn solve0(&self) -> i64 {
        self.machines.iter().map(Machine::solve0).sum::<u32>() as i64
    }

    fn solve1(&self) -> i64 {
        0
    }
}
