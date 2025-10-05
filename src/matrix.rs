use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign}, sync::Arc};
use num_traits::{Float, One, Zero};
use crate::vector::{Vector2, Vector3, Vector4};

//--------
// Matrix2
//--------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix2<T> {
    pub c1r1: T, pub c1r2: T,
    pub c2r1: T, pub c2r2: T
}

impl<T> Matrix2<T> {
    pub fn new(
     c1r1: T, c1r2: T, 
        c2r1: T, c2r2: T
    ) -> Self {
        Self {
         c1r1, c1r2,
            c2r1, c2r2
        }
    }
}

impl<T> Matrix2<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            c1r1: T::zero(),
            c1r2: T::zero(),
            c2r1: T::zero(),
            c2r2: T::zero()
        }
    }
}

impl<T> Matrix2<T> where T: One {
    pub fn one() -> Self {
        Self {
            c1r1: T::one(),
            c1r2: T::one(),
            c2r1: T::one(),
            c2r2: T::one()
        }
    }
}
// Matrix2 Addition

impl<T> Add for Matrix2<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 + rhs.c1r1,
            c1r2: self.c1r2 + rhs.c1r2,
            c2r1: self.c2r1 + rhs.c2r1,
            c2r2: self.c2r2 + rhs.c2r2
        }
    }
}

impl<T> AddAssign for Matrix2<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.c1r1 += rhs.c1r1;
        self.c1r2 += rhs.c1r2;
        self.c2r1 += rhs.c2r1;
        self.c2r2 += rhs.c2r2;
    }
}

// Matrix2 Subtraction

impl<T> Sub for Matrix2<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 - rhs.c1r1,
            c1r2: self.c1r2 - rhs.c1r2,
            c2r1: self.c2r1 - rhs.c2r1,
            c2r2: self.c2r2 - rhs.c2r2
        }
    }
}

impl<T> SubAssign for Matrix2<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.c1r1 -= rhs.c1r1;
        self.c1r2 -= rhs.c1r2;
        self.c2r1 -= rhs.c2r1;
        self.c2r1 -= rhs.c2r2;
    }
}

// Matrix2 Multiplication

impl<T> Mul for Matrix2<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 * rhs.c1r1,
            c1r2: self.c1r2 * rhs.c1r2,
            c2r1: self.c2r1 * rhs.c2r1,
            c2r2: self.c2r2 * rhs.c2r2
        }
    }
}

impl<T> MulAssign for Matrix2<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.c1r1 *= rhs.c1r1;
        self.c1r2 *= rhs.c1r2;
        self.c2r1 *= rhs.c2r1;
        self.c2r2 *= rhs.c2r2;
    }
}

// Matrix 2 Division

impl<T> Div for Matrix2<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 / rhs.c1r1,
            c1r2: self.c1r2 / rhs.c1r2,
            c2r1: self.c2r1 / rhs.c2r1,
            c2r2: self.c2r2 / rhs.c2r2
        }
    }
}

impl<T> DivAssign for Matrix2<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.c1r1 /= rhs.c1r1;
        self.c1r2 /= rhs.c1r2;
        self.c2r1 /= rhs.c2r1;
        self.c2r2 /= rhs.c2r2;
    }
}

// Matrix2 Remainder

impl<T> Rem for Matrix2<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 % rhs.c1r1,
            c1r2: self.c1r2 % rhs.c1r2,
            c2r1: self.c2r1 % rhs.c2r1,
            c2r2: self.c2r2 % rhs.c2r2
        }
    }
}

impl<T> RemAssign for Matrix2<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.c1r1 %= rhs.c1r1;
        self.c1r2 %= rhs.c1r2;
        self.c2r1 %= rhs.c2r1;
        self.c2r2 %= rhs.c2r2;
    }
}

//--------
// Matrix3
//--------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix3<T> {
    pub c1r1: T, pub c1r2: T, pub c1r3: T,
    pub c2r1: T, pub c2r2: T, pub c2r3: T,
    pub c3r1: T, pub c3r2: T, pub c3r3: T
}

