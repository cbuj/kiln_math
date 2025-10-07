use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use num_traits::{One, Zero};
use crate::vector::{Vector2, Vector3, Vector4};

//--------
// Matrix2
//--------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix2<T> {
    pub n11: T, pub n12: T,
    pub n21: T, pub n22: T
}

impl<T> Matrix2<T> {
    pub fn new(
     n11: T, n12: T, 
        n21: T, n22: T
    ) -> Self {
        Self {
         n11, n12,
            n21, n22
        }
    }
}

impl<T> Matrix2<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            n11: T::zero(),
            n12: T::zero(),
            n21: T::zero(),
            n22: T::zero()
        }
    }
}

impl<T> Matrix2<T> where T: One {
    pub fn one() -> Self {
        Self {
            n11: T::one(),
            n12: T::one(),
            n21: T::one(),
            n22: T::one()
        }
    }
}
// Matrix2 Addition

impl<T> Add for Matrix2<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 + rhs.n11,
            n12: self.n12 + rhs.n12,
            n21: self.n21 + rhs.n21,
            n22: self.n22 + rhs.n22
        }
    }
}

impl<T> AddAssign for Matrix2<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.n11 += rhs.n11;
        self.n12 += rhs.n12;
        self.n21 += rhs.n21;
        self.n22 += rhs.n22;
    }
}

// Matrix2 Subtraction

impl<T> Sub for Matrix2<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 - rhs.n11,
            n12: self.n12 - rhs.n12,
            n21: self.n21 - rhs.n21,
            n22: self.n22 - rhs.n22
        }
    }
}

impl<T> SubAssign for Matrix2<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n11 -= rhs.n11;
        self.n12 -= rhs.n12;
        self.n21 -= rhs.n21;
        self.n21 -= rhs.n22;
    }
}

// Matrix2 Multiplication

impl<T> Mul for Matrix2<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 * rhs.n11,
            n12: self.n12 * rhs.n12,
            n21: self.n21 * rhs.n21,
            n22: self.n22 * rhs.n22
        }
    }
}

impl<T> MulAssign for Matrix2<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.n11 *= rhs.n11;
        self.n12 *= rhs.n12;
        self.n21 *= rhs.n21;
        self.n22 *= rhs.n22;
    }
}

// Matrix 2 Division

impl<T> Div for Matrix2<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 / rhs.n11,
            n12: self.n12 / rhs.n12,
            n21: self.n21 / rhs.n21,
            n22: self.n22 / rhs.n22
        }
    }
}

impl<T> DivAssign for Matrix2<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.n11 /= rhs.n11;
        self.n12 /= rhs.n12;
        self.n21 /= rhs.n21;
        self.n22 /= rhs.n22;
    }
}

// Matrix2 Remainder

impl<T> Rem for Matrix2<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 % rhs.n11,
            n12: self.n12 % rhs.n12,
            n21: self.n21 % rhs.n21,
            n22: self.n22 % rhs.n22
        }
    }
}

impl<T> RemAssign for Matrix2<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.n11 %= rhs.n11;
        self.n12 %= rhs.n12;
        self.n21 %= rhs.n21;
        self.n22 %= rhs.n22;
    }
}

//--------
// Matrix3
//--------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix3<T> {
    pub n11: T, pub n12: T, pub n13: T,
    pub n21: T, pub n22: T, pub n23: T,
    pub n31: T, pub n32: T, pub n33: T
}

impl<T> Matrix3<T> {
        pub fn new(
     n11: T, n12: T, n13: T,
        n21: T, n22: T, n23: T,
        n31: T, n32: T, n33: T
    ) -> Self {
        Self {
         n11, n12, n13,
            n21, n22, n23,
            n31, n32, n33
        }
    }
}

impl<T> Matrix3<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            n11: T::zero(),
            n12: T::zero(),
            n13: T::zero(),
            n21: T::zero(),
            n22: T::zero(),
            n23: T::zero(),
            n31: T::zero(),
            n32: T::zero(),
            n33: T::zero()
        }
    }
}

impl<T> Matrix3<T> where T: One {
    pub fn one() -> Self {
        Self {
            n11: T::one(),
            n12: T::one(),
            n13: T::one(),
            n21: T::one(),
            n22: T::one(),
            n23: T::one(),
            n31: T::one(),
            n32: T::one(),
            n33: T::one()
        }
    }
}

// Matrix3 Addition

