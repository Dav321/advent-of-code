use crate::day::Day;
use crate::y25::utils_2d::Map2d;
use std::str::FromStr;

pub struct Day6 {
    numbers: Map2d<i64>,
    /// true -> +, false -> *
    operators: Vec<bool>,
    char_map: Map2d<char>,
}

impl Day for Day6 {
    fn new(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let last = lines.len() - 1;
        let numbers = lines
            .iter()
            .take(last)
            .map(|l| {
                l.split(" ")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let numbers = Map2d::new(numbers).rotate();

        let operators = lines[last]
            .replace(" ", "")
            .chars()
            .map(|c| c == '+')
            .collect();

        let char_map = Map2d::new(lines.iter().map(|l| l.chars().collect()).collect()).rotate();

        Self {
            numbers,
            operators,
            char_map,
        }
    }

    fn solve0(&self) -> i64 {
        let nums = self.numbers.inner();

        let mut res = 0;
        for i in 0..self.operators.len() {
            res += Self::compute(&nums[i], self.operators[i]);
        }

        res
    }

    fn solve1(&self) -> i64 {
        let map = self.char_map.inner();

        let mut res = 0;
        let mut operator = false;
        let mut nums: Vec<i64> = vec![];

        let (x_max, y_max) = self.char_map.size();
        for x in 0..x_max {
            let mut digit = String::new();
            for y in 0..y_max {
                let c = map[x][y];
                match c {
                    '*' => operator = false,
                    '+' => operator = true,
                    ' ' => continue,
                    '0'..='9' => digit.push(c),
                    _ => panic!("Invalid char: {}", c),
                }
            }
            if digit.is_empty() {
                res += Self::compute(&nums, operator);
                nums.clear();
                continue;
            }

            nums.push(i64::from_str(digit.as_str()).unwrap());
        }

        res + Self::compute(&nums, operator)
    }
}

impl Day6 {
    fn compute(nums: &Vec<i64>, operator: bool) -> i64 {
        let iter = nums.iter();
        if operator {
            iter.sum::<i64>()
        } else {
            iter.product::<i64>()
        }
    }
}
