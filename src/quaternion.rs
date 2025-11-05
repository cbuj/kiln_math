use crate::vector::Vector3;

//-----------
// Quaternion
//-----------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quaternion<T> {
    pub v: Vector3<T>,
    pub s: T
}