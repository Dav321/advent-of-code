use crate::day::Day;
use std::collections::HashMap;

pub struct Day11 {
    graph: HashMap<u32, Vec<u32>>,
}

impl Day for Day11 {
    fn new(input: &str) -> Self {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        for line in input.lines() {
            let (node, children) = line.split_once(':').unwrap();
            let node = Self::name_to_u32(node);
            let children = children
                .split(' ')
                .skip(1)
                .map(|child| Self::name_to_u32(child))
                .collect();
            graph.insert(node, children);
        }

        Self { graph }
    }

    fn solve0(&self) -> i64 {
        let you = Self::name_to_u32("you");
        let out = Self::name_to_u32("out");

        let mut cache = HashMap::new();

        self.dfs(you, out, &mut cache)
    }

    fn solve1(&self) -> i64 {
        0
    }
}

impl Day11 {
    fn dfs(&self, node: u32, out: u32, cache: &mut HashMap<u32, i64>) -> i64 {
        if let Some(&res) = cache.get(&node) {
            return res;
        }
        if node == out {
            return 1;
        }
        let mut res = 0;
        for child in self.graph.get(&node).unwrap_or(&vec![]) {
            res += self.dfs(*child, out, cache);
        }
        cache.insert(node, res);
        res
    }

    fn name_to_u32(name: &str) -> u32 {
        name.chars().fold(0, |acc, c| {
            let acc = acc << 8;
            acc + (c as u8) as u32
        })
    }

    fn u32_to_name(num: u32) -> String {
        let mut chars = [0; 3];
        for i in 0..3 {
            chars[2 - i] = ((num >> (i * 8)) & 0xFF) as u8;
        }
        str::from_utf8(chars.as_slice()).unwrap().to_string()
    }
}
