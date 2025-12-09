use crate::day::Day;
use crate::y25::utils_2d::{Line2d, Point2d, Rect2d};
use std::str::FromStr;

pub struct Day9 {
    tiles: Vec<Point2d>,
}

impl Day for Day9 {
    fn new(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (usize::from_str(x).unwrap(), usize::from_str(y).unwrap()))
            .map(|(x, y)| Point2d::new(x, y))
            .collect();
        Self { tiles }
    }

    fn solve0(&self) -> i64 {
        let mut largest_area = 0;
        for i in 0..self.tiles.len() {
            for j in (i + 1)..self.tiles.len() {
                let rect = Rect2d::new(&self.tiles[i], &self.tiles[j]);
                let area = rect.area();
                if area > largest_area {
                    largest_area = area;
                }
            }
        }

        largest_area as i64
    }

    fn solve1(&self) -> i64 {
        let polygon_lines: Vec<Line2d> = self
            .tiles
            .iter()
            .zip(
                self.tiles
                    .iter()
                    .skip(1)
                    .chain(std::iter::once(&self.tiles[0])),
            )
            .map(|(p1, p2)| Line2d::new(*p1, *p2))
            .collect();

        let mut largest_area = 0;
        for i in 0..self.tiles.len() {
            'next: for j in (i + 1)..self.tiles.len() {
                let rect = Rect2d::new(&self.tiles[i], &self.tiles[j]);
                let area = rect.area();

                if area <= largest_area {
                    continue 'next;
                }

                let rectangle_lines = rect.shrink(1).lines();
                for polygon_line in polygon_lines.iter() {
                    for rectangle_line in rectangle_lines {
                        if polygon_line.intersects_hv(rectangle_line) {
                            continue 'next;
                        }
                    }
                }

                largest_area = area;
            }
        }

        largest_area as i64
    }
}

impl Day9 {
    fn inside_rect(point: (i64, i64), b1: (i64, i64), b2: (i64, i64)) -> bool {
        let (x, y) = point;
        let (x1, y1) = b1;
        let (x2, y2) = b2;
        x > x1.min(x2) && y > y1.min(y2) && x < x1.max(x2) && y < y1.max(y2)
    }

    fn inside_line(point: i64, b1: i64, b2: i64) -> bool {
        point > b1.min(b2) && point < b1.max(b2)
    }
}
