use rustle_math::point::Point2;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};


#[test]
fn point2_impl_test() {
    let p = Point2::new(0, 0);
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
}

#[test]
fn point2_zero_test() {
    let p: Point2<i32> = Point2::zero();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
}

#[test]
fn point2_one_test() {
    let p: Point2<i32> = Point2::one();
    assert_eq!(p.x, 1);
    assert_eq!(p.y, 1);
}

#[test]
fn point2_up_test() {
    let p: Point2<i32> = Point2::up();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 1);
}

#[test]
fn point2_right_test() {
    let p: Point2<i32> = Point2::right();
    assert_eq!(p.x, 1);
    assert_eq!(p.y, 0);
}

#[test]
fn point2_down_test() {
    let p: Point2<i32> = Point2::down();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, -1);
}

#[test]
fn point2_left_test() {
    let p: Point2<i32> = Point2::left();
    assert_eq!(p.x, -1);
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

#[test]
fn point2_mul_test() {
    let p1 = Point2::new(3, 3);
    let p2 = Point2::new(2, 2);
    let p3 = p1.mul(p2);
    assert_eq!(p3.x, 6);
    assert_eq!(p3.y, 6);
}

#[test]
fn point2_mul_assign_test() {
    let mut p1 = Point2::new(3, 3);
    let p2 = Point2::new(2, 2);
    p1.mul_assign(p2);
    assert_eq!(p1.x, 6);
    assert_eq!(p1.y, 6);
}

#[test]
fn point2_div_test() {
    let p1 = Point2::new(6, 6);
    let p2 = Point2::new(2, 2);
    let p3 = p1.div(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
}

#[test]
fn point2_div_assign_test() {
    let mut p1 = Point2::new(6, 6);
    let p2 = Point2::new(2, 2);
    p1.div_assign(p2);
    assert_eq!(p1.x, 3);
    assert_eq!(p1.y, 3);
}


