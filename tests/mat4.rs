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
    assert_eq!(identity, identity * identity);
}

#[test]
fn test_mat4_zero() {
    assert_eq!(Into::<Mat4>::into(ZERO), Mat4::zero());
}

#[test]
fn test_mat4_new() {
    let a: Mat4 = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ]
    .into();
    assert_eq!(MATRIX, Into::<[[f32; 4]; 4]>::into(a));
    let b = Mat4::from_cols(
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
        Mat4::from_cols(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(1.0, 2.0, 3.0, 1.0)
        ),
        translate
    );
}

#[test]
fn test_mat4_mul() {
    let x_axis = vec3(1.0, 0.0, 0.0);
    let y_axis = vec3(0.0, 1.0, 0.0);
    let z_axis = vec3(0.0, 0.0, 1.0);
    let rotation_around_z = Mat4::from_axis_angle(z_axis, deg(90.0));
    let translate_x = Mat4::from_translation(x_axis);
    let mat_a = rotation_around_z * translate_x;
    let result3 = y_axis * mat_a;
    assert_ulps_eq!(vec3(-1.0, 0.0, 0.0), result3);
    let result4 = y_axis.extend(0.0) * mat_a;
    assert_ulps_eq!(vec4(-1.0, 0.0, 0.0, 0.0), result4);
}
