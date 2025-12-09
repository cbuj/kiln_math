// point.rs

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use num_traits::{One, Zero};

//--------
// Point2
//--------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<T> Point2<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }
}

impl<T> Point2<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one()
        }
    }
}

impl<T> Point2<T> where T: Zero + One {
    pub fn right() -> Self {
        Self {
            x: T::one(),
            y: T::zero()
        }
    }
    pub fn up() -> Self {
        Self {
            x: T::zero(),
            y: T::one()
        }
    }
}

impl<T> Point2<T> where T: Zero + One + Neg<Output = T> {
    pub fn left() -> Self {
        Self {
            x: T::one().neg(),
            y: T::zero()
        }
    }
    pub fn down() -> Self {
        Self {
            x: T::zero(),
            y: T::one().neg()
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

// Point2 Multiplication

impl<T> Mul for Point2<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T> MulAssign for Point2<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

// Point2 Division

impl<T> Div for Point2<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl<T> DivAssign for Point2<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

// Point2 Remainder

impl<T> Rem for Point2<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y
        }
    }
}

impl<T> RemAssign for Point2<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

// Point2 mint implentations

impl<T> From<Point2<T>> for mint::Point2<T> {
    fn from(point: Point2<T>) -> Self {
        Self {
            x: point.x,
            y: point.y
        }
    }
}

impl<T> From<mint::Point2<T>> for Point2<T> {
    fn from(point: mint::Point2<T>) -> Self {
        Self {
            x: point.x,
            y: point.y
        }
    }
}

//--------
// Point3
//--------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

impl<T> Point3<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero()
        }
    }
}

impl<T> Point3<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one()
        }
    }
}

impl<T> Point3<T> where T: Zero + One {
    pub fn right() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero()
        }
    }
    pub fn up() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero()
        }
    }
    pub fn forward() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one()
        }
    }
}

impl<T> Point3<T> where T: Zero + One + Neg<Output = T> {
    pub fn left() -> Self {
        Self {
            x: T::one().neg(),
            y: T::zero(),
            z: T::zero()
        }
    }
    pub fn down() -> Self {
        Self {
            x: T::zero(),
            y: T::one().neg(),
            z: T::zero()
        }
    }
    pub fn backward() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one().neg()
        }
    }
}
// Point3 Addition

impl<T> Add for Point3<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T> AddAssign for Point3<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Point3 Subtraction

impl<T> Sub for Point3<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T> SubAssign for Point3<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// Point3 Multiplication

impl<T> Mul for Point3<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl<T> MulAssign for Point3<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// Point3 Division

impl<T> Div for Point3<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl<T> DivAssign for Point3<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// Point3 Remainder

impl<T> Rem for Point3<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z
        }
    }
}

impl<T> RemAssign for Point3<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}

// Point3 mint implentations

impl<T> From<Point3<T>> for mint::Point3<T> {
    fn from(point: Point3<T>) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: point.z
        }
    }
}

impl<T> From<mint::Point3<T>> for Point3<T> {
    fn from(point: mint::Point3<T>) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: point.z
        }
    }
}