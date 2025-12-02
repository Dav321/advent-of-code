use std::str::FromStr;
use crate::day::Day;

#[allow(dead_code)]
pub struct Day2 {

}

#[allow(unused_variables)]
impl Day for Day2 {
    fn solve0(input: &str) -> i64 {
        let ranges: Vec<(i64, i64)> = input.split(",")
            .map(|s| s.split_once("-").unwrap())
            .map(|(x, y)|
                (i64::from_str(x).unwrap(), i64::from_str(y).unwrap()))
            .collect();

        let mut res = 0;

        for (range_start, range_end) in ranges {
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

    fn solve1(input: &str) -> i64 {
        todo!()
    }
}

impl Day2 {
    fn decimal_count(n: i64) -> u32 {
        n.to_string().chars().count() as u32
    }
}