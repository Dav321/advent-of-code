use crate::day::Day;
use crate::y25::utils_3d::Point3d;
use std::collections::{BTreeMap, HashMap, HashSet};

#[allow(dead_code)]
pub struct Day8 {
    boxes: Vec<Point3d>,
}

#[allow(unused_variables)]
impl Day for Day8 {
    fn new(input: &str) -> Self {
        let boxes = input
            .lines()
            .map(|l| {
                let mut points = l.splitn(3, ',').map(|s| s.parse::<i64>().unwrap());
                Point3d::new(
                    points.next().unwrap(),
                    points.next().unwrap(),
                    points.next().unwrap(),
                )
            })
            .collect();

        Self { boxes }
    }

    fn solve0(&self) -> i64 {
        let mut map = BTreeMap::new();
        for i in 0..self.boxes.len() {
            for j in i + 1..self.boxes.len() {
                let distance = self.boxes[i].distance2(&self.boxes[j]);
                match map.insert(distance, (i, j)) {
                    Some(_) => panic!("Duplicate distance"),
                    None => (),
                }
            }
        }

        let mut conn: HashMap<usize, usize> = HashMap::new();
        let mut group = 0;
        for (d, (i, j)) in map.iter().take(1000) {
            group += 1;
            let i_prev = conn.insert(*i, group);
            let j_prev = conn.insert(*j, group);

            if i_prev.is_some() || j_prev.is_some() {
                for g in conn.values_mut() {
                    // group can never be zero
                    if *g == i_prev.unwrap_or(0) || *g == j_prev.unwrap_or(0) {
                        *g = group
                    }
                }
            }
        }

        let mut groups = vec![0i64; group + 1];
        for v in conn.values() {
            groups[*v] += 1;
        }
        groups.sort_by_key(|&v| i64::MAX - v);

        groups[0] * groups[1] * groups[2]
    }

    fn solve1(&self) -> i64 {
        let mut map = BTreeMap::new();
        for i in 0..self.boxes.len() {
            for j in i + 1..self.boxes.len() {
                let distance = self.boxes[i].distance2(&self.boxes[j]);
                match map.insert(distance, (i, j)) {
                    Some(_) => panic!("Duplicate distance"),
                    None => (),
                }
            }
        }

        let mut conn: HashMap<usize, usize> = HashMap::new();
        let mut groups: HashSet<usize> = HashSet::new();
        let mut last = (0, 0);
        let mut group = 0;
        while groups.len() > 1 || conn.len() < self.boxes.len() || group < self.boxes.len() {
            group += 1;
            let (d, (i, j)) = map.pop_first().expect("Map empty :(");
            let i_prev = conn.insert(i, group).unwrap_or(0);
            let j_prev = conn.insert(j, group).unwrap_or(0);
            groups.insert(group);
            last = (i, j);

            if i_prev + j_prev != 0 {
                groups.remove(&i_prev);
                groups.remove(&j_prev);
                for g in conn.values_mut() {
                    // group can never be zero
                    if *g == i_prev || *g == j_prev {
                        *g = group
                    }
                }
            }
        }

        self.boxes[last.0].x * self.boxes[last.1].x
    }
}
