use approx::assert_ulps_eq;
use glam::f32::*;

const IDENTITY: [[f32; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];

const MATRIX: [[f32; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];

const ZERO: [[f32; 2]; 2] = [[0.0; 2]; 2];

#[test]
fn test_mat2_identity() {
    let identity = Mat2::identity();
    assert_eq!(IDENTITY, Into::<[[f32; 2]; 2]>::into(identity));
    assert_eq!(Into::<Mat2>::into(IDENTITY), identity);
    assert_eq!(identity, identity * identity);
    assert_eq!(identity, Mat2::default());
}

#[test]
fn test_mat2_zero() {
    assert_eq!(Into::<Mat2>::into(ZERO), Mat2::zero());
}

#[test]
fn test_mat2_accessors() {
    let mut m = Mat2::zero();
    m.set_x_axis(Vec2::new(1.0, 2.0));
    m.set_y_axis(Vec2::new(3.0, 4.0));
    assert_eq!(Into::<Mat2>::into(MATRIX), m);
    assert_eq!(Vec2::new(1.0, 2.0), m.x_axis());
    assert_eq!(Vec2::new(3.0, 4.0), m.y_axis());
}

#[test]
fn test_mat2_from_axes() {
    let a: Mat2 = [[1.0, 2.0], [3.0, 4.0]].into();
    assert_eq!(MATRIX, Into::<[[f32; 2]; 2]>::into(a));
    let b = Mat2::new(vec2(1.0, 2.0), vec2(3.0, 4.0));
    assert_eq!(a, b);
    let c = mat2(vec2(1.0, 2.0), vec2(3.0, 4.0));
    assert_eq!(a, c);
}

#[test]
fn test_mat2_mul() {
    let mat_a = Mat2::from_angle(deg(90.0));
    let res_a = Vec2::unit_y().transform_mat2(&mat_a);
    assert_ulps_eq!(vec2(-1.0, 0.0), res_a);
    let res_b = Vec2::unit_x().transform_mat2(&mat_a);
    assert_ulps_eq!(vec2(0.0, 1.0), res_b);
}

#[test]
fn test_from_scale() {
    let m = Mat2::from_scale(Vec2::new(2.0, 4.0));
    assert_ulps_eq!(Vec2::new(1.0, 1.0).transform_mat2(&m), Vec2::new(2.0, 4.0));
    assert_ulps_eq!(Vec2::unit_x() * 2.0, m.x_axis());
    assert_ulps_eq!(Vec2::unit_y() * 4.0, m.y_axis());

    let rot = Mat2::from_scale_angle(Vec2::new(4.0, 2.0), deg(180.0));
    assert_ulps_eq!(
        Vec2::unit_x() * -4.0,
        Vec2::unit_x() * rot,
        epsilon = 1.0e-6
    );
    assert_ulps_eq!(
        Vec2::unit_y() * -2.0,
        Vec2::unit_y() * rot,
        epsilon = 1.0e-6
    );
}

#[test]
fn test_mat2_transpose() {
    let m = mat2(vec2(1.0, 2.0), vec2(3.0, 4.0));
    let mt = m.transpose();
    assert_eq!(mt.x_axis(), vec2(1.0, 3.0));
    assert_eq!(mt.y_axis(), vec2(2.0, 4.0));
}

#[test]
fn test_mat2_det() {
    assert_eq!(0.0, Mat2::zero().determinant());
    assert_eq!(1.0, Mat2::identity().determinant());
    assert_eq!(1.0, Mat2::from_angle(deg(90.0)).determinant());
    assert_eq!(1.0, Mat2::from_angle(deg(180.0)).determinant());
    assert_eq!(1.0, Mat2::from_angle(deg(270.0)).determinant());
    assert_eq!(2.0 * 2.0, Mat2::from_scale(vec2(2.0, 2.0)).determinant());
}

#[test]
fn test_mat2_inverse() {
    let inv = Mat2::identity().inverse();
    assert_ulps_eq!(Mat2::identity(), inv);

    let rot = Mat2::from_angle(deg(90.0));
    let rot_inv = rot.inverse();
    assert_ulps_eq!(Mat2::identity(), rot * rot_inv);
    assert_ulps_eq!(Mat2::identity(), rot_inv * rot);

    let scale = Mat2::from_scale(vec2(4.0, 5.0));
    let scale_inv = scale.inverse();
    assert_ulps_eq!(Mat2::identity(), scale * scale_inv);
    assert_ulps_eq!(Mat2::identity(), scale_inv * scale);

    let m = scale * rot;
    let m_inv = m.inverse();
    assert_ulps_eq!(Mat2::identity(), m * m_inv);
    assert_ulps_eq!(Mat2::identity(), m_inv * m);
    assert_ulps_eq!(m_inv, rot_inv * scale_inv);
}

#[test]
fn test_mat2_ops() {
    let m0: Mat2 = MATRIX.into();
    assert_eq!(Mat2::from([[2.0, 4.0], [6.0, 8.0]]), m0 * 2.0);
    assert_eq!(Mat2::from([[2.0, 4.0], [6.0, 8.0]]), 2.0 * m0);
    assert_eq!(Mat2::from([[2.0, 4.0], [6.0, 8.0]]), m0 + m0);
    assert_eq!(Mat2::zero(), m0 - m0);
    assert_ulps_eq!(Mat2::from([[1.0, 2.0], [3.0, 4.0]]), m0 * Mat2::identity());
    assert_ulps_eq!(Mat2::from([[1.0, 2.0], [3.0, 4.0]]), Mat2::identity() * m0);
}
