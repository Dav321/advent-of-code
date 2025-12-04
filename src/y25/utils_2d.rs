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

    pub fn rotate(&self) -> Map2d<T> {
        let mut new = self.map.clone();
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

    pub fn print(&self, fmt: &dyn Fn(&T) -> &str) {
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
