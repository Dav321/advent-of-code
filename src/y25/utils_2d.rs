pub struct Map2d<T> {
    map: Vec<Vec<T>>,
}

impl<T> Map2d<T> {
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

    pub fn neighbours(&self, vertical: bool) -> Vec<Self> {
        let mut res = vec![self.up(), self.down(), self.left(), self.right()];
        if vertical {
            let mut rest = vec![];
            for neighbor in [self.left(), self.right()] {
                if let Some(neighbor) = neighbor { 
                    rest.push(neighbor.up());
                    rest.push(neighbor.down());
                }
            }
            res.append(&mut rest)
        }
        res.iter().filter_map(|x| *x).collect()
    }
}
