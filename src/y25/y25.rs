use crate::day::Day;
use crate::y25::day5::Day5;

pub fn run() {
    let day = Day5::new(include_str!("../../input.txt"));
    let p0 = day.solve0();
    let p1 = day.solve1();
    println!("Results: {:?} : {:?}", p0, p1);
}