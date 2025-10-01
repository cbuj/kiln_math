use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::{Float, One, Zero};
use crate::vector::{Vector2, Vector3, Vector4};
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