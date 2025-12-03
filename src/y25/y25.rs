use crate::day::Day;
use crate::y25::day3::Day3;

pub fn run() {
    let day = Day3::new(include_str!("../../input.txt"));
    let p0 = day.solve0();
    let p1 = day.solve1();
    println!("Results: {:?}, {:?}", p0, p1);
}