// Generated from camera_view.rs.tera template. Edit the template, not the generated file.

//! View (camera) matrix constructors.
//!
//! Transforms world-space points into Y-up view space while
//! preserving right-handedness.
//!
//! `look_at` targets a focal point; `look_to` targets a direction.
//! Output types include [`Mat4`], [`Affine3`], [`Mat3`],
//! and [`Quat`] as well as [`Affine3A`] and [`Mat3A`].

use crate::{camera::camera_impl, Affine3, Affine3A, Mat3, Mat3A, Mat4, Quat, Vec3};

/// Creates a view transform using a camera position, a focal point, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat4(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_to_mat4(eye, (center - eye).normalize(), up)
}

/// Creates a view transform using a camera position, a facing direction, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat4(eye: Vec3, dir: Vec3, up: Vec3) -> Mat4 {
    camera_impl::look_to_mat4::<true>(eye, dir, up)
}

/// Creates a view transform using a camera position, a focal point, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_affine3(eye: Vec3, center: Vec3, up: Vec3) -> Affine3 {
    look_to_affine3(eye, (center - eye).normalize(), up)
}

/// Creates a view transform using a camera position, a facing direction, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_affine3(eye: Vec3, dir: Vec3, up: Vec3) -> Affine3 {
    camera_impl::look_to_affine3::<true>(eye, dir, up)
}

/// Creates a view transform using a camera position, a focal point, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_affine3a(eye: Vec3, center: Vec3, up: Vec3) -> Affine3A {
    look_to_affine3a(eye, (center - eye).normalize(), up)
}

/// Creates a view transform using a camera position, a facing direction, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_affine3a(eye: Vec3, dir: Vec3, up: Vec3) -> Affine3A {
    camera_impl::look_to_affine3a::<true>(eye, dir, up)
}

/// Creates a view rotation matrix using a camera position, a focal point, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat3(eye: Vec3, center: Vec3, up: Vec3) -> Mat3 {
    look_to_mat3((center - eye).normalize(), up)
}

/// Creates a view rotation matrix using a facing direction and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat3(dir: Vec3, up: Vec3) -> Mat3 {
    camera_impl::look_to_mat3::<true>(dir, up)
}

/// Creates a view rotation matrix using a camera position, a focal point, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat3a(eye: Vec3, center: Vec3, up: Vec3) -> Mat3A {
    look_to_mat3a((center - eye).normalize(), up)
}

/// Creates a view rotation matrix using a facing direction and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat3a(dir: Vec3, up: Vec3) -> Mat3A {
    camera_impl::look_to_mat3a::<true>(dir, up)
}

/// Creates a quaternion rotation from a camera position, a focal point, and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_quat(eye: Vec3, center: Vec3, up: Vec3) -> Quat {
    look_to_quat((center - eye).normalize(), up)
}

/// Creates a quaternion rotation from a facing direction and an up direction.
///
/// For a right-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_quat(dir: Vec3, up: Vec3) -> Quat {
    camera_impl::look_to_quat::<true>(dir, up)
}