impl<T> Matrix3<T> {
        pub fn new(
     c1r1: T, c1r2: T, c1r3: T,
        c2r1: T, c2r2: T, c2r3: T,
        c3r1: T, c3r2: T, c3r3: T
    ) -> Self {
        Self {
         c1r1, c1r2, c1r3,
            c2r1, c2r2, c2r3,
            c3r1, c3r2, c3r3
        }
    }
}

impl<T> Matrix3<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            c1r1: T::zero(),
            c1r2: T::zero(),
            c1r3: T::zero(),
            c2r1: T::zero(),
            c2r2: T::zero(),
            c2r3: T::zero(),
            c3r1: T::zero(),
            c3r2: T::zero(),
            c3r3: T::zero()
        }
    }
}

impl<T> Matrix3<T> where T: One {
    pub fn one() -> Self {
        Self {
            c1r1: T::one(),
            c1r2: T::one(),
            c1r3: T::one(),
            c2r1: T::one(),
            c2r2: T::one(),
            c2r3: T::one(),
            c3r1: T::one(),
            c3r2: T::one(),
            c3r3: T::one()
        }
    }
}

// Matrix3 Addition

impl<T> Add for Matrix3<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 + rhs.c1r1,
            c1r2: self.c1r2 + rhs.c1r2,
            c1r3: self.c1r3 + rhs.c1r3,
            c2r1: self.c2r1 + rhs.c2r1,
            c2r2: self.c2r2 + rhs.c2r2,
            c2r3: self.c2r3 + rhs.c2r3,
            c3r1: self.c3r1 + rhs.c3r1,
            c3r2: self.c3r2 + rhs.c3r2,
            c3r3: self.c3r3 + rhs.c3r3
        }
    }
}

impl<T> AddAssign for Matrix3<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.c1r1 += rhs.c1r1;
        self.c1r2 += rhs.c1r2;
        self.c1r3 += rhs.c1r3;
        self.c2r1 += rhs.c2r1;
        self.c2r2 += rhs.c2r2;
        self.c2r3 += rhs.c2r3;
        self.c3r1 += rhs.c3r1;
        self.c3r2 += rhs.c3r2;
        self.c3r3 += rhs.c3r3;
    }
}

// Matrix3 Subtraction

impl<T> Sub for Matrix3<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 - rhs.c1r1,
            c1r2: self.c1r2 - rhs.c1r2,
            c1r3: self.c1r3 - rhs.c1r3,
            c2r1: self.c2r1 - rhs.c2r1,
            c2r2: self.c2r2 - rhs.c2r2,
            c2r3: self.c2r3 - rhs.c2r3,
            c3r1: self.c3r1 - rhs.c3r1,
            c3r2: self.c3r2 - rhs.c3r2,
            c3r3: self.c3r3 - rhs.c3r3
        }
    }
}

impl<T> SubAssign for Matrix3<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.c1r1 -= rhs.c1r1;
        self.c1r2 -= rhs.c1r2;
        self.c1r3 -= rhs.c1r3;
        self.c2r1 -= rhs.c2r1;
        self.c2r1 -= rhs.c2r2;
        self.c2r3 -= rhs.c2r3;
        self.c3r1 -= rhs.c3r1;
        self.c3r2 -= rhs.c3r2;
        self.c3r3 -= rhs.c3r3;
    }
}

// Matrix3 Multiplication

impl<T> Mul for Matrix3<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 * rhs.c1r1,
            c1r2: self.c1r2 * rhs.c1r2,
            c1r3: self.c1r3 * rhs.c1r3,
            c2r1: self.c2r1 * rhs.c2r1,
            c2r2: self.c2r2 * rhs.c2r2,
            c2r3: self.c2r3 * rhs.c2r3,
            c3r1: self.c3r1 * rhs.c3r1,
            c3r2: self.c3r2 * rhs.c3r2,
            c3r3: self.c3r3 * rhs.c3r3
        }
    }
}

impl<T> MulAssign for Matrix3<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.c1r1 *= rhs.c1r1;
        self.c1r2 *= rhs.c1r2;
        self.c1r3 *= rhs.c1r3;
        self.c2r1 *= rhs.c2r1;
        self.c2r2 *= rhs.c2r2;
        self.c2r3 *= rhs.c2r3;
        self.c3r1 *= rhs.c3r1;
        self.c3r2 *= rhs.c3r2;
        self.c3r3 *= rhs.c3r3;
    }
}

