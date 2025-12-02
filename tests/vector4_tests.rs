use kiln_math::vector::Vector4;

#[test]
fn vector4_new_test() {
    let v  = Vector4::new(1, 2, 3, 4);
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 2);
    assert_eq!(v.z, 3);
    assert_eq!(v.w, 4);
}

#[test]
fn vector4_zero_test() {
    let v: Vector4<i32> = Vector4::zero();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 0);
    assert_eq!(v.z, 0);
    assert_eq!(v.w, 0);    
}

#[test]
fn vector4_one_test() {
    let v: Vector4<i32> = Vector4::one();
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 1);
    assert_eq!(v.z, 1);
    assert_eq!(v.w, 1);    
}

#[test]
fn vector4_min_test() {
    todo!()
}

#[test]
fn vector4_max_test() {
    todo!()
}

#[test]
fn vector4_dot_test() {
    todo!()
}

#[test]
fn vector4_magnitude_test() {
    todo!()
}

#[test]
fn vector4_square_magnitude_test() {
    todo!()
}

#[test]
fn vector4_clamp_magnitude_test() {
    todo!()
}

#[test]
fn vector4_normalize_test() {
    todo!()
}

#[test]
fn vector4_distance_test() {
    todo!()
}

#[test]
fn vector4_project_test() {
    todo!()
}


