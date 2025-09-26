use std::ops::{Add, AddAssign, Sub, SubAssign};
use num_traits::Float;
//--------
// Point2
//--------

pub struct Point2<T> {
    pub x: T,
    pub y: T
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

// Point2 Addition

impl<T> Add for Point2<T> where T: Add<Output = T>  {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T> AddAssign for Point2<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// Point2 Subtraction

impl<T> Sub for Point2<T> where T: Sub<Output= T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T> SubAssign for Point2<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

