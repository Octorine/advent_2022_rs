use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub},
};

#[derive(Hash, Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }
    pub fn mv(&mut self, dir: char) {
        match dir {
            'U' => self.y -= 1,
            'D' => self.y += 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _ => (),
        }
    }
    pub fn follow(&mut self, other: Coords) {
        if (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1 {
            if self.x - other.x == 0 {
                let delta = self.y - other.y;
                self.y += delta / delta.abs();
            } else if self.y - other.y == 0 {
                let delta = self.x - other.x;
                self.x += delta / delta.abs();
            } else {
                let xd = self.x - other.x;
                let yd = self.y - other.y;
                self.x += xd / xd.abs();
                self.y += yd / yd.abs();
            }
        }
    }
    pub fn distance(self, other: &Coords) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
