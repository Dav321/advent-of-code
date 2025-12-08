use std::fmt::Display;
use std::ops::{Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point3d {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3d {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn distance2(&self, other: &Self) -> i64 {
        let diff = *self - *other;
        let diff = diff * diff;
        diff.x + diff.y + diff.z
    }
}

impl Sub for Point3d {
    type Output = Point3d;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Point3d> for Point3d {
    type Output = Point3d;

    fn mul(self, other: Point3d) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Display for Point3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
