use crate::day::Day;
use crate::y25::day10::Day10 as CurrentDay;

pub fn run() {
    let day = CurrentDay::new(include_str!("../../input.txt"));
    let p0 = day.solve0();
    let p1 = day.solve1();
    println!("Results: {:?} : {:?}", p0, p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let day = CurrentDay::new(include_str!("../../test.txt"));
        let p0 = day.solve0();
        let p1 = day.solve1();
        println!("Test Results: {:?} : {:?}", p0, p1);
    }
}
