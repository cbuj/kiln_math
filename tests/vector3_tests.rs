use kiln_math::vector::Vector3;

#[test]
fn vector3_new_test() {
    let v = Vector3::new(1, 2, 3);
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 2);
    assert_eq!(v.z, 3);
}

#[test]
fn vector3_zero_test() {
    let v: Vector3<u32> = Vector3::zero();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 0);
    assert_eq!(v.z, 0);
}

#[test]
fn vector3_one_test() {
    let v: Vector3<u32> = Vector3::one();
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 1);
    assert_eq!(v.z, 1);
}

#[test]
fn vector3_right_test() {
    let v: Vector3<i32> = Vector3::right();
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 0);
    assert_eq!(v.z, 0);
}

#[test]
fn vector3_left_test() {
    let v: Vector3<i32> = Vector3::left();
    assert_eq!(v.x, -1);
    assert_eq!(v.y, 0);
    assert_eq!(v.z, 0);
}

#[test]
fn vector3_up_test() {
    let v: Vector3<i32> = Vector3::up();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 1);
    assert_eq!(v.z, 0);
}

#[test]
fn vector3_down_test() {
    let v: Vector3<i32> = Vector3::down();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, -1);
    assert_eq!(v.z, 0);
}

#[test]
fn vector3_forward_test() {
    let v: Vector3<i32> = Vector3::forward();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 0);
    assert_eq!(v.z, 1);
}

#[test]
fn vector3_backward_test() {
    let v: Vector3<i32> = Vector3::backward();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 0);
    assert_eq!(v.z, -1);
}

#[test]
fn vector3_min_test() {
    let v1 = Vector3::new(3, 2, 3);
    let v2 = Vector3::new(2, 3, 2);
    let v3 = v1.min(v2);
    assert_eq!(v3.x, 2);
    assert_eq!(v3.y, 2);
    assert_eq!(v3.z, 2);
}

#[test]
fn vector3_max_test() {
    let v1 = Vector3::new(3, 2, 3);
    let v2 = Vector3::new(2, 3, 2);
    let v3 = v1.max(v2);
    assert_eq!(v3.x, 3);
    assert_eq!(v3.y, 3);
    assert_eq!(v3.z, 3);
}

#[test]
fn vector3_dot_test() {
    todo!()
}

#[test]
fn vector3_cross_test() {
    todo!()
}

#[test]
fn vector3_magnitude_test() {
    todo!()
}

#[test]
fn vector3_square_magnitude_test() {
    todo!()
}

#[test]
fn vector3_clamp_magnitude_test() {
    todo!()
}

#[test]
fn vector3_normalize_test() {
    todo!()
}

#[test]
fn vector3_distance_test() {
    todo!()
}

#[test]
fn vector3_project_test() {
    todo!()
}

#[test]
fn vector3_reflect_test() {
    todo!()
}

#[test]
fn vector3_lerp_test() {
    todo!()
}

#[test]
fn vector3_lerp_unclamped_test() {
    todo!()
}

#[test]
fn vector3_slerp_test() {
    todo!()
}

#[test]
fn vector3_slerp_unclamped_test() {
    todo!()
}

#[test]
fn vector3_angle_test() {
    todo!()
}

#[test]
fn vector3_add_test() {
    todo!()
}

#[test]
fn vector3_add_assign_test() {
    todo!()
}

#[test]
fn vector3_sub_test() {
    todo!()
}

#[test]
fn vector3_sub_assign_test() {
    todo!()
}

#[test]
fn vector3_mul_test() {
    todo!()
}

#[test]
fn vector3_mul_assign_test() {
    todo!()
}

#[test]
fn vector3_div_test() {
    todo!()
}

#[test]
fn vector3_div_assign_test() {
    todo!()
}

#[test]
fn vector3_rem_test() {
    todo!()
}

#[test]
fn vector3_rem_assign_test() {
    todo!()
}
