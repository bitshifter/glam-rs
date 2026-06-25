// Generated from camera_view.rs.tera template. Edit the template, not the generated file.

//! View (camera) matrix constructors for right-handed world coordinates.
//!
//! Every function transforms world space points into a right-handed Y-up
//! view space with X-right and -Z-forward.
//!
//! * `look_at_*` targets a focal point (`center`)
//! * `look_to_*` targets a forward direction (`dir`)

use crate::{dcamera::camera_impl, DAffine3, DMat3, DMat4, DQuat, DVec3};

/// Returns a `DMat4` view matrix from eye, focal point, and up.
///
/// Transforms right-handed world space points into right-handed Y-up view space.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat4(eye: DVec3, center: DVec3, up: DVec3) -> DMat4 {
    look_to_mat4(eye, (center - eye).normalize(), up)
}

/// Returns a `DMat4` view matrix from eye, forward direction, and up.
///
/// Transforms right-handed world space points into right-handed Y-up view space.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat4(eye: DVec3, dir: DVec3, up: DVec3) -> DMat4 {
    camera_impl::look_to_mat4::<true>(eye, dir, up)
}

/// Returns an `DAffine3` view transform from eye, focal point, and up.
///
/// Transforms right-handed world space points into right-handed Y-up view space.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_affine3(eye: DVec3, center: DVec3, up: DVec3) -> DAffine3 {
    look_to_affine3(eye, (center - eye).normalize(), up)
}

/// Returns an `DAffine3` view transform from eye, forward direction, and up.
///
/// Transforms right-handed world space points into right-handed Y-up view space.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_affine3(eye: DVec3, dir: DVec3, up: DVec3) -> DAffine3 {
    camera_impl::look_to_affine3::<true>(eye, dir, up)
}

/// Returns a `DMat3` view rotation (no translation) from eye, focal point, and up.
///
/// Transforms right-handed world space points into right-handed Y-up view space.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat3(eye: DVec3, center: DVec3, up: DVec3) -> DMat3 {
    look_to_mat3((center - eye).normalize(), up)
}

/// Returns a `DMat3` view rotation (no translation) from direction and up.
///
/// Transforms right-handed world space points into right-handed Y-up view space.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat3(dir: DVec3, up: DVec3) -> DMat3 {
    camera_impl::look_to_mat3::<true>(dir, up)
}

/// Returns a `DQuat` view rotation from eye, focal point, and up.
///
/// Transforms right-handed world space points into right-handed Y-up view space.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_quat(eye: DVec3, center: DVec3, up: DVec3) -> DQuat {
    look_to_quat((center - eye).normalize(), up)
}

/// Returns a `DQuat` view rotation from direction and up.
///
/// Transforms right-handed world space points into right-handed Y-up view space.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_quat(dir: DVec3, up: DVec3) -> DQuat {
    camera_impl::look_to_quat::<true>(dir, up)
}