// Matrix3 Division

impl<T> Div for Matrix3<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 / rhs.c1r1,
            c1r2: self.c1r2 / rhs.c1r2,
            c1r3: self.c1r3 / rhs.c1r3,
            c2r1: self.c2r1 / rhs.c2r1,
            c2r2: self.c2r2 / rhs.c2r2,
            c2r3: self.c2r3 / rhs.c2r3,
            c3r1: self.c3r1 / rhs.c3r1,
            c3r2: self.c3r2 / rhs.c3r2,
            c3r3: self.c3r3 / rhs.c3r3
        }
    }
}

impl<T> DivAssign for Matrix3<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.c1r1 /= rhs.c1r1;
        self.c1r2 /= rhs.c1r2;
        self.c1r3 /= rhs.c1r3;
        self.c2r1 /= rhs.c2r1;
        self.c2r2 /= rhs.c2r2;
        self.c2r3 /= rhs.c2r3;
        self.c3r1 /= rhs.c3r1;
        self.c3r2 /= rhs.c3r2;
        self.c3r3 /= rhs.c3r3;
    }
}

// Matrix3 Remainder

impl<T> Rem for Matrix3<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 % rhs.c1r1,
            c1r2: self.c1r2 % rhs.c1r2,
            c1r3: self.c1r3 % rhs.c1r3,
            c2r1: self.c2r1 % rhs.c2r1,
            c2r2: self.c2r2 % rhs.c2r2,
            c2r3: self.c2r3 % rhs.c2r3,
            c3r1: self.c3r1 % rhs.c3r1,
            c3r2: self.c3r2 % rhs.c3r2,
            c3r3: self.c3r3 % rhs.c3r3
        }
    }
}

impl<T> RemAssign for Matrix3<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.c1r1 %= rhs.c1r1;
        self.c1r2 %= rhs.c1r2;
        self.c1r3 %= rhs.c1r3;
        self.c2r1 %= rhs.c2r1;
        self.c2r2 %= rhs.c2r2;
        self.c2r3 %= rhs.c2r3;
        self.c3r1 %= rhs.c3r1;
        self.c3r2 %= rhs.c3r2;
        self.c3r3 %= rhs.c3r3;
    }
}

//--------
// Matrix4
//--------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Matrix4<T> {
    pub c1r1: T, pub c1r2: T, pub c1r3: T, pub c1r4: T,
    pub c2r1: T, pub c2r2: T, pub c2r3: T, pub c2r4: T,
    pub c3r1: T, pub c3r2: T, pub c3r3: T, pub c3r4: T,
    pub c4r1: T, pub c4r2: T, pub c4r3: T, pub c4r4: T
}

impl<T> Matrix4<T> {
        pub fn new(
     c1r1: T, c1r2: T, c1r3: T, c1r4: T,
        c2r1: T, c2r2: T, c2r3: T, c2r4: T,
        c3r1: T, c3r2: T, c3r3: T, c3r4: T,
        c4r1: T, c4r2: T, c4r3: T, c4r4: T
    ) -> Self {
        Self {
         c1r1, c1r2, c1r3, c1r4,
            c2r1, c2r2, c2r3, c2r4,
            c3r1, c3r2, c3r3, c3r4,
            c4r1, c4r2, c4r3, c4r4
        }
    }
}

impl<T> Matrix4<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            c1r1: T::zero(),
            c1r2: T::zero(),
            c1r3: T::zero(),
            c1r4: T::zero(),
            c2r1: T::zero(),
            c2r2: T::zero(),
            c2r3: T::zero(),
            c2r4: T::zero(),
            c3r1: T::zero(),
            c3r2: T::zero(),
            c3r3: T::zero(),
            c3r4: T::zero(),
            c4r1: T::zero(),
            c4r2: T::zero(),
            c4r3: T::zero(),
            c4r4: T::zero()
        }
    }
}