impl<T> Add for Matrix3<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 + rhs.n11,
            n12: self.n12 + rhs.n12,
            n13: self.n13 + rhs.n13,
            n21: self.n21 + rhs.n21,
            n22: self.n22 + rhs.n22,
            n23: self.n23 + rhs.n23,
            n31: self.n31 + rhs.n31,
            n32: self.n32 + rhs.n32,
            n33: self.n33 + rhs.n33
        }
    }
}

impl<T> AddAssign for Matrix3<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.n11 += rhs.n11;
        self.n12 += rhs.n12;
        self.n13 += rhs.n13;
        self.n21 += rhs.n21;
        self.n22 += rhs.n22;
        self.n23 += rhs.n23;
        self.n31 += rhs.n31;
        self.n32 += rhs.n32;
        self.n33 += rhs.n33;
    }
}

// Matrix3 Subtraction

impl<T> Sub for Matrix3<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 - rhs.n11,
            n12: self.n12 - rhs.n12,
            n13: self.n13 - rhs.n13,
            n21: self.n21 - rhs.n21,
            n22: self.n22 - rhs.n22,
            n23: self.n23 - rhs.n23,
            n31: self.n31 - rhs.n31,
            n32: self.n32 - rhs.n32,
            n33: self.n33 - rhs.n33
        }
    }
}

impl<T> SubAssign for Matrix3<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n11 -= rhs.n11;
        self.n12 -= rhs.n12;
        self.n13 -= rhs.n13;
        self.n21 -= rhs.n21;
        self.n21 -= rhs.n22;
        self.n23 -= rhs.n23;
        self.n31 -= rhs.n31;
        self.n32 -= rhs.n32;
        self.n33 -= rhs.n33;
    }
}

// Matrix3 Multiplication

impl<T> Mul for Matrix3<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 * rhs.n11,
            n12: self.n12 * rhs.n12,
            n13: self.n13 * rhs.n13,
            n21: self.n21 * rhs.n21,
            n22: self.n22 * rhs.n22,
            n23: self.n23 * rhs.n23,
            n31: self.n31 * rhs.n31,
            n32: self.n32 * rhs.n32,
            n33: self.n33 * rhs.n33
        }
    }
}

impl<T> MulAssign for Matrix3<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.n11 *= rhs.n11;
        self.n12 *= rhs.n12;
        self.n13 *= rhs.n13;
        self.n21 *= rhs.n21;
        self.n22 *= rhs.n22;
        self.n23 *= rhs.n23;
        self.n31 *= rhs.n31;
        self.n32 *= rhs.n32;
        self.n33 *= rhs.n33;
    }
}

// Matrix3 Division

impl<T> Div for Matrix3<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 / rhs.n11,
            n12: self.n12 / rhs.n12,
            n13: self.n13 / rhs.n13,
            n21: self.n21 / rhs.n21,
            n22: self.n22 / rhs.n22,
            n23: self.n23 / rhs.n23,
            n31: self.n31 / rhs.n31,
            n32: self.n32 / rhs.n32,
            n33: self.n33 / rhs.n33
        }
    }
}

impl<T> DivAssign for Matrix3<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.n11 /= rhs.n11;
        self.n12 /= rhs.n12;
        self.n13 /= rhs.n13;
        self.n21 /= rhs.n21;
        self.n22 /= rhs.n22;
        self.n23 /= rhs.n23;
        self.n31 /= rhs.n31;
        self.n32 /= rhs.n32;
        self.n33 /= rhs.n33;
    }
}

// Matrix3 Remainder

impl<T> Rem for Matrix3<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 % rhs.n11,
            n12: self.n12 % rhs.n12,
            n13: self.n13 % rhs.n13,
            n21: self.n21 % rhs.n21,
            n22: self.n22 % rhs.n22,
            n23: self.n23 % rhs.n23,
            n31: self.n31 % rhs.n31,
            n32: self.n32 % rhs.n32,
            n33: self.n33 % rhs.n33
        }
    }
}

impl<T> RemAssign for Matrix3<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.n11 %= rhs.n11;
        self.n12 %= rhs.n12;
        self.n13 %= rhs.n13;
        self.n21 %= rhs.n21;
        self.n22 %= rhs.n22;
        self.n23 %= rhs.n23;
        self.n31 %= rhs.n31;
        self.n32 %= rhs.n32;
        self.n33 %= rhs.n33;
    }
}

//--------
// Matrix4
//--------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Matrix4<T> {
    pub n11: T, pub n12: T, pub n13: T, pub n14: T,
    pub n21: T, pub n22: T, pub n23: T, pub n24: T,
    pub n31: T, pub n32: T, pub n33: T, pub n34: T,
    pub n41: T, pub n42: T, pub n43: T, pub n44: T
}

