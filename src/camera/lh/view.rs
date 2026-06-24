// Generated from camera_view.rs.tera template. Edit the template, not the generated file.

//! View (camera) matrix constructors for left-handed world coordinates.
//!
//! Every function transforms world-space points into Y-up view space while
//! preserving handedness.
//!
//! * `look_at_*` targets a focal point (`center`)
//! * `look_to_*` targets a forward direction (`dir`)

use crate::{camera::camera_impl, Affine3, Affine3A, Mat3, Mat3A, Mat4, Quat, Vec3};

/// Returns a `Mat4` view matrix from eye, focal point, and up.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat4(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_to_mat4(eye, (center - eye).normalize(), up)
}

/// Returns a `Mat4` view matrix from eye, forward direction, and up.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat4(eye: Vec3, dir: Vec3, up: Vec3) -> Mat4 {
    camera_impl::look_to_mat4::<false>(eye, dir, up)
}

/// Returns an `Affine3` view transform from eye, focal point, and up.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_affine3(eye: Vec3, center: Vec3, up: Vec3) -> Affine3 {
    look_to_affine3(eye, (center - eye).normalize(), up)
}

/// Returns an `Affine3` view transform from eye, forward direction, and up.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_affine3(eye: Vec3, dir: Vec3, up: Vec3) -> Affine3 {
    camera_impl::look_to_affine3::<false>(eye, dir, up)
}

/// Returns an `Affine3A` view transform from eye, focal point, and up.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_affine3a(eye: Vec3, center: Vec3, up: Vec3) -> Affine3A {
    look_to_affine3a(eye, (center - eye).normalize(), up)
}

/// Returns an `Affine3A` view transform from eye, forward direction, and up.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_affine3a(eye: Vec3, dir: Vec3, up: Vec3) -> Affine3A {
    camera_impl::look_to_affine3a::<false>(eye, dir, up)
}

/// Returns a `Mat3` view rotation (no translation) from eye, focal point, and up.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat3(eye: Vec3, center: Vec3, up: Vec3) -> Mat3 {
    look_to_mat3((center - eye).normalize(), up)
}

/// Returns a `Mat3` view rotation (no translation) from direction and up.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat3(dir: Vec3, up: Vec3) -> Mat3 {
    camera_impl::look_to_mat3::<false>(dir, up)
}

/// Returns a `Mat3A` view rotation (no translation) from eye, focal point, and up.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat3a(eye: Vec3, center: Vec3, up: Vec3) -> Mat3A {
    look_to_mat3a((center - eye).normalize(), up)
}

/// Returns a `Mat3A` view rotation (no translation) from direction and up.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat3a(dir: Vec3, up: Vec3) -> Mat3A {
    camera_impl::look_to_mat3a::<false>(dir, up)
}

/// Returns a `Quat` view rotation from eye, focal point, and up.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_quat(eye: Vec3, center: Vec3, up: Vec3) -> Quat {
    look_to_quat((center - eye).normalize(), up)
}

/// Returns a `Quat` view rotation from direction and up.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_quat(dir: Vec3, up: Vec3) -> Quat {
    camera_impl::look_to_quat::<false>(dir, up)
}
