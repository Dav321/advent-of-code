use crate::day::Day;
use crate::y25::day6::Day6;

pub fn run() {
    let day = Day6::new(include_str!("../../input.txt"));
    let p0 = day.solve0();
    let p1 = day.solve1();
    println!("Results: {:?} : {:?}", p0, p1);
}
