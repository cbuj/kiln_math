use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use kiln_math::point::Point2;

#[test]
fn new() {
    let p: Point2<f32> = Point2::new(2.0, 2.0);
    assert_eq!(p.x, 2.0);
    assert_eq!(p.y, 2.0);
}

#[test]
fn eq() {
    let p1: Point2<f32> = Point2::new(2.0, 2.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    assert!(!p1.eq(&p2));
}

#[test]
fn ne() {
    let p1: Point2<f32> = Point2::new(2.0, 2.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    assert!(p1.ne(&p2));
}

#[test]
fn zero() {
    let p: Point2<f32> = Point2::zero();
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn is_zero() {
    let p: Point2<f32> = Point2::new(2.0, 2.0);
    assert!(!p.is_zero());
}

#[test]
fn set_zero() {
    let mut p: Point2<f32> = Point2::new(2.0, 2.0);
    p.set_zero();
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn one() {
    let p: Point2<f32> = Point2::one();
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 1.0);
}

#[test]
fn is_one() {
    let p: Point2<f32> = Point2::new(2.0, 2.0);
    assert!(!p.is_one());
}

#[test]
fn set_one() {
    let mut p: Point2<f32> = Point2::new(2.0, 2.0);
    p.set_one();
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 1.0);
}

#[test]
fn add() {
    let p1: Point2<f32> = Point2::new(2.0, 2.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    let p3: Point2<f32> = p1.add(p2);
    assert_eq!(p3.x, 5.0);
    assert_eq!(p3.y, 5.0);
}

#[test]
fn add_assign() {
    let mut p1: Point2<f32> = Point2::new(2.0, 2.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    p1.add_assign(p2);
    assert_eq!(p1.x, 5.0);
    assert_eq!(p1.y, 5.0);
}

#[test]
fn sub() {
    let p1: Point2<f32> = Point2::new(5.0, 5.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    let p3: Point2<f32> = p1.sub(p2);
    assert_eq!(p3.x, 2.0);
    assert_eq!(p3.y, 2.0);
}

#[test]
fn sub_assign() {
    let mut p1: Point2<f32> = Point2::new(5.0, 5.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    p1.sub_assign(p2);
    assert_eq!(p1.x, 2.0);
    assert_eq!(p1.y, 2.0);
}

#[test]
fn mul() {
    let p1: Point2<f32> = Point2::new(2.0, 2.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    let p3: Point2<f32> = p1.mul(p2);
    assert_eq!(p3.x, 6.0);
    assert_eq!(p3.y, 6.0);
}

#[test]
fn mul_assign() {
    let mut p1: Point2<f32> = Point2::new(2.0, 2.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    p1.mul_assign(p2);
    assert_eq!(p1.x, 6.0);
    assert_eq!(p1.y, 6.0);
}

#[test]
fn div() {
    let p1: Point2<f32> = Point2::new(6.0, 6.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    let p3: Point2<f32> = p1.div(p2);
    assert_eq!(p3.x, 2.0);
    assert_eq!(p3.y, 2.0);
}

#[test]
fn div_assign() {
    let mut p1: Point2<f32> = Point2::new(6.0, 6.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    p1.div_assign(p2);
    assert_eq!(p1.x, 2.0);
    assert_eq!(p1.y, 2.0);
}

#[test]
fn rem() {
    let p1: Point2<f32> = Point2::new(5.0, 5.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    let p3: Point2<f32> = p1.rem(p2);
    assert_eq!(p3.x, 2.0);
    assert_eq!(p3.y, 2.0);
}

#[test]
fn rem_assign() {
    let mut p1: Point2<f32> = Point2::new(5.0, 5.0);
    let p2: Point2<f32> = Point2::new(3.0, 3.0);
    p1.rem_assign(p2);
    assert_eq!(p1.x, 2.0);
    assert_eq!(p1.y, 2.0);
}