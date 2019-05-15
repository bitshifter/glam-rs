use approx::assert_ulps_eq;
use glam::f32::*;

const IDENTITY: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

const MATRIX: [[f32; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];

const ZERO: [[f32; 3]; 3] = [[0.0; 3]; 3];

#[test]
fn test_mat3_identity() {
    let identity = Mat3::identity();
    assert_eq!(IDENTITY, Into::<[[f32; 3]; 3]>::into(identity));
    assert_eq!(Into::<Mat3>::into(IDENTITY), identity);
    assert_eq!(identity, identity * &identity);
}

#[test]
fn test_mat3_zero() {
    assert_eq!(Into::<Mat3>::into(ZERO), Mat3::zero());
}

#[test]
fn test_mat3_accessors() {
    let mut m = Mat3::zero();
    m.set_x_axis(Vec3::new(1.0, 2.0, 3.0));
    m.set_y_axis(Vec3::new(4.0, 5.0, 6.0));
    m.set_z_axis(Vec3::new(7.0, 8.0, 9.0));
    assert_eq!(Into::<Mat3>::into(MATRIX), m);
    assert_eq!(Vec3::new(1.0, 2.0, 3.0), m.get_x_axis());
    assert_eq!(Vec3::new(4.0, 5.0, 6.0), m.get_y_axis());
    assert_eq!(Vec3::new(7.0, 8.0, 9.0), m.get_z_axis());
}

#[test]
fn test_mat3_from_axes() {
    let a: Mat3 = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]].into();
    assert_eq!(MATRIX, Into::<[[f32; 3]; 3]>::into(a));
    let b = Mat3::new(
        vec3(1.0, 2.0, 3.0),
        vec3(4.0, 5.0, 6.0),
        vec3(7.0, 8.0, 9.0),
    );
    assert_eq!(a, b);
    let c = mat3(
        vec3(1.0, 2.0, 3.0),
        vec3(4.0, 5.0, 6.0),
        vec3(7.0, 8.0, 9.0),
    );
    assert_eq!(a, c);
}

#[test]
fn test_from_rotation() {
    let rot_x1 = Mat3::from_rotation_x(deg(180.0));
    let rot_x2 = Mat3::from_axis_angle(Vec3::unit_x(), deg(180.0));
    assert_ulps_eq!(rot_x1, rot_x2);
    let rot_y1 = Mat3::from_rotation_y(deg(180.0));
    let rot_y2 = Mat3::from_axis_angle(Vec3::unit_y(), deg(180.0));
    assert_ulps_eq!(rot_y1, rot_y2);
    let rot_z1 = Mat3::from_rotation_z(deg(180.0));
    let rot_z2 = Mat3::from_axis_angle(Vec3::unit_z(), deg(180.0));
    assert_ulps_eq!(rot_z1, rot_z2);
}

#[test]
fn test_mat3_mul() {
    let mat_a = Mat3::from_axis_angle(Vec3::unit_z(), deg(90.0));
    let result3 = Vec3::unit_y().transform_mat3(&mat_a);
    assert_ulps_eq!(vec3(-1.0, 0.0, 0.0), result3);
}

#[test]
fn test_from_ypr() {
    let zero = deg(0.0);
    let yaw = deg(30.0);
    let pitch = deg(60.0);
    let roll = deg(90.0);
    let y0 = Mat3::from_rotation_y(yaw);
    let y1 = Mat3::from_rotation_ypr(yaw, zero, zero);
    assert_ulps_eq!(y0, y1);

    let x0 = Mat3::from_rotation_x(pitch);
    let x1 = Mat3::from_rotation_ypr(zero, pitch, zero);
    assert_ulps_eq!(x0, x1);

    let z0 = Mat3::from_rotation_z(roll);
    let z1 = Mat3::from_rotation_ypr(zero, zero, roll);
    assert_ulps_eq!(z0, z1);

    let yx0 = y0 * x0;
    let yx1 = Mat3::from_rotation_ypr(yaw, pitch, zero);
    assert_ulps_eq!(yx0, yx1);

    let yxz0 = y0 * x0 * z0;
    let yxz1 = Mat3::from_rotation_ypr(yaw, pitch, roll);
    assert_ulps_eq!(yxz0, yxz1);
}

#[test]
fn test_from_scale() {
    let m = Mat3::from_scale(Vec3::new(2.0, 4.0, 8.0));
    assert_ulps_eq!(
        Vec3::new(1.0, 1.0, 1.0).transform_mat3(&m),
        Vec3::new(2.0, 4.0, 8.0)
    );
    assert_ulps_eq!(Vec3::unit_x() * 2.0, m.get_x_axis());
    assert_ulps_eq!(Vec3::unit_y() * 4.0, m.get_y_axis());
    assert_ulps_eq!(Vec3::unit_z() * 8.0, m.get_z_axis());
}

#[test]
fn test_mat3_transpose() {
    let m = mat3(
        vec3(1.0, 2.0, 3.0),
        vec3(4.0, 5.0, 6.0),
        vec3(7.0, 8.0, 9.0),
    );
    let mt = m.transpose();
    assert_eq!(mt.get_x_axis(), vec3(1.0, 4.0, 7.0));
    assert_eq!(mt.get_y_axis(), vec3(2.0, 5.0, 8.0));
    assert_eq!(mt.get_z_axis(), vec3(3.0, 6.0, 9.0));
}

#[test]
fn test_mat3_det() {
    assert_eq!(0.0, Mat3::zero().determinant());
    assert_eq!(1.0, Mat3::identity().determinant());
    assert_eq!(1.0, Mat3::from_rotation_x(deg(90.0)).determinant());
    assert_eq!(1.0, Mat3::from_rotation_y(deg(180.0)).determinant());
    assert_eq!(1.0, Mat3::from_rotation_z(deg(270.0)).determinant());
    assert_eq!(
        2.0 * 2.0 * 2.0,
        Mat3::from_scale(vec3(2.0, 2.0, 2.0)).determinant()
    );
}

#[test]
fn test_mat3_inverse() {
    // assert_eq!(None, Mat3::zero().inverse());
    let inv = Mat3::identity().inverse();
    // assert_ne!(None, inv);
    assert_ulps_eq!(Mat3::identity(), inv);

    let rotz = Mat3::from_rotation_z(deg(90.0));
    let rotz_inv = rotz.inverse();
    // assert_ne!(None, rotz_inv);
    // let rotz_inv = rotz_inv.unwrap();
    assert_ulps_eq!(Mat3::identity(), rotz * rotz_inv);
    assert_ulps_eq!(Mat3::identity(), rotz_inv * rotz);

    let scale = Mat3::from_scale(vec3(4.0, 5.0, 6.0));
    let scale_inv = scale.inverse();
    // assert_ne!(None, scale_inv);
    // let scale_inv = scale_inv.unwrap();
    assert_ulps_eq!(Mat3::identity(), scale * scale_inv);
    assert_ulps_eq!(Mat3::identity(), scale_inv * scale);

    let m = scale * rotz;
    let m_inv = m.inverse();
    // assert_ne!(None, m_inv);
    // let m_inv = m_inv.unwrap();
    assert_ulps_eq!(Mat3::identity(), m * m_inv);
    assert_ulps_eq!(Mat3::identity(), m_inv * m);
    assert_ulps_eq!(m_inv, rotz_inv * scale_inv);
}
