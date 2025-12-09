use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Map2d<T> {
    map: Vec<Vec<T>>,
}

impl<T> Map2d<T>
where
    T: Clone,
{
    pub fn new(map: Vec<Vec<T>>) -> Self {
        Self { map }
    }

    pub fn get(&self, p: Point2d) -> &T {
        &self.map[p.x][p.y]
    }

    pub fn set(&mut self, p: Point2d, value: T) {
        self.map[p.x][p.y] = value;
    }

    pub fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    pub fn rotate(&self) -> Map2d<T>
    where
        T: Default,
    {
        let mut new: Vec<Vec<T>> = vec![vec![T::default(); self.map.len()]; self.map[0].len()];
        let (x_max, y_max) = self.size();
        for x in 0..x_max {
            for y in 0..y_max {
                new[y][x] = self.map[x][y].clone();
            }
        }
        Map2d::new(new)
    }

    pub fn count(&self, filter: &dyn Fn(&T) -> bool) -> usize {
        let mut res = 0;
        let (x_max, y_max) = self.size();
        for x in 0..x_max {
            for y in 0..y_max {
                if filter(&self.map[x][y]) {
                    res += 1;
                }
            }
        }
        res
    }

    pub fn map(&self, mut f: impl FnMut(&T, Point2d) -> T) -> Map2d<T> {
        let mut new = self.map.clone();
        let (x_max, y_max) = self.size();
        for x in 0..x_max {
            for y in 0..y_max {
                new[x][y] = f(&self.map[x][y], Point2d::new(x, y));
            }
        }
        Map2d::new(new)
    }

    pub fn inner(&self) -> Vec<Vec<T>> {
        self.map.clone()
    }

    pub fn neighbours(&self, p: Point2d, vertical: bool) -> Vec<Point2d> {
        let mut res = vec![p.up(), p.down(), p.left(), p.right()];
        if vertical {
            if let Some(up) = p.up() {
                res.append(&mut vec![up.left(), up.right()])
            };
            if let Some(down) = p.down() {
                res.append(&mut vec![down.left(), down.right()])
            };
        }
        let (x_max, y_max) = self.size();
        res.iter()
            .filter_map(|p| *p)
            .filter(|p| p.inside(0, 0, x_max, y_max))
            .collect()
    }

    pub fn print(&self, fmt: impl Fn(&T) -> String) {
        let (x_max, y_max) = self.size();
        for y in 0..y_max {
            for x in 0..x_max {
                print!("{}", fmt(&self.map[x][y]));
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

impl Point2d {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn up(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Self::new(self.x, self.y - 1))
    }

    pub fn down(&self) -> Option<Self> {
        if self.y == usize::MAX {
            return None;
        }
        Some(Self::new(self.x, self.y + 1))
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Self::new(self.x - 1, self.y))
    }

    pub fn right(&self) -> Option<Self> {
        if self.x == usize::MAX {
            return None;
        }
        Some(Self::new(self.x + 1, self.y))
    }

    pub fn inside(&self, x_min: usize, y_min: usize, x_max: usize, y_max: usize) -> bool {
        self.x >= x_min && self.y >= y_min && self.x < x_max && self.y < y_max
    }
}

impl Display for Point2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect2d {
    pub top_left: Point2d,
    pub bottom_right: Point2d,
}

impl Rect2d {
    pub fn new(p1: &Point2d, p2: &Point2d) -> Self {
        let top_left = Point2d::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let bottom_right = Point2d::new(p1.x.max(p2.x), p1.y.max(p2.y));
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        let width = self.bottom_right.x - self.top_left.x + 1;
        let height = self.bottom_right.y - self.top_left.y + 1;
        (width, height)
    }

    pub fn area(&self) -> usize {
        let (w, h) = self.size();
        w * h
    }

    pub fn points(&self) -> [Point2d; 4] {
        [
            self.top_left,
            Point2d::new(self.top_left.x, self.bottom_right.y),
            self.bottom_right,
            Point2d::new(self.bottom_right.x, self.top_left.y),
        ]
    }

    pub fn shrink(&self, n: usize) -> Self {
        Self::new(
            &Point2d::new(self.top_left.x + n, self.top_left.y + n),
            &Point2d::new(self.bottom_right.x - n, self.bottom_right.y - n),
        )
    }

    pub fn lines(&self) -> [Line2d; 4] {
        let [p1, p2, p3, p4] = self.points();
        [
            Line2d::new(p1, p2),
            Line2d::new(p2, p3),
            Line2d::new(p3, p4),
            Line2d::new(p4, p1),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line2d {
    pub start: Point2d,
    pub end: Point2d,
}

impl Line2d {
    pub fn new(start: Point2d, end: Point2d) -> Self {
        Self { start, end }
    }

    pub fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn intersects_hv(&self, other: Line2d) -> bool {
        assert!(self.horizontal() || self.vertical());
        assert!(other.horizontal() || other.vertical());
        if self.vertical() {
            if other.horizontal() {
                let x = self.start.x;
                let y = other.start.y;

                let (x1, x2) = (
                    other.start.x.min(other.end.x),
                    other.start.x.max(other.end.x),
                );
                let (y1, y2) = (self.start.y.min(self.end.y), self.start.y.max(self.end.y));

                x >= x1 && x <= x2 && y >= y1 && y <= y2
            } else {
                false
            }
        } else if self.horizontal() {
            if other.vertical() {
                let y = self.start.y;
                let x = other.start.x;

                let (x1, x2) = (self.start.x.min(self.end.x), self.start.x.max(self.end.x));
                let (y1, y2) = (
                    other.start.y.min(other.end.y),
                    other.start.y.max(other.end.y),
                );

                x >= x1 && x <= x2 && y >= y1 && y <= y2
            } else {
                false
            }
        } else {
            unreachable!()
        }
    }
}
