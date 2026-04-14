use num_traits::{Num, One, Zero};
pub struct Point2<S> {
    pub x: S,
    pub y: S
}

impl<S> Point2<S> {
    pub fn new(x: S, y: S) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<S> Point2<S> where S: PartialEq {
    pub fn eq(&self, other: &Self) -> bool {
        if self.x.eq(&other.x) && self.y.eq(&other.y) {
            return true;
        }
        return false;
    }

    pub fn ne(&self, other: &Self) -> bool {
        if self.x.ne(&other.x) && self.y.ne(&other.y) {
            return true;
        }
        return false;
    }
}

impl<S> Point2<S> where S: Zero {
    pub fn zero() -> Self {
        Self {
            x: S::zero(),
            y: S::zero()
        }
    }

    pub fn is_zero(&self) -> bool {
        if self.x.is_zero() && self.y.is_zero() {
            return true;
        }
        return false;
    }

    pub fn set_zero(&mut self) {
        self.x.set_zero();
        self.y.set_zero();
    }
}

impl<S> Point2<S> where S: One {
    pub fn one() -> Self {
        Self {
            x: S::one(),
            y: S::one()
        }
    }
    pub fn is_one(&self) -> bool where S: PartialEq {
        if self.x.is_one() && self.y.is_one() {
            return true;
        }
        return false;
    }
    pub fn set_one(&mut self) {
        self.x.set_one();
        self.y.set_one();
    }
}