impl<T> Matrix4<T> {
        pub fn new(
     n11: T, n12: T, n13: T, n14: T,
        n21: T, n22: T, n23: T, n24: T,
        n31: T, n32: T, n33: T, n34: T,
        n41: T, n42: T, n43: T, n44: T
    ) -> Self {
        Self {
         n11, n12, n13, n14,
            n21, n22, n23, n24,
            n31, n32, n33, n34,
            n41, n42, n43, n44
        }
    }
}

impl<T> Matrix4<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            n11: T::zero(),
            n12: T::zero(),
            n13: T::zero(),
            n14: T::zero(),
            n21: T::zero(),
            n22: T::zero(),
            n23: T::zero(),
            n24: T::zero(),
            n31: T::zero(),
            n32: T::zero(),
            n33: T::zero(),
            n34: T::zero(),
            n41: T::zero(),
            n42: T::zero(),
            n43: T::zero(),
            n44: T::zero()
        }
    }
}

impl<T> Matrix4<T> where T: One {
    pub fn one() -> Self {
        Self {
            n11: T::one(),
            n12: T::one(),
            n13: T::one(),
            n14: T::one(),
            n21: T::one(),
            n22: T::one(),
            n23: T::one(),
            n24: T::one(),
            n31: T::one(),
            n32: T::one(),
            n33: T::one(),
            n34: T::one(),
            n41: T::one(),
            n42: T::one(),
            n43: T::one(),
            n44: T::one()           
        }
    }
}

// Matrix4 Addition

impl<T> Add for Matrix4<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 + rhs.n11,
            n12: self.n12 + rhs.n12,
            n13: self.n13 + rhs.n13,
            n14: self.n14 + rhs.n14,
            n21: self.n21 + rhs.n21,
            n22: self.n22 + rhs.n22,
            n23: self.n23 + rhs.n23,
            n24: self.n24 + rhs.n24,
            n31: self.n31 + rhs.n31,
            n32: self.n32 + rhs.n32,
            n33: self.n33 + rhs.n33,
            n34: self.n34 + rhs.n34,
            n41: self.n41 + rhs.n41,
            n42: self.n42 + rhs.n42,
            n43: self.n43 + rhs.n43,
            n44: self.n44 + rhs.n44
        }
    }
}

impl<T> AddAssign for Matrix4<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.n11 += rhs.n11;
        self.n12 += rhs.n12;
        self.n13 += rhs.n13;
        self.n14 += rhs.n14;
        self.n21 += rhs.n21;
        self.n22 += rhs.n22;
        self.n23 += rhs.n23;
        self.n24 += rhs.n24;
        self.n31 += rhs.n31;
        self.n32 += rhs.n32;
        self.n33 += rhs.n33;
        self.n34 += rhs.n34;
        self.n41 += rhs.n41;
        self.n42 += rhs.n42;
        self.n43 += rhs.n43;
        self.n44 += rhs.n44;
    }
}

// Matrix4 Subtraction

impl<T> Sub for Matrix4<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 - rhs.n11,
            n12: self.n12 - rhs.n12,
            n13: self.n13 - rhs.n13,
            n14: self.n14 - rhs.n14,
            n21: self.n21 - rhs.n21,
            n22: self.n22 - rhs.n22,
            n23: self.n23 - rhs.n23,
            n24: self.n24 - rhs.n24,
            n31: self.n31 - rhs.n31,
            n32: self.n32 - rhs.n32,
            n33: self.n33 - rhs.n33,
            n34: self.n34 - rhs.n34,
            n41: self.n41 - rhs.n41,
            n42: self.n42 - rhs.n42,
            n43: self.n43 - rhs.n43,
            n44: self.n44 - rhs.n44
        }
    }
}

impl<T> SubAssign for Matrix4<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n11 -= rhs.n11;
        self.n12 -= rhs.n12;
        self.n13 -= rhs.n13;
        self.n14 -= rhs.n14;
        self.n21 -= rhs.n21;
        self.n21 -= rhs.n22;
        self.n23 -= rhs.n23;
        self.n24 -= rhs.n24;
        self.n31 -= rhs.n31;
        self.n32 -= rhs.n32;
        self.n33 -= rhs.n33;
        self.n34 -= rhs.n34;
        self.n41 -= rhs.n41;
        self.n42 -= rhs.n42;
        self.n43 -= rhs.n43;
        self.n44 -= rhs.n44;
    }
}

