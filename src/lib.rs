use std::{marker::PhantomData, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};
use num_traits::{Float, Zero};
//--------
// Point2
//--------

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

//-------
// Point3
//-------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3<T> {
    x: T,
    y: T,
    z: T
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
    pub fn to_vector3(self) -> Vector3<T> {
        Vector3 { 
            x: self.x, 
            y: self.y, 
            z: self.z 
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

//--------
// Vector2
//--------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x, 
            y
        }
    }
    pub fn to_point(self) -> Point2<T> {
        Point2 { 
            x: self.x, 
            y: self.y 
        }
    }
}

impl<T> Vector2<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }
}

impl<T> Vector2<T> where T: Float {
    pub fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalize(self) -> Self {
        let m = self.magnitude();
        if m.is_zero() {
            Self::zero()
        } else {
            Self {
                x: self.x / m,
                y: self.y / m
            }
        }
    }
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn cross(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

// Vector2 Addition

impl<T> Add for Vector2<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T> AddAssign for Vector2<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// Vector2 Subtraction

impl<T> Sub for Vector2<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T> SubAssign for Vector2<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// Vector2 Multiplication

impl<T> Mul for Vector2<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T> MulAssign for Vector2<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
// Vector2 Division

impl<T> Div for Vector2<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl<T> DivAssign for Vector2<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

//--------
// Vector3
//--------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
    pub fn to_point3(self) -> Point3<T> {
        Point3 { 
            x: self.x, 
            y: self.y, 
            z: self.z 
        }
    }
}
//--------
// Vector4
//--------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }
}

//------------
// EulerAngles
//------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EulerAngles<T, B> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub marker: PhantomData<B>

}
//-----------
// Quaternion
//-----------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quaternion<T> {
    pub v: Vector3<T>,
    pub s: T
}

//--------
// Matrix2
//--------

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

//--------
// Matrix3
//--------

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

//--------
// Matrix4
//--------

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

//
// ColumnMatrix2x3
//

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix2x3<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
    pub z: Vector2<T>
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix2x4<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
    pub z: Vector2<T>,
    pub w: Vector2<T>
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumMatrix3x2<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix3x4<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>,
    pub w: Vector3<T>
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix4x2<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnMatrix4x3<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>
}