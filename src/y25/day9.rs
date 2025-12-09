use crate::day::Day;
use std::str::FromStr;

pub struct Day9 {
    tiles: Vec<(i64, i64)>,
}

impl Day for Day9 {
    fn new(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (i64::from_str(x).unwrap(), i64::from_str(y).unwrap()))
            .collect();
        Self { tiles }
    }

    fn solve0(&self) -> i64 {
        let mut largest_area = 0;
        for i in 0..self.tiles.len() {
            for j in (i + 1)..self.tiles.len() {
                let width = self.tiles[i].0 - self.tiles[j].0;
                let width = width.abs() + 1;
                let height = self.tiles[i].1 - self.tiles[j].1;
                let height = height.abs() + 1;
                let area = width * height;
                if area > largest_area {
                    largest_area = area;
                }
            }
        }

        largest_area
    }

    fn solve1(&self) -> i64 {
        0
    }
}
