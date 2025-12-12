use crate::day::Day;
use std::array;

pub struct Day12 {
    presents: Vec<[[bool; 3]; 3]>,
    trees: Vec<((usize, usize), Vec<usize>)>,
}

impl Day for Day12 {
    fn new(input: &str) -> Self {
        let mut presents = Vec::new();
        let mut trees = Vec::new();

        let mut iter = input.split("\n\n");
        let mut section = "";

        let mut i = 0;
        loop {
            section = iter.next().unwrap().trim();
            if !section.starts_with(&i.to_string()) {
                break;
            }

            let mut lines = section.lines().skip(1);
            let present: [[bool; 3]; 3] = array::from_fn(|_| {
                let line = lines.next().unwrap();
                let mut chars = line.chars();
                array::from_fn(|_| chars.next().unwrap() == '#')
            });
            presents.push(present);
            i += 1;
        }

        for line in section.lines() {
            let (size, list) = line.split_once(": ").unwrap();
            let (x, y) = size.split_once("x").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();

            let list = list
                .split(' ')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            trees.push(((x, y), list));
        }

        Self { presents, trees }
    }

    fn solve0(&self) -> i64 {
        self.trees
            .iter()
            .map(|((x, y), presents)| ((x / 3) * (y / 3), presents.iter().sum::<usize>()))
            .filter(|(possible, presents)| presents <= possible)
            .count() as i64
    }

    fn solve1(&self) -> i64 {
        0
    }
}
