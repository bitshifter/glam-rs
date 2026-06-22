// Generated from camera_view.rs.tera template. Edit the template, not the generated file.

//! View (camera) matrix constructors.
//!
//! Transforms world-space points into Y-up view space while
//! preserving left-handedness.
//!
//! `look_at` targets a focal point; `look_to` targets a direction.
//! Output types include [`DMat4`], [`DAffine3`], [`DMat3`],
//! and [`DQuat`].

use crate::{dcamera::camera_impl, DAffine3, DMat3, DMat4, DQuat, DVec3};

/// Creates a view transform using a camera position, a focal point, and an up direction.
///
/// For a left-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat4(eye: DVec3, center: DVec3, up: DVec3) -> DMat4 {
    look_to_mat4(eye, (center - eye).normalize(), up)
}

/// Creates a view transform using a camera position, a facing direction, and an up direction.
///
/// For a left-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat4(eye: DVec3, dir: DVec3, up: DVec3) -> DMat4 {
    camera_impl::look_to_mat4::<false>(eye, dir, up)
}

/// Creates a view transform using a camera position, a focal point, and an up direction.
///
/// For a left-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_affine3(eye: DVec3, center: DVec3, up: DVec3) -> DAffine3 {
    look_to_affine3(eye, (center - eye).normalize(), up)
}

/// Creates a view transform using a camera position, a facing direction, and an up direction.
///
/// For a left-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_affine3(eye: DVec3, dir: DVec3, up: DVec3) -> DAffine3 {
    camera_impl::look_to_affine3::<false>(eye, dir, up)
}

/// Creates a view rotation matrix using a camera position, a focal point, and an up direction.
///
/// For a left-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_mat3(eye: DVec3, center: DVec3, up: DVec3) -> DMat3 {
    look_to_mat3((center - eye).normalize(), up)
}

/// Creates a view rotation matrix using a facing direction and an up direction.
///
/// For a left-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_mat3(dir: DVec3, up: DVec3) -> DMat3 {
    camera_impl::look_to_mat3::<false>(dir, up)
}

/// Creates a quaternion rotation from a camera position, a focal point, and an up direction.
///
/// For a left-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `up` is not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_at_quat(eye: DVec3, center: DVec3, up: DVec3) -> DQuat {
    look_to_quat((center - eye).normalize(), up)
}

/// Creates a quaternion rotation from a facing direction and an up direction.
///
/// For a left-handed, Y-up view coordinate system.
///
/// # Panics
///
/// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
#[inline]
#[must_use]
pub fn look_to_quat(dir: DVec3, up: DVec3) -> DQuat {
    camera_impl::look_to_quat::<false>(dir, up)
}
