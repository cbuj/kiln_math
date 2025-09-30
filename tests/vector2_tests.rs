use num_traits::Float;
use rustle_math::Vector2;
use std::ops::*;

#[test]
fn vector2_impl_test() {
    let v = Vector2::new(0, 0);
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 0);
}

#[test]
fn vector2_zero_test() {
    let v: Vector2<i32> = Vector2::zero();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 0);    
}

#[test]
fn vector2_magnitude_test() {
    let v: Vector2<f32> = Vector2::new(2.0, 2.0);
    let m = v.magnitude();
    assert_eq!(m, (2.0 * 2.0 + 2.0 * 2.0).sqrt())
}

#[test]
fn vector2_normalize_test() {
    let v: Vector2<f32> = Vector2::new(2.0, 2.0);
    let n = v.normalize();
    let m = (2.0 * 2.0 + 2.0 * 2.0).sqrt();
    assert_eq!(n.x, v.x / m);
    assert_eq!(n.y, v.y / m);
}

#[test]
fn vector2_dot_test() {
    let v1: Vector2<f32> = Vector2::new(2.0, 2.0);
    let v2: Vector2<f32> = Vector2::new(3.0, 3.0);
    let s = v1.dot(v2);
    assert_eq!(s, (2.0 * 3.0 + 2.0 * 3.0));
}

#[test]
fn vector2_cross_test() {
    let v1: Vector2<f32> = Vector2::new(2.0, 2.0);
    let v2: Vector2<f32> = Vector2::new(3.0, 3.0);
    let s = v1.cross(v2);
    assert_eq!(s, (2.0 * 3.0 - 2.0 * 3.0));
}