// Matrix4 Multiplication

impl<T> Mul for Matrix4<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 * rhs.n11,
            n12: self.n12 * rhs.n12,
            n13: self.n13 * rhs.n13,
            n14: self.n14 * rhs.n14,
            n21: self.n21 * rhs.n21,
            n22: self.n22 * rhs.n22,
            n23: self.n23 * rhs.n23,
            n24: self.n24 * rhs.n24,
            n31: self.n31 * rhs.n31,
            n32: self.n32 * rhs.n32,
            n33: self.n33 * rhs.n33,
            n34: self.n34 * rhs.n34,
            n41: self.n41 * rhs.n41,
            n42: self.n42 * rhs.n42,
            n43: self.n43 * rhs.n43,
            n44: self.n44 * rhs.n44
        }
    }
}

impl<T> MulAssign for Matrix4<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.n11 *= rhs.n11;
        self.n12 *= rhs.n12;
        self.n13 *= rhs.n13;
        self.n14 *= rhs.n14;
        self.n21 *= rhs.n21;
        self.n22 *= rhs.n22;
        self.n23 *= rhs.n23;
        self.n24 *= rhs.n24;
        self.n31 *= rhs.n31;
        self.n32 *= rhs.n32;
        self.n33 *= rhs.n33;
        self.n34 *= rhs.n34;
        self.n41 *= rhs.n41;
        self.n42 *= rhs.n42;
        self.n43 *= rhs.n43;
        self.n44 *= rhs.n44;
    }
}

// Matrix4 Division

impl<T> Div for Matrix4<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 / rhs.n11,
            n12: self.n12 / rhs.n12,
            n13: self.n13 / rhs.n13,
            n14: self.n14 / rhs.n14,
            n21: self.n21 / rhs.n21,
            n22: self.n22 / rhs.n22,
            n23: self.n23 / rhs.n23,
            n24: self.n24 / rhs.n24,
            n31: self.n31 / rhs.n31,
            n32: self.n32 / rhs.n32,
            n33: self.n33 / rhs.n33,
            n34: self.n34 / rhs.n34,
            n41: self.n41 / rhs.n41,
            n42: self.n42 / rhs.n42,
            n43: self.n43 / rhs.n43,
            n44: self.n44 / rhs.n44
        }
    }
}

impl<T> DivAssign for Matrix4<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.n11 /= rhs.n11;
        self.n12 /= rhs.n12;
        self.n13 /= rhs.n13;
        self.n14 /= rhs.n14;
        self.n21 /= rhs.n21;
        self.n22 /= rhs.n22;
        self.n23 /= rhs.n23;
        self.n24 /= rhs.n24;
        self.n31 /= rhs.n31;
        self.n32 /= rhs.n32;
        self.n33 /= rhs.n33;
        self.n34 /= rhs.n34;
        self.n41 /= rhs.n41;
        self.n42 /= rhs.n42;
        self.n43 /= rhs.n43;
        self.n44 /= rhs.n44;
    }
}

// Matrix4 Remainder

impl<T> Rem for Matrix4<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            n11: self.n11 % rhs.n11,
            n12: self.n12 % rhs.n12,
            n13: self.n13 % rhs.n13,
            n14: self.n14 % rhs.n14,
            n21: self.n21 % rhs.n21,
            n22: self.n22 % rhs.n22,
            n23: self.n23 % rhs.n23,
            n24: self.n24 % rhs.n24,
            n31: self.n31 % rhs.n31,
            n32: self.n32 % rhs.n32,
            n33: self.n33 % rhs.n33,
            n34: self.n34 % rhs.n34,
            n41: self.n41 % rhs.n41,
            n42: self.n42 % rhs.n42,
            n43: self.n43 % rhs.n43,
            n44: self.n44 % rhs.n44
        }
    }
}

impl<T> RemAssign for Matrix4<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.n11 %= rhs.n11;
        self.n12 %= rhs.n12;
        self.n13 %= rhs.n13;
        self.n14 %= rhs.n14;
        self.n21 %= rhs.n21;
        self.n22 %= rhs.n22;
        self.n23 %= rhs.n23;
        self.n24 %= rhs.n24;
        self.n31 %= rhs.n31;
        self.n32 %= rhs.n32;
        self.n33 %= rhs.n33;
        self.n34 %= rhs.n34;
        self.n41 %= rhs.n41;
        self.n42 %= rhs.n42;
        self.n43 %= rhs.n43;
        self.n44 %= rhs.n44;
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