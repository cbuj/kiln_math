pub mod matrix;
pub mod point;
pub mod vector;

use std::fmt::Debug;
use std::{marker::PhantomData};
use vector::Vector3;

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


