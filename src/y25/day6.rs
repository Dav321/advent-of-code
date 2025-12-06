use crate::day::Day;
use crate::y25::utils_2d::Map2d;

#[allow(dead_code)]
pub struct Day6 {
    numbers: Map2d<i64>,
    /// true -> +, false -> *
    operators: Vec<bool>,
}

#[allow(unused_variables)]
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

        Self { numbers, operators }
    }

    fn solve0(&self) -> i64 {
        let nums = self.numbers.inner();

        let mut res = 0;
        for i in 0..self.operators.len() {
            let iter = nums[i].iter();
            if self.operators[i] {
                res += iter.sum::<i64>();
            } else {
                res += iter.product::<i64>();
            }
        }

        res
    }

    fn solve1(&self) -> i64 {
        0
    }
}
