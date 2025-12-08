use crate::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day7 {
    map: Vec<Vec<bool>>,
}

impl Day for Day7 {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|s| s.chars().map(|c| c != '.').collect::<Vec<_>>())
            // Filter empty lines
            .filter(|v| v.iter().any(|b| *b))
            .collect();

        Self { map }
    }

    fn solve0(&self) -> i64 {
        let map = self.map.clone();
        let start = map.first().unwrap().iter().position(|b| *b).unwrap();

        let mut res = 0;
        let mut positions = HashSet::from([start]);
        for line in map.iter().skip(1) {
            for pos in positions.clone() {
                if line[pos] {
                    res += 1;
                    positions.remove(&pos);
                    // no clamping needed because of gap in puzzle input
                    positions.insert(pos - 1);
                    positions.insert(pos + 1);
                }
            }
        }

        res
    }

    fn solve1(&self) -> i64 {
        let map = self.map.clone();
        let start = map.first().unwrap().iter().position(|b| *b).unwrap();

        let mut positions = HashMap::from([(start, 1)]);
        for line in map.iter().skip(1) {
            for (pos, n) in positions.clone() {
                if line[pos] {
                    positions.remove(&pos);
                    // no clamping needed because of gap in puzzle input
                    positions.insert(pos - 1, n + positions.get(&(pos - 1)).unwrap_or(&0));
                    positions.insert(pos + 1, n + positions.get(&(pos + 1)).unwrap_or(&0));
                }
            }
        }

        positions.values().sum()
    }
}
