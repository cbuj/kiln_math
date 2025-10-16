use num_traits::Float;
use kiln_math::vector::Vector2;

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
fn vector2_one_test() {
    let v: Vector2<i32> = Vector2::one();
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 1);    
}
#[test]
fn vector2_up_test() {
    let v: Vector2<i32> = Vector2::up();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 1);
}

#[test]
fn vector2_right_test() {
    let v: Vector2<i32> = Vector2::right();
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 0);
}

#[test]
fn vector2_down_test() {
    let v: Vector2<i32> = Vector2::down();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, -1);
}

#[test]
fn vector2_left_test() {
    let v: Vector2<i32> = Vector2::left();
    assert_eq!(v.x, -1);
    assert_eq!(v.y, 0);
}

#[test]
fn vector2_min_test() {
    let v1: Vector2<i32> = Vector2::new(2, 3);
    let v2: Vector2<i32> = Vector2::new(3, 2);
    let v3 = v1.min(v2);
    assert_eq!(v3.x, 2);
    assert_eq!(v3.y, 2);
}

#[test]
fn vector2_max_test() {
    let v1: Vector2<i32> = Vector2::new(2, 3);
    let v2: Vector2<i32> = Vector2::new(3, 2);
    let v3 = v1.max(v2);
    assert_eq!(v3.x, 3);
    assert_eq!(v3.y, 3);
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

#[test]
fn vector2_perpendicular_test() {
    let v: Vector2<f32> = Vector2::new(2.0, 2.0);
    let p = v.perpendicular();
    assert_eq!(p.x, -2.0);
    assert_eq!(p.y, 2.0);
}

#[test]
fn vector2_magnitude_test() {
    let v: Vector2<f32> = Vector2::new(2.0, 2.0);
    let m = v.magnitude();
    assert_eq!(m, (2.0 * 2.0 + 2.0 * 2.0).sqrt())
}

#[test]
fn vector2_square_magnitude_test() {
    let v: Vector2<f32> = Vector2::new(2.0, 2.0);
    let m = v.square_magnitude();
    assert_eq!(m, (2.0 * 2.0 + 2.0 * 2.0))
}

#[test]
fn vector2_clamp_magnitude_test() {
    let mut v: Vector2<f32> = Vector2::new(3.0, 3.0);
    let mut c = v.clamp_magnitude(2.0, 4.0);
    let mut m = v.magnitude();
    assert_eq!(c.x, v.x * 4.0 / m);
    assert_eq!(c.y, v.y * 4.0 / m);

    v = Vector2::new(2.0, 2.0);
    c = v.clamp_magnitude(3.0, 4.0);
    m = v.magnitude();
    assert_eq!(c.x, v.x * 3.0 / m);
    assert_eq!(c.y, v.y * 3.0 / m);

    v = Vector2::new(1.0, 1.0);
    c = v.clamp_magnitude(1.0, 2.0);
    assert_eq!(c.x, v.x);
    assert_eq!(c.y, v.y);
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
fn vector2_distance_test() {
    let v1: Vector2<f32> = Vector2::new(2.0, 2.0);
    let v2: Vector2<f32> = Vector2::new(3.0, 3.0);
    let d = v1.distance(v2);
    assert_eq!(d, ((3.0 - 2.0) * (3.0 - 2.0) + (3.0 - 2.0) * (3.0 - 2.0)).sqrt());
}

#[test]
fn vector2_project_test() {
    let v1: Vector2<f32> = Vector2::new(2.0, 2.0);
    let mut v2: Vector2<f32> = Vector2::new(3.0, 3.0);
    let mut p = v1.project(v2);
    let d = v1.dot(v2) / v2.square_magnitude();
    assert_eq!(p.x, v2.x * d);
    assert_eq!(p.y, v2.y * d);

    v2 = Vector2::new(0.0, 0.0);
    p = v1.project(v2);
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn vector2_reflect_test() {
    let v: Vector2<f32> = Vector2::new(2.0, 2.0);
    let n: Vector2<f32> = Vector2::new(0.0, 1.0);
    let r = v.reflect(n);
    assert_eq!(r.x, 2.0);
    assert_eq!(r.y, -2.0);
}

// Maybe add more tests for else cases
#[test]
fn vector2_lerp_test() {
    let v1: Vector2<f32> = Vector2::new(2.0, 2.0);
    let v2: Vector2<f32> = Vector2::new(4.0, 4.0);
    let t = 0.5;
    let v = v1.lerp(v2, t);
    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, 3.0);
}

#[test]
fn vector2_lerp_unclamped_test() {
    let v1: Vector2<f32> = Vector2::new(2.0, 2.0);
    let v2: Vector2<f32> = Vector2::new(4.0, 4.0);
    let t = 1.5;
    let v = v1.lerp_unclamped(v2, t);
    assert_eq!(v.x, 5.0);
    assert_eq!(v.y, 5.0);
}

#[test]
fn vector2_add_test() {
    let v1: Vector2<i32> = Vector2::new(2, 2);
    let v2: Vector2<i32> = Vector2::new(3, 3);
    let v3 = v1 + v2;
    assert_eq!(v3.x, 5);
    assert_eq!(v3.y, 5);
}

#[test]
fn vector2_add_assign_test() {
    let mut v1: Vector2<i32> = Vector2::new(2, 2);
    let v2: Vector2<i32> = Vector2::new(3, 3);
    v1 += v2;
    assert_eq!(v1.x, 5);
    assert_eq!(v1.y, 5);
}

#[test]
fn vector2_sub_test() {
    let v1: Vector2<i32> = Vector2::new(2, 2);
    let v2: Vector2<i32> = Vector2::new(3, 3);
    let v3 = v1 - v2;
    assert_eq!(v3.x, -1);
    assert_eq!(v3.y, -1);
}   

#[test]
fn vector2_sub_assign_test() {
    let mut v1: Vector2<i32> = Vector2::new(2, 2);
    let v2: Vector2<i32> = Vector2::new(3, 3);
    v1 -= v2;
    assert_eq!(v1.x, -1);
    assert_eq!(v1.y, -1);
}

#[test]
fn vector2_mul_test() {
    let v1: Vector2<i32> = Vector2::new(2, 2);
    let v2: Vector2<i32> = Vector2::new(3, 3);
    let v3 = v1 * v2;
    assert_eq!(v3.x, 6);
    assert_eq!(v3.y, 6);
}

#[test]
fn vector2_mul_assign_test() {
    let mut v1: Vector2<i32> = Vector2::new(2, 2);
    let v2: Vector2<i32> = Vector2::new(3, 3);
    v1 *= v2;
    assert_eq!(v1.x, 6);
    assert_eq!(v1.y, 6);
}

#[test]
fn vector2_div_test() {
    let v1: Vector2<f32> = Vector2::new(6.0, 6.0);
    let v2: Vector2<f32> = Vector2::new(3.0, 3.0);
    let v3 = v1 / v2;
    assert_eq!(v3.x, 2.0);
    assert_eq!(v3.y, 2.0);
}

#[test]
fn vector2_div_assign_test() {
    let mut v1: Vector2<f32> = Vector2::new(6.0, 6.0);
    let v2: Vector2<f32> = Vector2::new(3.0, 3.0);
    v1 /= v2;
    assert_eq!(v1.x, 2.0);
    assert_eq!(v1.y, 2.0);
}

#[test]
fn vector2_rem_test() {
    let v1: Vector2<i32> = Vector2::new(7, 7);
    let v2: Vector2<i32> = Vector2::new(3, 3);
    let v3 = v1 % v2;
    assert_eq!(v3.x, 1);
    assert_eq!(v3.y, 1);
}

#[test]
fn vector2_rem_assign_test() {
    let mut v1: Vector2<i32> = Vector2::new(7, 7);
    let v2: Vector2<i32> = Vector2::new(3, 3);
    v1 %= v2;
    assert_eq!(v1.x, 1);
    assert_eq!(v1.y, 1);
}
