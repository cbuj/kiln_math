use kiln_math::point::Point2;

#[test]
fn new() {
    let p2: Point2<f32> = Point2::new(2.0f32, 2.0f32);
    assert_eq!(p2.x, 2.0f32);
    assert_eq!(p2.y, 2.0f32);
}

#[test]
fn zero() {
    let p2: Point2<f32> = Point2::zero();
    assert_eq!(p2.x, 0.0f32);
    assert_eq!(p2.y, 0.0f32);
}

#[test]
fn is_zero() {
    let p2: Point2<f32> = Point2::new(2.0f32, 2.0f32);
    assert!(!p2.is_zero());
}

#[test]
fn set_zero() {
    let mut p2: Point2<f32> = Point2::new(2.0f32, 2.0f32);
    p2.set_zero();
    assert_eq!(p2.x, 0.0f32);
    assert_eq!(p2.y, 0.0f32);
}

#[test]
fn one() {
    let p2: Point2<f32> = Point2::one();
    assert_eq!(p2.x, 1.0f32);
    assert_eq!(p2.y, 1.0f32);
}

#[test]
fn is_one() {
    let p2: Point2<f32> = Point2::new(2.0f32, 2.0f32);
    assert!(!p2.is_one());
}

#[test]
fn set_one() {
    let mut p2: Point2<f32> = Point2::new(2.0f32, 2.0f32);
    p2.set_one();
    assert_eq!(p2.x, 1.0f32);
    assert_eq!(p2.y, 1.0f32);
}