impl<T> Matrix4<T> where T: One {
    pub fn one() -> Self {
        Self {
            c1r1: T::one(),
            c1r2: T::one(),
            c1r3: T::one(),
            c1r4: T::one(),
            c2r1: T::one(),
            c2r2: T::one(),
            c2r3: T::one(),
            c2r4: T::one(),
            c3r1: T::one(),
            c3r2: T::one(),
            c3r3: T::one(),
            c3r4: T::one(),
            c4r1: T::one(),
            c4r2: T::one(),
            c4r3: T::one(),
            c4r4: T::one()           
        }
    }
}

// Matrix4 Addition

impl<T> Add for Matrix4<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 + rhs.c1r1,
            c1r2: self.c1r2 + rhs.c1r2,
            c1r3: self.c1r3 + rhs.c1r3,
            c1r4: self.c1r4 + rhs.c1r4,
            c2r1: self.c2r1 + rhs.c2r1,
            c2r2: self.c2r2 + rhs.c2r2,
            c2r3: self.c2r3 + rhs.c2r3,
            c2r4: self.c2r4 + rhs.c2r4,
            c3r1: self.c3r1 + rhs.c3r1,
            c3r2: self.c3r2 + rhs.c3r2,
            c3r3: self.c3r3 + rhs.c3r3,
            c3r4: self.c3r4 + rhs.c3r4,
            c4r1: self.c4r1 + rhs.c4r1,
            c4r2: self.c4r2 + rhs.c4r2,
            c4r3: self.c4r3 + rhs.c4r3,
            c4r4: self.c4r4 + rhs.c4r4
        }
    }
}

impl<T> AddAssign for Matrix4<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.c1r1 += rhs.c1r1;
        self.c1r2 += rhs.c1r2;
        self.c1r3 += rhs.c1r3;
        self.c1r4 += rhs.c1r4;
        self.c2r1 += rhs.c2r1;
        self.c2r2 += rhs.c2r2;
        self.c2r3 += rhs.c2r3;
        self.c2r4 += rhs.c2r4;
        self.c3r1 += rhs.c3r1;
        self.c3r2 += rhs.c3r2;
        self.c3r3 += rhs.c3r3;
        self.c3r4 += rhs.c3r4;
        self.c4r1 += rhs.c4r1;
        self.c4r2 += rhs.c4r2;
        self.c4r3 += rhs.c4r3;
        self.c4r4 += rhs.c4r4;
    }
}

// Matrix4 Subtraction

impl<T> Sub for Matrix4<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 - rhs.c1r1,
            c1r2: self.c1r2 - rhs.c1r2,
            c1r3: self.c1r3 - rhs.c1r3,
            c1r4: self.c1r4 - rhs.c1r4,
            c2r1: self.c2r1 - rhs.c2r1,
            c2r2: self.c2r2 - rhs.c2r2,
            c2r3: self.c2r3 - rhs.c2r3,
            c2r4: self.c2r4 - rhs.c2r4,
            c3r1: self.c3r1 - rhs.c3r1,
            c3r2: self.c3r2 - rhs.c3r2,
            c3r3: self.c3r3 - rhs.c3r3,
            c3r4: self.c3r4 - rhs.c3r4,
            c4r1: self.c4r1 - rhs.c4r1,
            c4r2: self.c4r2 - rhs.c4r2,
            c4r3: self.c4r3 - rhs.c4r3,
            c4r4: self.c4r4 - rhs.c4r4
        }
    }
}

impl<T> SubAssign for Matrix4<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.c1r1 -= rhs.c1r1;
        self.c1r2 -= rhs.c1r2;
        self.c1r3 -= rhs.c1r3;
        self.c1r4 -= rhs.c1r4;
        self.c2r1 -= rhs.c2r1;
        self.c2r1 -= rhs.c2r2;
        self.c2r3 -= rhs.c2r3;
        self.c2r4 -= rhs.c2r4;
        self.c3r1 -= rhs.c3r1;
        self.c3r2 -= rhs.c3r2;
        self.c3r3 -= rhs.c3r3;
        self.c3r4 -= rhs.c3r4;
        self.c4r1 -= rhs.c4r1;
        self.c4r2 -= rhs.c4r2;
        self.c4r3 -= rhs.c4r3;
        self.c4r4 -= rhs.c4r4;
    }
}

// Matrix4 Multiplication

