use crate::day::Day;
use crate::y25::utils_2d::{Map2d, Point2d};

#[allow(dead_code)]
pub struct Day4 {
    map: Map2d<bool>,
}

#[allow(unused_variables)]
impl Day for Day4 {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect();

        Self {
            map: Map2d::new(map),
        }
    }

    fn solve0(&self) -> i64 {
        let mut res = 0;
        let (x_max, y_max) = self.map.size();
        for x in 0..x_max {
            for y in 0..y_max {
                let p = Point2d::new(x, y);
                let cell = *self.map.get(p);
                if cell {
                    let neighbours = p
                        .neighbours(true)
                        .iter()
                        .filter(|p| p.x < x_max && p.y < y_max)
                        .filter(|p| *self.map.get(**p) == true)
                        .count();
                    if neighbours < 4 {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    fn solve1(&self) -> i64 {
        0
    }
}
