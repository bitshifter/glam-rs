// Generated from camera_impl.rs.tera template. Edit the template, not the generated file.

use crate::{f32::math, Affine3, Affine3A, Mat3, Mat3A, Mat4, Quat, Vec3, Vec3A, Vec4};

#[inline(always)]
#[must_use]
fn look_to_axes4<const RH: bool>(eye: Vec3, dir: Vec3, up: Vec3) -> [Vec3; 4] {
    glam_assert!(dir.is_normalized());
    glam_assert!(up.is_normalized());
    let f = if RH { dir } else { -dir };
    let s = f.cross(up).normalize();
    let u = s.cross(f);
    [
        Vec3::new(s.x, u.x, -f.x),
        Vec3::new(s.y, u.y, -f.y),
        Vec3::new(s.z, u.z, -f.z),
        Vec3::new(-eye.dot(s), -eye.dot(u), eye.dot(f)),
    ]
}

#[inline(always)]
#[must_use]
fn look_to_axes3<const RH: bool>(dir: Vec3, up: Vec3) -> [Vec3; 3] {
    glam_assert!(dir.is_normalized());
    glam_assert!(up.is_normalized());
    let f = if RH { dir } else { -dir };
    let s = f.cross(up).normalize();
    let u = s.cross(f);
    [
        Vec3::new(s.x, u.x, -f.x),
        Vec3::new(s.y, u.y, -f.y),
        Vec3::new(s.z, u.z, -f.z),
    ]
}

