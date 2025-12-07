use crate::day::Day;
use crate::y25::day7::Day7 as CurrentDay;

pub fn run() {
    let day = CurrentDay::new(include_str!("../../input.txt"));
    let p0 = day.solve0();
    let p1 = day.solve1();
    println!("Results: {:?} : {:?}", p0, p1);
}