impl<T> Mul for Matrix4<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 * rhs.c1r1,
            c1r2: self.c1r2 * rhs.c1r2,
            c1r3: self.c1r3 * rhs.c1r3,
            c1r4: self.c1r4 * rhs.c1r4,
            c2r1: self.c2r1 * rhs.c2r1,
            c2r2: self.c2r2 * rhs.c2r2,
            c2r3: self.c2r3 * rhs.c2r3,
            c2r4: self.c2r4 * rhs.c2r4,
            c3r1: self.c3r1 * rhs.c3r1,
            c3r2: self.c3r2 * rhs.c3r2,
            c3r3: self.c3r3 * rhs.c3r3,
            c3r4: self.c3r4 * rhs.c3r4,
            c4r1: self.c4r1 * rhs.c4r1,
            c4r2: self.c4r2 * rhs.c4r2,
            c4r3: self.c4r3 * rhs.c4r3,
            c4r4: self.c4r4 * rhs.c4r4
        }
    }
}

impl<T> MulAssign for Matrix4<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.c1r1 *= rhs.c1r1;
        self.c1r2 *= rhs.c1r2;
        self.c1r3 *= rhs.c1r3;
        self.c1r4 *= rhs.c1r4;
        self.c2r1 *= rhs.c2r1;
        self.c2r2 *= rhs.c2r2;
        self.c2r3 *= rhs.c2r3;
        self.c2r4 *= rhs.c2r4;
        self.c3r1 *= rhs.c3r1;
        self.c3r2 *= rhs.c3r2;
        self.c3r3 *= rhs.c3r3;
        self.c3r4 *= rhs.c3r4;
        self.c4r1 *= rhs.c4r1;
        self.c4r2 *= rhs.c4r2;
        self.c4r3 *= rhs.c4r3;
        self.c4r4 *= rhs.c4r4;
    }
}

// Matrix4 Division

impl<T> Div for Matrix4<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 / rhs.c1r1,
            c1r2: self.c1r2 / rhs.c1r2,
            c1r3: self.c1r3 / rhs.c1r3,
            c1r4: self.c1r4 / rhs.c1r4,
            c2r1: self.c2r1 / rhs.c2r1,
            c2r2: self.c2r2 / rhs.c2r2,
            c2r3: self.c2r3 / rhs.c2r3,
            c2r4: self.c2r4 / rhs.c2r4,
            c3r1: self.c3r1 / rhs.c3r1,
            c3r2: self.c3r2 / rhs.c3r2,
            c3r3: self.c3r3 / rhs.c3r3,
            c3r4: self.c3r4 / rhs.c3r4,
            c4r1: self.c4r1 / rhs.c4r1,
            c4r2: self.c4r2 / rhs.c4r2,
            c4r3: self.c4r3 / rhs.c4r3,
            c4r4: self.c4r4 / rhs.c4r4
        }
    }
}

impl<T> DivAssign for Matrix4<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.c1r1 /= rhs.c1r1;
        self.c1r2 /= rhs.c1r2;
        self.c1r3 /= rhs.c1r3;
        self.c1r4 /= rhs.c1r4;
        self.c2r1 /= rhs.c2r1;
        self.c2r2 /= rhs.c2r2;
        self.c2r3 /= rhs.c2r3;
        self.c2r4 /= rhs.c2r4;
        self.c3r1 /= rhs.c3r1;
        self.c3r2 /= rhs.c3r2;
        self.c3r3 /= rhs.c3r3;
        self.c3r4 /= rhs.c3r4;
        self.c4r1 /= rhs.c4r1;
        self.c4r2 /= rhs.c4r2;
        self.c4r3 /= rhs.c4r3;
        self.c4r4 /= rhs.c4r4;
    }
}

// Matrix4 Remainder

