use crate::day::Day;

pub struct Day1 {

}

impl Day for Day1 {
    fn solve0(input: &str) -> i64 {
        input.lines()
            .map(|l| l.split_at(1))
            .map(|(d, n)| Self::dir(d) * n.parse::<i64>().unwrap())
            .fold((50, 0), |(i, res), n| {
                let i = Self::add(i, n);
                let res = res + if i == 0 { 1 } else { 0 };
                (i, res)
            }).1
    }

    fn solve1(input: &str) -> i64 {
        input.lines()
            .map(|l| l.split_at(1))
            .map(|(d, n)| [Self::dir(d)].repeat(n.parse::<usize>().unwrap()))
            .flatten()
            .fold((50, 0), |(i, res), n| {
                let i = Self::add(i, n);
                (i, res + if i == 0 { 1 } else { 0 })
            }).1
    }
}

impl Day1 {
    fn dir(dir: &str) -> i64 {

        if dir == "R" {
            1
        } else if dir == "L" {
            -1
        } else {
            panic!("Invalid direction");
        }
    }

    fn add(a: i64, b: i64) -> i64 {
        let res = a + b;

        if res % 100 == 0 {
            0
        } else if res >= 100 {
            res % 100
        } else if res < 0 {
            100 - (res.abs() % 100)
        } else {
            res
        }
    }
}