// Generated from camera_impl.rs.tera template. Edit the template, not the generated file.

use crate::{f64::math, DAffine3, DMat3, DMat4, DQuat, DVec3, DVec4};

#[inline(always)]
#[must_use]
fn look_to_axes4<const RH: bool>(eye: DVec3, dir: DVec3, up: DVec3) -> [DVec3; 4] {
    glam_assert!(dir.is_normalized());
    glam_assert!(up.is_normalized());
    let f = if RH { dir } else { -dir };
    let s = f.cross(up).normalize();
    let u = s.cross(f);
    [
        DVec3::new(s.x, u.x, -f.x),
        DVec3::new(s.y, u.y, -f.y),
        DVec3::new(s.z, u.z, -f.z),
        DVec3::new(-eye.dot(s), -eye.dot(u), eye.dot(f)),
    ]
}

#[inline(always)]
#[must_use]
fn look_to_axes3<const RH: bool>(dir: DVec3, up: DVec3) -> [DVec3; 3] {
    glam_assert!(dir.is_normalized());
    glam_assert!(up.is_normalized());
    let f = if RH { dir } else { -dir };
    let s = f.cross(up).normalize();
    let u = s.cross(f);
    [
        DVec3::new(s.x, u.x, -f.x),
        DVec3::new(s.y, u.y, -f.y),
        DVec3::new(s.z, u.z, -f.z),
    ]
}

#[inline]
#[must_use]
pub(crate) fn look_to_mat4<const RH: bool>(eye: DVec3, dir: DVec3, up: DVec3) -> DMat4 {
    let axes = look_to_axes4::<RH>(eye, dir, up);
    DMat4::from_cols(
        axes[0].extend(0.0),
        axes[1].extend(0.0),
        axes[2].extend(0.0),
        axes[3].extend(1.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn look_to_affine3<const RH: bool>(eye: DVec3, dir: DVec3, up: DVec3) -> DAffine3 {
    let axes = look_to_axes4::<RH>(eye, dir, up);
    DAffine3 {
        matrix3: DMat3::from_cols(axes[0], axes[1], axes[2]),
        translation: axes[3],
    }
}

#[inline]
#[must_use]
pub(crate) fn look_to_mat3<const RH: bool>(dir: DVec3, up: DVec3) -> DMat3 {
    let axes = look_to_axes3::<RH>(dir, up);
    DMat3::from_cols(axes[0], axes[1], axes[2])
}

#[inline]
#[must_use]
pub(crate) fn look_to_quat<const RH: bool>(dir: DVec3, up: DVec3) -> DQuat {
    let axes = look_to_axes3::<RH>(dir, up);
    DQuat::from_rotation_axes(axes[0], axes[1], axes[2])
}

#[inline]
#[must_use]
pub(crate) fn perspective<const RH: bool, const ZO: bool, const YFLIP: bool>(
    vertical_fov: f64,
    aspect_ratio: f64,
    z_near: f64,
    z_far: f64,
) -> DMat4 {
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

    DMat4::from_cols(
        DVec4::new(xx, 0.0, 0.0, 0.0),
        DVec4::new(0.0, yy, 0.0, 0.0),
        DVec4::new(0.0, 0.0, zz, zw),
        DVec4::new(0.0, 0.0, tz, 0.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn perspective_infinite_z<const RH: bool, const ZO: bool, const YFLIP: bool>(
    vertical_fov: f64,
    aspect_ratio: f64,
    z_near: f64,
) -> DMat4 {
    glam_assert!(z_near > 0.0);
    let (sin_fov, cos_fov) = math::sin_cos(0.5 * vertical_fov);
    let h = cos_fov / sin_fov;
    let xx = h / aspect_ratio;
    let yy = if YFLIP { -h } else { h };
    let zz = if RH { -1.0 } else { 1.0 };
    let zw = zz;
    let tz = if ZO { -z_near } else { -2.0 * z_near };
    DMat4::from_cols(
        DVec4::new(xx, 0.0, 0.0, 0.0),
        DVec4::new(0.0, yy, 0.0, 0.0),
        DVec4::new(0.0, 0.0, zz, zw),
        DVec4::new(0.0, 0.0, tz, 0.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn perspective_infinite_reverse_z<const RH: bool, const YFLIP: bool>(
    vertical_fov: f64,
    aspect_ratio: f64,
    z_near: f64,
) -> DMat4 {
    glam_assert!(z_near > 0.0);
    let (sin_fov, cos_fov) = math::sin_cos(0.5 * vertical_fov);
    let h = cos_fov / sin_fov;
    let xx = h / aspect_ratio;
    let yy = if YFLIP { -h } else { h };
    let zw = if RH { -1.0 } else { 1.0 };
    let tz = z_near;
    DMat4::from_cols(
        DVec4::new(xx, 0.0, 0.0, 0.0),
        DVec4::new(0.0, yy, 0.0, 0.0),
        DVec4::new(0.0, 0.0, 0.0, zw),
        DVec4::new(0.0, 0.0, tz, 0.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn orthographic<const RH: bool, const ZO: bool, const YFLIP: bool>(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> DMat4 {
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
    DMat4::from_cols(
        DVec4::new(xx, 0.0, 0.0, 0.0),
        DVec4::new(0.0, yy, 0.0, 0.0),
        DVec4::new(0.0, 0.0, zz, 0.0),
        DVec4::new(tx, ty, tz, 1.0),
    )
}

#[inline]
#[must_use]
pub(crate) fn frustum<const RH: bool, const ZO: bool, const YFLIP: bool>(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> DMat4 {
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
    DMat4::from_cols(
        DVec4::new(xx, 0.0, 0.0, 0.0),
        DVec4::new(0.0, yy, 0.0, 0.0),
        DVec4::new(zx, zy, zz, zw),
        DVec4::new(0.0, 0.0, tz, 0.0),
    )
}
