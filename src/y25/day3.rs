use std::str::FromStr;
use crate::day::Day;

#[allow(dead_code)]
pub struct Day3 {
    banks: Vec<Box<[u8]>>,
}

#[allow(unused_variables)]
impl Day for Day3 {
    fn new(input: &str) -> Self {
        let banks = input.lines()
            .map(|l|
                l.chars()
                    .map(|s| u8::from_str(&s.to_string()).unwrap())
                    .collect())
            .collect();

        Self {
            banks
        }
    }

    fn solve0(&self) -> i64 {
        let mut res = 0;
        for bank in self.banks.clone() {
            let (first, first_i) = Day3::first_max(&bank[..bank.len()-1], 9);
            let (second, second_i) = Day3::first_max(&bank[first_i+1..], 9);
            let joltage = (first as i64 * 10) + second as i64;
            res += joltage;
        }
        res
    }

    fn solve1(&self) -> i64 {
        0
    }
}

impl Day3 {
    fn first_max(slice: &[u8], max_possible: u8) -> (u8, usize) {
        let mut max = 0;
        let mut max_value = slice[0];
        for i in 0..slice.len() {
            let val = slice[i];
            if val > max_value {
                max = i;
                max_value = val;
                if max_value == max_possible {
                    break;
                }
            }
        }
        (max_value, max)
    }
}