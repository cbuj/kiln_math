pub mod matrix;
pub mod point;
pub mod vector;
pub mod quaternion;

use std::fmt::Debug;
use std::{marker::PhantomData};

use num_traits::Num;

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



