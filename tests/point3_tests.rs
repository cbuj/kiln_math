use kiln_math::point::Point3;
use std::ops::*;

#[test]
fn point3_impl_test() {
    let p = Point3::new(0, 0, 0);
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
    assert_eq!(p.z, 0);
}

#[test]
fn point3_zero_test() {
    let p: Point3<i32> = Point3::zero();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
    assert_eq!(p.z, 0);
}

#[test]
fn point3_add_test() {
    let p1 = Point3::new(1, 1, 1);
    let p2 = Point3::new(2, 2, 2);
    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
    assert_eq!(p3.z, 3);
}

#[test]
fn point3_add_assign_test() {
    let mut p1 = Point3::new(1, 1, 1);
    let p2 = Point3::new(2, 2, 2);
    p1.add_assign(p2);
    assert_eq!(p1.x, 3);
    assert_eq!(p1.y, 3);
    assert_eq!(p1.z, 3);
}

#[test]
fn point3_sub_test() {
    let p1 = Point3::new(3, 3, 3);
    let p2 = Point3::new(2, 2, 2);
    let p3 = p1.sub(p2);
    assert_eq!(p3.x, 1);
    assert_eq!(p3.y, 1);
    assert_eq!(p3.z, 1);
}

#[test]
fn point3_sub_assign_test() {
    let mut p1 = Point3::new(3, 3, 3);
    let p2 = Point3::new(2, 2, 2);
    p1.sub_assign(p2);
    assert_eq!(p1.x, 1);
    assert_eq!(p1.y, 1);
    assert_eq!(p1.z, 1);
}

#[test]
fn point3_mul_test() {
    let p1 = Point3::new(2, 2, 2);
    let p2 = Point3::new(3, 3, 3);
    let p3 = p1.mul(p2);
    assert_eq!(p3.x, 6);
    assert_eq!(p3.y, 6);
    assert_eq!(p3.z, 6);
}

#[test]
fn point3_mul_assign_test() {
    let mut p1 = Point3::new(3, 3, 3);
    let p2 = Point3::new(2, 2, 2);
    p1.mul_assign(p2);
    assert_eq!(p1.x, 6);
    assert_eq!(p1.y, 6);
    assert_eq!(p1.z, 6);
}

#[test]
fn point3_div_test() {
    let p1 = Point3::new(6, 6, 6);
    let p2 = Point3::new(2, 2, 2);
    let p3 = p1.div(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
    assert_eq!(p3.z, 3);
}

#[test]
fn point3_div_assign_test() {
    let mut p1 = Point3::new(6, 6, 6);
    let p2 = Point3::new(2, 2, 2);
    p1.div_assign(p2);
    assert_eq!(p1.x, 3);
    assert_eq!(p1.y, 3);
    assert_eq!(p1.z, 3);
}