impl<T> Rem for Matrix4<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            c1r1: self.c1r1 % rhs.c1r1,
            c1r2: self.c1r2 % rhs.c1r2,
            c1r3: self.c1r3 % rhs.c1r3,
            c1r4: self.c1r4 % rhs.c1r4,
            c2r1: self.c2r1 % rhs.c2r1,
            c2r2: self.c2r2 % rhs.c2r2,
            c2r3: self.c2r3 % rhs.c2r3,
            c2r4: self.c2r4 % rhs.c2r4,
            c3r1: self.c3r1 % rhs.c3r1,
            c3r2: self.c3r2 % rhs.c3r2,
            c3r3: self.c3r3 % rhs.c3r3,
            c3r4: self.c3r4 % rhs.c3r4,
            c4r1: self.c4r1 % rhs.c4r1,
            c4r2: self.c4r2 % rhs.c4r2,
            c4r3: self.c4r3 % rhs.c4r3,
            c4r4: self.c4r4 % rhs.c4r4
        }
    }
}

impl<T> RemAssign for Matrix4<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.c1r1 %= rhs.c1r1;
        self.c1r2 %= rhs.c1r2;
        self.c1r3 %= rhs.c1r3;
        self.c1r4 %= rhs.c1r4;
        self.c2r1 %= rhs.c2r1;
        self.c2r2 %= rhs.c2r2;
        self.c2r3 %= rhs.c2r3;
        self.c2r4 %= rhs.c2r4;
        self.c3r1 %= rhs.c3r1;
        self.c3r2 %= rhs.c3r2;
        self.c3r3 %= rhs.c3r3;
        self.c3r4 %= rhs.c3r4;
        self.c4r1 %= rhs.c4r1;
        self.c4r2 %= rhs.c4r2;
        self.c4r3 %= rhs.c4r3;
        self.c4r4 %= rhs.c4r4;
    }
}

//--------------
// ColumnMatrix2
//--------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix2<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>
}

impl<T> ColumnMatrix2<T> {
    pub fn new(x: Vector2<T>, y: Vector2<T>) -> Self {
        Self {
            x,
            y
        }
    }
}
//--------------
// ColumnMatrix3
//--------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix3<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>
}

impl<T> ColumnMatrix3<T> {
    pub fn new(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

//--------------
// ColumnMatrix4
//--------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix4<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>,
    pub w: Vector4<T>
}

impl<T> ColumnMatrix4<T> {
    pub fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }
}

//----------------
// ColumnMatrix2x3
//----------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix2x3<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
    pub z: Vector2<T>
}

//----------------
// ColumnMatrix2x4
//----------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix2x4<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
    pub z: Vector2<T>,
    pub w: Vector2<T>
}

//----------------
// ColumnMatrix3x2
//----------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumMatrix3x2<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>
}

//----------------
// ColumnMatrix3x4
//----------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix3x4<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>,
    pub w: Vector3<T>
}

//----------------
// ColumnMatrix4x2
//----------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix4x2<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>
}

//----------------
// ColumnMatrix4x3
//----------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix4x3<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>
}

//-----------
// RowMatrix2
//-----------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowMatrix2<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>
}

impl<T> RowMatrix2<T> {
    pub fn new(x: Vector2<T>, y: Vector2<T>) -> Self {
        Self {
            x,
            y
        }
    }
}

//-----------
// RowMatrix3
//-----------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowMatrix3<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>
}

impl<T> RowMatrix3<T> {
    pub fn new(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

//-----------
// RowMatrix4
//-----------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowMatrix4<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>,
    pub w: Vector4<T>
}

impl<T> RowMatrix4<T> {
    pub fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }
}

//-------------
// RowMatrix2x3
//-------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowMatrix2x3<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>
}

impl<T> RowMatrix2x3<T> {
    pub fn new(x: Vector3<T>, y: Vector3<T>) -> Self {
        Self {
            x,
            y
        }
    }
}

//-------------
// RowMatrix2x4
//-------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowMatrix2x4<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>
}

impl<T> RowMatrix2x4<T> {
    pub fn new(x: Vector4<T>, y: Vector4<T>) -> Self {
        Self {
            x,
            y
        }
    }
}

//-------------
// RowMatrix3x2
//-------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowMatrix3x2<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
    pub z: Vector2<T>
}

impl<T> RowMatrix3x2<T> {
    pub fn new(x: Vector2<T>, y: Vector2<T>, z: Vector2<T>) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

//-------------
// RowMatrix3x4
//-------------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowMatrix3x4<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>
}

impl<T> RowMatrix3x4<T> {
    pub fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}