use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::day::Day;

#[allow(dead_code)]
pub struct Day5 {
    fresh: Vec<RangeInclusive<i64>>,
    available: Vec<i64>,
}

#[allow(unused_variables)]
impl Day for Day5 {
    fn new(input: &str) -> Self {
        let (input_fresh, input_available) = input.split_once("\n\n").expect("invalid input");
        let fresh = input_fresh.lines()
            .map(|l| l.split_once("-").expect("no range"))
            .map(|(a, b)| i64::from_str(a).expect("fresh0: nan")..=i64::from_str(b).expect("fresh1: nan"))
            .collect();
        let available = input_available.lines()
            .map(|l| i64::from_str(l).expect("available: nan"))
            .collect();

        Self {
            fresh, available
        }
    }

    fn solve0(&self) -> i64 {
        self.available.iter().filter(|&n|
            self.fresh.iter().any(|r| r.contains(n))
        ).count() as i64
    }

    fn solve1(&self) -> i64 {
        0
    }
}
