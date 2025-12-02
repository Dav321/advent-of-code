use crate::day::Day;
use crate::y25::day2::Day2;

pub fn run() {
    let p0 = Day2::solve0(include_str!("../../input.txt"));
    let p1 = 0; //Day2::solve1(include_str!("../../input.txt"));
    println!("Results: {:?}, {:?}", p0, p1);
}