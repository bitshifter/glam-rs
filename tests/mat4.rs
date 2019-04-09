use approx::assert_ulps_eq;
use glam::*;

const IDENTITY: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

const MATRIX: [[f32; 4]; 4] = [
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
];

const ZERO: [[f32; 4]; 4] = [[0.0; 4]; 4];

#[test]
fn test_mat4_identity() {
    let identity = Mat4::identity();
    assert_eq!(IDENTITY, Into::<[[f32; 4]; 4]>::into(identity));
    assert_eq!(Into::<Mat4>::into(IDENTITY), identity);
    assert_eq!(identity, identity * &identity);
}

#[test]
fn test_mat4_zero() {
    assert_eq!(Into::<Mat4>::into(ZERO), Mat4::zero());
}

#[test]
fn test_mat4_accessors() {
    let mut m = Mat4::zero();
    m.set_x_axis(Vec4::new(1.0, 2.0, 3.0, 4.0));
    m.set_y_axis(Vec4::new(5.0, 6.0, 7.0, 8.0));
    m.set_z_axis(Vec4::new(9.0, 10.0, 11.0, 12.0));
    m.set_w_axis(Vec4::new(13.0, 14.0, 15.0, 16.0));
    assert_eq!(Into::<Mat4>::into(MATRIX), m);
    assert_eq!(Vec4::new(1.0, 2.0, 3.0, 4.0), m.get_x_axis());
    assert_eq!(Vec4::new(5.0, 6.0, 7.0, 8.0), m.get_y_axis());
    assert_eq!(Vec4::new(9.0, 10.0, 11.0, 12.0), m.get_z_axis());
    assert_eq!(Vec4::new(13.0, 14.0, 15.0, 16.0), m.get_w_axis());
}

#[test]
fn test_mat4_from_axes() {
    let a: Mat4 = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ]
    .into();
    assert_eq!(MATRIX, Into::<[[f32; 4]; 4]>::into(a));
    let b = Mat4::from_axes(
        vec4(1.0, 2.0, 3.0, 4.0),
        vec4(5.0, 6.0, 7.0, 8.0),
        vec4(9.0, 10.0, 11.0, 12.0),
        vec4(13.0, 14.0, 15.0, 16.0),
    );
    assert_eq!(a, b);
    let c = mat4(
        vec4(1.0, 2.0, 3.0, 4.0),
        vec4(5.0, 6.0, 7.0, 8.0),
        vec4(9.0, 10.0, 11.0, 12.0),
        vec4(13.0, 14.0, 15.0, 16.0),
    );
    assert_eq!(a, c);
}

#[test]
fn test_mat4_translation() {
    let translate = Mat4::from_translation(vec3(1.0, 2.0, 3.0));
    assert_eq!(
        Mat4::from_axes(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(1.0, 2.0, 3.0, 1.0)
        ),
        translate
    );
}

#[test]
fn test_from_rotation() {
    let rot_x1 = Mat4::from_rotation_x(deg(180.0));
    let rot_x2 = Mat4::from_axis_angle(Vec3::unit_x(), deg(180.0));
    assert_ulps_eq!(rot_x1, rot_x2);
    let rot_y1 = Mat4::from_rotation_y(deg(180.0));
    let rot_y2 = Mat4::from_axis_angle(Vec3::unit_y(), deg(180.0));
    assert_ulps_eq!(rot_y1, rot_y2);
    let rot_z1 = Mat4::from_rotation_z(deg(180.0));
    let rot_z2 = Mat4::from_axis_angle(Vec3::unit_z(), deg(180.0));
    assert_ulps_eq!(rot_z1, rot_z2);
}

#[test]
fn test_mat4_mul() {
    let mat_a = Mat4::from_axis_angle(Vec3::unit_z(), deg(90.0));
    // TODO: need to create a matrix with rotation and translation, not mul them togehter
    let result3 = Vec3::unit_y().transform_normal_mat4(&mat_a);
    assert_ulps_eq!(vec3(-1.0, 0.0, 0.0), result3);
    let result4 = Vec4::unit_y() * &mat_a;
    assert_ulps_eq!(vec4(-1.0, 0.0, 0.0, 0.0), result4);
}

