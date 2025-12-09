use num_traits::{One, Zero};

use crate::vector::Vector3;

//-----------
// Quaternion
//-----------

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quaternion<T> {
    pub v: Vector3<T>,
    pub s: T
}

impl<T> Quaternion<T> {
    pub fn new(v: Vector3<T>, s: T) -> Self {
        Self {
            v,
            s
        }
    }
}

impl<T> Quaternion<T> where T: Zero {
    pub fn zero() -> Self {
        Self {
            v: Vector3::zero(),
            s: T::zero()
        }
    }
}

impl<T> Quaternion<T> where T: One {
    pub fn one() -> Self {
        Self {
            v: Vector3::one(),
            s: T::one()
        }
    }
}