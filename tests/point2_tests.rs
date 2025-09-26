use rustle_math::Point2;
use std::ops::{Add, AddAssign, Sub, SubAssign};


#[test]
fn point2_impl_test() {
    let p = Point2::new(0, 0);
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
}

#[test]
fn point2_add_test() {
    let p1 = Point2::new(1,1);
    let p2 = Point2::new(2, 2);
    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
}

#[test]
fn point2_add_assign_test() {
    let mut p1 = Point2::new(1, 1);
    let p2 = Point2::new(2, 2);
    p1.add_assign(p2);
    assert_eq!(p1.x, 3);
    assert_eq!(p1.y, 3);
}

#[test]
fn point2_sub_test() {
    let p1 = Point2::new(3, 3);
    let p2 = Point2::new(2, 2);
    let p3 = p1.sub(p2);
    assert_eq!(p3.x, 1);
    assert_eq!(p3.y, 1);
}

#[test]
fn point2_sub_assign_test() {
    let mut p1 = Point2::new(3, 3);
    let p2 = Point2::new(2, 2);
    p1.sub_assign(p2);
    assert_eq!(p1.x, 1);
    assert_eq!(p1.y, 1);

}