#[test]
fn test_from_scale() {
    let m = Mat4::from_scale(Vec3::new(2.0, 4.0, 8.0));
    assert_ulps_eq!(
        Vec3::new(1.0, 1.0, 1.0).transform_mat4(&m),
        Vec3::new(2.0, 4.0, 8.0)
    );
    assert_ulps_eq!(Vec4::unit_x() * 2.0, m.get_x_axis());
    assert_ulps_eq!(Vec4::unit_y() * 4.0, m.get_y_axis());
    assert_ulps_eq!(Vec4::unit_z() * 8.0, m.get_z_axis());
    assert_ulps_eq!(Vec4::unit_w(), m.get_w_axis());
}

#[test]
fn test_mat4_transpose() {
    let m = mat4(
        vec4(1.0, 2.0, 3.0, 4.0),
        vec4(5.0, 6.0, 7.0, 8.0),
        vec4(9.0, 10.0, 11.0, 12.0),
        vec4(13.0, 14.0, 15.0, 16.0),
    );
    let mt = m.transpose();
    assert_eq!(mt.get_x_axis(), vec4(1.0, 5.0, 9.0, 13.0));
    assert_eq!(mt.get_y_axis(), vec4(2.0, 6.0, 10.0, 14.0));
    assert_eq!(mt.get_z_axis(), vec4(3.0, 7.0, 11.0, 15.0));
    assert_eq!(mt.get_w_axis(), vec4(4.0, 8.0, 12.0, 16.0));
}

#[test]
fn test_mat4_det() {
    assert_eq!(0.0, Mat4::zero().determinant());
    assert_eq!(1.0, Mat4::identity().determinant());
    assert_eq!(1.0, Mat4::from_rotation_x(deg(90.0)).determinant());
    assert_eq!(1.0, Mat4::from_rotation_y(deg(180.0)).determinant());
    assert_eq!(1.0, Mat4::from_rotation_z(deg(270.0)).determinant());
    assert_eq!(
        2.0 * 2.0 * 2.0,
        Mat4::from_scale(vec3(2.0, 2.0, 2.0)).determinant()
    );
}

#[test]
fn test_mat4_inverse() {
    assert_eq!(None, Mat4::zero().inverse());
    let inv = Mat4::identity().inverse();
    assert_ne!(None, inv);
    assert_ulps_eq!(Mat4::identity(), inv.unwrap());

    let rotz = Mat4::from_rotation_z(deg(90.0));
    let rotz_inv = rotz.inverse();
    assert_ne!(None, rotz_inv);
    let rotz_inv = rotz_inv.unwrap();
    assert_ulps_eq!(Mat4::identity(), rotz * rotz_inv);
    assert_ulps_eq!(Mat4::identity(), rotz_inv * rotz);

    let trans = Mat4::from_translation(vec3(1.0, 2.0, 3.0));
    let trans_inv = trans.inverse();
    assert_ne!(None, trans_inv);
    let trans_inv = trans_inv.unwrap();
    assert_ulps_eq!(Mat4::identity(), trans * trans_inv);
    assert_ulps_eq!(Mat4::identity(), trans_inv * trans);

    let scale = Mat4::from_scale(vec3(4.0, 5.0, 6.0));
    let scale_inv = scale.inverse();
    assert_ne!(None, scale_inv);
    let scale_inv = scale_inv.unwrap();
    assert_ulps_eq!(Mat4::identity(), scale * scale_inv);
    assert_ulps_eq!(Mat4::identity(), scale_inv * scale);

    let m = scale * rotz * trans;
    let m_inv = m.inverse();
    assert_ne!(None, m_inv);
    let m_inv = m_inv.unwrap();
    assert_ulps_eq!(Mat4::identity(), m * m_inv);
    assert_ulps_eq!(Mat4::identity(), m_inv * m);
    assert_ulps_eq!(m_inv, trans_inv * rotz_inv * scale_inv);
}

#[test]
fn test_mat4_look_at() {
    let eye = Vec3::new(0.0, 0.0, -5.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(1.0, 0.0, 0.0);
    let m = Mat4::look_at(eye, center, up);
    let point = Vec3::new(1.0, 0.0, 0.0);
    let view_point = Vec3::new(0.0, 1.0, -5.0);
    assert_ulps_eq!(point.transform_mat4(&m), view_point);
}
