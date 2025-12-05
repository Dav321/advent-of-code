use crate::day::Day;
use crate::y25::utils_2d::Map2d;

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
            map: Map2d::new(map).rotate(),
        }
    }

    fn solve0(&self) -> i64 {
        self.map
            .map(|cell, p| {
                if !*cell {
                    return false;
                }
                let n = self
                    .map
                    .neighbours(p, true)
                    .iter()
                    .filter(|p| *self.map.get(**p))
                    .count();
                n < 4
            })
            .count(&|p| *p) as i64
    }

    fn solve1(&self) -> i64 {
        let mut last = self.map.count(&|cell| *cell);
        let initial = last;
        let mut current = last + 1;
        let mut map = self.map.clone();

        while current != last {
            map = map.map(|cell, p| {
                if !*cell {
                    return false;
                }
                let n = map
                    .neighbours(p, true)
                    .iter()
                    .filter(|p| *map.get(**p))
                    .count();
                n >= 4
            });
            last = current;
            current = map.count(&|cell| *cell);
        }

        (initial - current) as i64
    }
}
