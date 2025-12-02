use crate::day::Day;
use std::collections::HashSet;
use std::str::FromStr;

#[allow(dead_code)]
pub struct Day2 {
    ranges: Vec<(i64, i64)>
}

#[allow(unused_variables)]
impl Day for Day2 {
    fn new(input: &str) -> Self {
        let ranges = input.split(",")
            .map(|s| s.split_once("-").unwrap())
            .map(|(x, y)|
                (i64::from_str(x).unwrap(), i64::from_str(y).unwrap()))
            .collect();
        Self {
            ranges
        }
    }

    fn solve0(&self) -> i64 {
        let mut res = 0;
        for (range_start, range_end) in self.ranges.clone() {
            let mut start_decimals = Self::decimal_count(range_start) / 2;
            let end_decimals = Self::decimal_count(range_end) / 2;

            if start_decimals != 0 {
                start_decimals -= 1;
            }

            let start = 10i64.pow(start_decimals);
            let end = 10i64.pow(end_decimals);

            for i in start..end {
                let x = (i * 10i64.pow(Self::decimal_count(i))) + i;
                if x < range_start || x > range_end {
                    continue;
                }
                res += x;
            }
        }

        res
    }

    fn solve1(&self) -> i64 {
        let mut res = HashSet::new();
        for (range_start, range_end) in self.ranges.clone() {
            let end_decimals = Self::decimal_count(range_end) / 2;
            let end = 10i64.pow(end_decimals);

            for i in 1..end {
                let mut repeat = 1;
                loop {
                    repeat += 1;
                    let x = Self::repeat(i, repeat);
                    if x < range_start {
                        continue;
                    } else if x > range_end {
                        break;
                    }
                    res.insert(x);
                }
            }
        }

        res.iter().sum()
    }
}

impl Day2 {
    fn decimal_count(n: i64) -> u32 {
        n.to_string().chars().count() as u32
    }

    fn repeat(n: i64, times: u32) -> i64 {
        let len = Self::decimal_count(n);

        let mut res = n;
        for i in 1..times {
            res += n * 10i64.pow(len * i);
        }
        res
    }
}