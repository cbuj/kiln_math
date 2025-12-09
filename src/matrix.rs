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
    pub fn get_row(self, index: usize) -> Option<Vector2<T>> {
        match index {
            0 => Some(Vector2 { x: self.n11, y: self.n12 }),
            1 => Some(Vector2 { x: self.n21, y: self.n22 }),
            _ => None
        }
    }
    pub fn get_column(self, index: usize) -> Option<Vector2<T>> {
        match index {
            0 => Some(Vector2 { x: self.n11, y: self.n21 }),
            1 => Some(Vector2 { x: self.n12, y: self.n22 }),
            _ => None
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

impl<T> Into<ColumnMatrix2<T>> for Matrix2<T> {
    fn into(self) -> ColumnMatrix2<T> {
        ColumnMatrix2 { 
            x: Vector2::new(self.n11, self.n21),
            y: Vector2::new(self.n12, self.n22)
        }
    }
}

impl<T> Into<RowMatrix2<T>> for Matrix2<T> {
    fn into(self) -> RowMatrix2<T> {
        RowMatrix2 { 
            x: Vector2::new(self.n11, self.n12), 
            y: Vector2::new(self.n21, self.n22)
        }
    }
}

impl<T> From<ColumnMatrix2<T>> for Matrix2<T> {
    fn from(column_matrix: ColumnMatrix2<T>) -> Self {
        Self {
            n11: column_matrix.x.x, n12: column_matrix.y.x,
            n21: column_matrix.x.y, n22: column_matrix.y.y
        }
    } 
}

impl<T> From<RowMatrix2<T>> for Matrix2<T> {
    fn from(row_matrix: RowMatrix2<T>) -> Self {
        Self {
            n11: row_matrix.x.x, n12: row_matrix.x.y,
            n21: row_matrix.y.x, n22: row_matrix.y.y
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
    pub fn get_row(self, index: usize) -> Option<Vector3<T>> {
        match index {
            0 => Some(Vector3::new(self.n11, self.n12, self.n13)),
            1 => Some(Vector3::new(self.n21, self.n22, self.n23)),
            2 => Some(Vector3::new(self.n31, self.n32, self.n33)),
            _ => None
        }
    }
    pub fn get_column(self, index: usize) -> Option<Vector3<T>> {
        match index {
            0 => Some(Vector3::new(self.n11, self.n21, self.n31)),
            1 => Some(Vector3::new(self.n12, self.n22, self.n32)),
            2 => Some(Vector3::new(self.n13, self.n23, self.n33)),
            _ => None
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

impl<T> Into<ColumnMatrix3<T>> for Matrix3<T> {
    fn into(self) -> ColumnMatrix3<T> {
        ColumnMatrix3 { 
            x: Vector3::new(self.n11, self.n21, self.n31), 
            y: Vector3::new(self.n12, self.n22, self.n32), 
            z: Vector3::new(self.n13, self.n23, self.n33)
        }
    }
}

impl<T> Into<RowMatrix3<T>> for Matrix3<T> {
    fn into(self) -> RowMatrix3<T> {
        RowMatrix3 { 
            x: Vector3::new(self.n11, self.n12, self.n13),
            y: Vector3::new(self.n21, self.n22, self.n23),
            z: Vector3::new(self.n31, self.n32, self.n33)
        }
    }
}

impl<T> From<ColumnMatrix3<T>> for Matrix3<T> {
    fn from(column_matrix: ColumnMatrix3<T>) -> Self {
        Self {
            n11: column_matrix.x.x, n12: column_matrix.y.x, n13: column_matrix.z.x,
            n21: column_matrix.x.y, n22: column_matrix.y.y, n23: column_matrix.z.y,
            n31: column_matrix.x.z, n32: column_matrix.y.z, n33: column_matrix.z.z
        }
    }
}

impl<T> From<RowMatrix3<T>> for Matrix3<T> {
    fn from(row_matrix: RowMatrix3<T>) -> Self {
        Self {
            n11: row_matrix.x.x, n12: row_matrix.x.y, n13: row_matrix.x.z,
            n21: row_matrix.y.x, n22: row_matrix.y.y, n23: row_matrix.y.z,
            n31: row_matrix.z.x, n32: row_matrix.z.y, n33: row_matrix.z.z
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

impl<T> Into<ColumnMatrix4<T>> for Matrix4<T> {
    fn into(self) -> ColumnMatrix4<T> {
        ColumnMatrix4 { 
            x: Vector4::new(self.n11, self.n21, self.n31, self.n41),
            y: Vector4::new(self.n12, self.n22, self.n32, self.n42),
            z: Vector4::new(self.n13, self.n23, self.n33, self.n43),
            w: Vector4::new(self.n14, self.n24, self.n34, self.n44)
        }
    }
}

impl<T> Into<RowMatrix4<T>> for Matrix4<T> {
    fn into(self) -> RowMatrix4<T> {
        RowMatrix4 {
            x: Vector4::new(self.n11, self.n12, self.n13, self.n14),
            y: Vector4::new(self.n21, self.n22, self.n23, self.n24),
            z: Vector4::new(self.n31, self.n32, self.n33, self.n34),
            w: Vector4::new(self.n41, self.n42, self.n43, self.n44)
        }
    }
}

impl<T> From<ColumnMatrix4<T>> for Matrix4<T> {
    fn from(column_matrix: ColumnMatrix4<T>) -> Self {
        Self {
            n11: column_matrix.x.x, n12: column_matrix.y.x, n13: column_matrix.z.x, n14: column_matrix.w.x,
            n21: column_matrix.x.y, n22: column_matrix.y.y, n23: column_matrix.z.y, n24: column_matrix.w.y,
            n31: column_matrix.x.z, n32: column_matrix.y.z, n33: column_matrix.z.z, n34: column_matrix.w.z,
            n41: column_matrix.x.w, n42: column_matrix.y.w, n43: column_matrix.z.w, n44: column_matrix.w.w
        }
    }
}

impl<T> From<RowMatrix4<T>> for Matrix4<T> {
    fn from(row_matrix: RowMatrix4<T>) -> Self {
        Self {
            n11: row_matrix.x.x, n12: row_matrix.x.y, n13: row_matrix.x.z, n14: row_matrix.x.w,
            n21: row_matrix.y.x, n22: row_matrix.y.y, n23: row_matrix.y.z, n24: row_matrix.y.w,
            n31: row_matrix.z.x, n32: row_matrix.z.y, n33: row_matrix.z.z, n34: row_matrix.z.w,
            n41: row_matrix.w.x, n42: row_matrix.w.y, n43: row_matrix.w.z, n44: row_matrix.w.w
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

impl<T> ColumnMatrix2<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: Vector2::zero(),
            y: Vector2::zero()
        }
    }
}

impl<T> ColumnMatrix2<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: Vector2::one(),
            y: Vector2::one()
        }
    }
}

impl<T> Into<RowMatrix2<T>> for ColumnMatrix2<T> {
    fn into(self) -> RowMatrix2<T> {
        RowMatrix2 { 
            x: Vector2::new(self.x.x, self.y.x), 
            y: Vector2::new(self.x.y, self.y.y)
        }
    }
}

impl<T> From<RowMatrix2<T>> for ColumnMatrix2<T> {
    fn from(row_matrix: RowMatrix2<T>) -> Self {
        Self {
            x: Vector2::new(row_matrix.x.x, row_matrix.y.x),
            y: Vector2::new(row_matrix.x.y, row_matrix.y.y)
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

impl<T> ColumnMatrix3<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: Vector3::zero(),
            y: Vector3::zero(),
            z: Vector3::zero()
        }
    }
}

impl<T> ColumnMatrix3<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: Vector3::one(),
            y: Vector3::one(),
            z: Vector3::one()
        }
    }
}

impl<T> Into<RowMatrix3<T>> for ColumnMatrix3<T> {
    fn into(self) -> RowMatrix3<T> {
        RowMatrix3 { 
            x: Vector3::new(self.x.x, self.y.x, self.z.x),
            y: Vector3::new(self.x.y, self.y.y, self.z.y),
            z: Vector3::new(self.x.z, self.y.z, self.z.z)
        }
    }
}

impl<T> From<RowMatrix3<T>> for ColumnMatrix3<T> {
    fn from(row_matrix: RowMatrix3<T>) -> Self {
        Self {
            x: Vector3::new(row_matrix.x.x, row_matrix.y.x, row_matrix.z.x),
            y: Vector3::new(row_matrix.x.y, row_matrix.y.y, row_matrix.z.y),
            z: Vector3::new(row_matrix.x.z, row_matrix.y.z, row_matrix.z.z)
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

impl<T> ColumnMatrix4<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: Vector4::zero(),
            y: Vector4::zero(),
            z: Vector4::zero(),
            w: Vector4::zero()
        }
    }
}

impl<T> ColumnMatrix4<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: Vector4::one(),
            y: Vector4::one(),
            z: Vector4::one(),
            w: Vector4::one()
        }
    }
}

impl<T> Into<RowMatrix4<T>> for ColumnMatrix4<T> {
    fn into(self) -> RowMatrix4<T> {
        RowMatrix4 { 
            x: Vector4::new(self.x.x, self.y.x, self.z.x, self.w.x),
            y: Vector4::new(self.x.y, self.y.y, self.z.y, self.w.y),
            z: Vector4::new(self.x.z, self.y.z, self.z.z, self.w.z),
            w: Vector4::new(self.x.w, self.y.w, self.z.w, self.w.w)
        }
    }
}
impl<T> From<RowMatrix4<T>> for ColumnMatrix4<T> {
    fn from(row_matrix: RowMatrix4<T>) -> Self {
        Self {
            x: Vector4::new(row_matrix.x.x, row_matrix.y.x, row_matrix.z.x, row_matrix.w.x),
            y: Vector4::new(row_matrix.x.y, row_matrix.y.y, row_matrix.z.y, row_matrix.w.y),
            z: Vector4::new(row_matrix.x.z, row_matrix.y.z, row_matrix.z.z, row_matrix.w.z),
            w: Vector4::new(row_matrix.x.w, row_matrix.y.w, row_matrix.z.w, row_matrix.w.w)
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

impl<T> ColumnMatrix2x3<T> {
    pub fn new(x: Vector2<T>, y: Vector2<T>, z: Vector2<T>) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

impl<T> ColumnMatrix2x3<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: Vector2::zero(),
            y: Vector2::zero(),
            z: Vector2::zero()
        }
    }
}

impl<T> ColumnMatrix2x3<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: Vector2::one(),
            y: Vector2::one(),
            z: Vector2::one()
        }
    }
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

impl<T> RowMatrix2<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: Vector2::zero(),
            y: Vector2::zero()
        }
    }
}

impl<T> RowMatrix2<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: Vector2::one(),
            y: Vector2::one()
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