#[inline]
#[must_use]
pub(crate) fn look_to_mat4<const RH: bool>(eye: Vec3, dir: Vec3, up: Vec3) -> Mat4 {
    let axes = look_to_axes4::<RH>(eye, dir, up);
    Mat4::from_cols(
        axes[0].extend(0.0),
        axes[1].extend(0.0),
        axes[2].extend(0.0),
        axes[3].extend(1.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn look_to_affine3<const RH: bool>(eye: Vec3, dir: Vec3, up: Vec3) -> Affine3 {
    let axes = look_to_axes4::<RH>(eye, dir, up);
    Affine3 {
        matrix3: Mat3::from_cols(axes[0], axes[1], axes[2]),
        translation: axes[3],
    }
}

#[inline]
#[must_use]
pub(crate) fn look_to_affine3a<const RH: bool>(eye: Vec3, dir: Vec3, up: Vec3) -> Affine3A {
    let axes = look_to_axes4::<RH>(eye, dir, up);
    Affine3A {
        matrix3: Mat3A::from_cols(
            Vec3A::from(axes[0]),
            Vec3A::from(axes[1]),
            Vec3A::from(axes[2]),
        ),
        translation: Vec3A::from(axes[3]),
    }
}

#[inline]
#[must_use]
pub(crate) fn look_to_mat3a<const RH: bool>(dir: Vec3, up: Vec3) -> Mat3A {
    let axes = look_to_axes3::<RH>(dir, up);
    Mat3A::from_cols(
        Vec3A::from(axes[0]),
        Vec3A::from(axes[1]),
        Vec3A::from(axes[2]),
    )
}

#[inline]
#[must_use]
pub(crate) fn look_to_mat3<const RH: bool>(dir: Vec3, up: Vec3) -> Mat3 {
    let axes = look_to_axes3::<RH>(dir, up);
    Mat3::from_cols(axes[0], axes[1], axes[2])
}

#[inline]
#[must_use]
pub(crate) fn look_to_quat<const RH: bool>(dir: Vec3, up: Vec3) -> Quat {
    let axes = look_to_axes3::<RH>(dir, up);
    Quat::from_rotation_axes(axes[0], axes[1], axes[2])
}

#[inline]
#[must_use]
pub(crate) fn perspective<const RH: bool, const ZO: bool, const YFLIP: bool>(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
) -> Mat4 {
    glam_assert!(z_near > 0.0 && z_far > 0.0);
    let (sin_fov, cos_fov) = math::sin_cos(0.5 * vertical_fov);
    let h = cos_fov / sin_fov;
    let xx = h / aspect_ratio;
    let yy = if YFLIP { -h } else { h };
    let z_range_inv = 1.0 / (z_far - z_near);

    let (zz, tz, zw) = match (ZO, RH) {
        (true, false) => (z_far * z_range_inv, -z_near * z_far * z_range_inv, 1.0),
        (false, false) => (
            (z_far + z_near) * z_range_inv,
            -2.0 * z_far * z_near * z_range_inv,
            1.0,
        ),
        (true, true) => (-z_far * z_range_inv, -z_near * z_far * z_range_inv, -1.0),
        (false, true) => (
            -(z_far + z_near) * z_range_inv,
            -2.0 * z_far * z_near * z_range_inv,
            -1.0,
        ),
    };

    Mat4::from_cols(
        Vec4::new(xx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, yy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, zz, zw),
        Vec4::new(0.0, 0.0, tz, 0.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn perspective_infinite<const RH: bool, const ZO: bool, const YFLIP: bool>(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
) -> Mat4 {
    glam_assert!(z_near > 0.0);
    let (sin_fov, cos_fov) = math::sin_cos(0.5 * vertical_fov);
    let h = cos_fov / sin_fov;
    let xx = h / aspect_ratio;
    let yy = if YFLIP { -h } else { h };
    let zz = if RH { -1.0 } else { 1.0 };
    let zw = zz;
    let tz = if ZO { -z_near } else { -2.0 * z_near };
    Mat4::from_cols(
        Vec4::new(xx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, yy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, zz, zw),
        Vec4::new(0.0, 0.0, tz, 0.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn perspective_infinite_reverse<const RH: bool, const YFLIP: bool>(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
) -> Mat4 {
    glam_assert!(z_near > 0.0);
    let (sin_fov, cos_fov) = math::sin_cos(0.5 * vertical_fov);
    let h = cos_fov / sin_fov;
    let xx = h / aspect_ratio;
    let yy = if YFLIP { -h } else { h };
    let zw = if RH { -1.0 } else { 1.0 };
    let tz = z_near;
    Mat4::from_cols(
        Vec4::new(xx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, yy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, zw),
        Vec4::new(0.0, 0.0, tz, 0.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn orthographic<const RH: bool, const ZO: bool, const YFLIP: bool>(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Mat4 {
    let width_inv = 1.0 / (right - left);
    let height_inv = 1.0 / (top - bottom);
    let depth_inv = 1.0 / (far - near);
    let xx = width_inv + width_inv;
    let yy = if YFLIP {
        -(height_inv + height_inv)
    } else {
        height_inv + height_inv
    };
    let zz = match (RH, ZO) {
        (true, true) => -depth_inv,
        (false, true) => depth_inv,
        (true, false) => -2.0 * depth_inv,
        (false, false) => 2.0 * depth_inv,
    };
    let tx = -(left + right) * width_inv;
    let ty = -(top + bottom) * height_inv;
    let tz = if ZO {
        -near * depth_inv
    } else {
        -(far + near) * depth_inv
    };
    Mat4::from_cols(
        Vec4::new(xx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, yy, 0.0, 0.0),
        Vec4::new(0.0, 0.0, zz, 0.0),
        Vec4::new(tx, ty, tz, 1.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn frustum<const RH: bool, const ZO: bool, const YFLIP: bool>(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Mat4 {
    glam_assert!(near > 0.0 && far > 0.0);
    let inv_width = 1.0 / (right - left);
    let inv_height = 1.0 / (top - bottom);
    let inv_depth = 1.0 / (far - near);
    let two_near = 2.0 * near;
    let xx = two_near * inv_width;
    let yy = if YFLIP {
        -two_near * inv_height
    } else {
        two_near * inv_height
    };
    let zx = (right + left) * inv_width;
    let zy = if YFLIP {
        -(top + bottom) * inv_height
    } else {
        (top + bottom) * inv_height
    };
    let (zz, zw, tz) = match (RH, ZO) {
        (true, true) => (-far * inv_depth, -1.0, -(far * near) * inv_depth),
        (false, true) => (far * inv_depth, 1.0, -(far * near) * inv_depth),
        (true, false) => (
            -(far + near) * inv_depth,
            -1.0,
            -(2.0 * far * near) * inv_depth,
        ),
        (false, false) => (
            (far + near) * inv_depth,
            1.0,
            -(2.0 * far * near) * inv_depth,
        ),
    };
    Mat4::from_cols(
        Vec4::new(xx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, yy, 0.0, 0.0),
        Vec4::new(zx, zy, zz, zw),
        Vec4::new(0.0, 0.0, tz, 0.0),
    )
}
