use crate::day::Day;
use crate::y25::day1::Day1;

pub fn run() {
    let p0 = Day1::solve0(include_str!("../../input.txt"));
    let p1 = Day1::solve1(include_str!("../../input.txt"));
    println!("Results: {:?}, {:?}", p0, p1);
}