use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use num_traits::{Float, Num, One, Zero};

//--------
// Vector2
//--------

#[repr(C)]
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
}

impl<T> Vector2<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }
}

impl<T> Vector2<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one()
        }
    }
}

impl<T> Vector2<T> where T: Zero + One {
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

impl<T> Vector2<T> where T: Zero + One + Neg<Output = T> {
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

impl<T> Vector2<T> where T: Ord + Copy {
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y)
        }
    }
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y)
        }
    }
}

impl<T> Vector2<T> where T: Num {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn cross(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T> Vector2<T> where T: Neg<Output = T> {
    pub fn perpendicular(self) -> Self {
        Self {
            x: self.y.neg(),
            y: self.x
        }
    }
}

impl<T> Vector2<T> where T: Float {
    pub fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn square_magnitude(self) -> T {
        self.x * self.x + self.y * self.y
    }
    pub fn clamp_magnitude(self, min_length: T, max_length: T) -> Self {
        let magnitude = self.magnitude();
        if magnitude.is_zero() {
            Self::zero()
        } else if magnitude < min_length {
            self * Self {
                x: min_length / magnitude, 
                y: min_length / magnitude
            }
        } else if magnitude > max_length {
            self * Self {
                x: max_length / magnitude, 
                y: max_length / magnitude
            }
        } else {
            self
        }
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
    pub fn distance(self, rhs: Self) -> T {
        self.sub(rhs).magnitude()
    }
    pub fn project(self, rhs: Self) -> Self {
        let dot = self.dot(rhs);
        let rhs_square_magnitude = rhs.square_magnitude();
        if rhs_square_magnitude.is_zero() {
            Self::zero()
        } else {
            rhs * Self {
                x: dot / rhs_square_magnitude,
                y: dot / rhs_square_magnitude
            }
        }
    }
    pub fn reflect(self, normal: Self) -> Self {
        let projection = self.project(normal);
        self.sub(projection.add(projection))
    }
    pub fn lerp(self, rhs: Self, t: T) -> Self {
        let t = if t < T::zero() {
            T::zero()
        } else if t > T::one() {
            T::one()
        } else {
            t
        };
        self.lerp_unclamped(rhs, t)
    }
    pub fn lerp_unclamped(self, rhs: Self, t: T) -> Self {
        self.add(rhs.sub(self).mul(Self {x: t, y: t}))
    }
    pub fn angle(self, rhs: Self) -> T {
        let dot = self.dot(rhs);
        let mags = self.magnitude() * rhs.magnitude();
        if mags.is_zero() {
            T::zero()
        } else {
            (dot / mags).acos()
        }
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

// Vector2 Remainder

impl<T> Rem for Vector2<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y
        }
    }
}

impl<T> RemAssign for Vector2<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

//--------
// Vector3
//--------

#[repr(C)]
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
}

impl<T> Vector3<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero()
        }
    }
}

impl<T> Vector3<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one()
        }
    }
}

impl<T> Vector3<T> where T: Zero + One {
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

impl<T> Vector3<T> where T: Zero + One + Neg<Output = T> {
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

// Vector3 Addition

impl<T> Add for Vector3<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T> AddAssign for Vector3<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Vector3 Subtraction

impl<T> Sub for Vector3<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T> SubAssign for Vector3<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// Vector3 Multiplication

impl<T> Mul for Vector3<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl<T> MulAssign for Vector3<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// Vector3 Division

impl<T> Div for Vector3<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl<T> DivAssign for Vector3<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// Vector3 Remainder

impl<T> Rem for Vector3<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z
        }
    }
}

impl<T> RemAssign for Vector3<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}

//--------
// Vector4
//--------

#[repr(C)]
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

impl<T> Vector4<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero()
        }
    }
}

impl<T> Vector4<T> where T: One {
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one(),
            w: T::one()
        }
    }
}

// Vector4 Addition

impl<T> Add for Vector4<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl<T> AddAssign for Vector4<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

// Vector4 Subtraction

impl<T> Sub for Vector4<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl<T> SubAssign for Vector4<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

// Vector4 Multiplication

impl<T> Mul for Vector4<T> where T: Mul<Output = T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w
        }
    }
}

impl<T> MulAssign for Vector4<T> where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

// Vector4 Division

impl<T> Div for Vector4<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w
        }
    }
}

impl<T> DivAssign for Vector4<T> where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

// Vector4 Remainder

impl<T> Rem for Vector4<T> where T: Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
            w: self.w % rhs.w
        }
    }
}

impl<T> RemAssign for Vector4<T> where T: RemAssign<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
        self.w %= rhs.w;
    }
}