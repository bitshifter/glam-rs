// Generated from camera_model.rs.tera template. Edit the template, not the generated file.

//! Model transform constructors: orient objects in world space.
//!
//! These functions produce local -> world transforms (object rotation and translation).
//!
//! Unlike view transforms, model transforms take explicit local axis parameters
//! (`fwd_axis` and `up_axis`) because object conventions vary:
//!
//! | Engine / Convention | `fwd_axis`    | `up_axis` |
//! |---------------------|---------------|-----------|
//! | Bevy / OpenGL       | `DVec3::NEG_Z` | `DVec3::Y` |
//! | Unreal Engine       | `DVec3::X`     | `DVec3::Z` |
//! | Unity / glTF        | `DVec3::Z`     | `DVec3::Y` |
//! | Blender             | `DVec3::Y`     | `DVec3::Z` |
//!
//! `fwd_axis` and `up_axis` must be perpendicular.
//!
//! `look_to_*` functions target a forward direction (`dir`), `look_at_*` functions
//! target a focal point (`center`). Functions returning `DMat4`, `DAffine3`,
//! or similar return a full model transform (rotation and translation). Functions returning
//! `DQuat`, `DMat3`, or similar return only the model rotation.
//!
//! # Example
//!
//! ```
//! use glam::DVec3;
//! use glam::dcamera::model;
//!
//! // Bevy: object faces -Z, up is Y
//! let q = model::look_to_quat(DVec3::X, DVec3::Y, DVec3::NEG_Z, DVec3::Y);
//! assert!((q * DVec3::NEG_Z - DVec3::X).length() < 1e-6);
//! ```

use crate::{DAffine3, DMat3, DMat4, DQuat, DVec3};

/// Removes the component of `v` parallel to the unit vector `n` and normalizes the
/// result.
#[inline]
#[must_use]
fn orthogonalize(v: DVec3, n: DVec3) -> DVec3 {
    (v - n * v.dot(n)).normalize()
}

/// Returns the rotation that maps the local frame given by `fwd_axis` and `up_axis`
/// onto the world frame given by `dir` and `up`.
///
/// The third local and world basis vectors are derived from the given pairs, so the
/// handedness of the local frame is encoded in `fwd_axis` and `up_axis`.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
fn look_to_rotation(dir: DVec3, up: DVec3, fwd_axis: DVec3, up_axis: DVec3) -> DMat3 {
    glam_assert!(dir.is_normalized());
    glam_assert!(up.is_normalized());
    glam_assert!(fwd_axis.is_normalized());
    glam_assert!(up_axis.is_normalized());
    glam_assert!(
        fwd_axis.dot(up_axis).abs() <= 2e-4,
        "fwd_axis and up_axis must be perpendicular"
    );

    let u = orthogonalize(up, dir);
    let z_local = fwd_axis.cross(up_axis);
    let z_world = dir.cross(u);

    let local = DMat3::from_cols(fwd_axis, up_axis, z_local);
    let world = DMat3::from_cols(dir, u, z_world);

    world * local.transpose()
}

/// Returns a `DQuat` model rotation from eye, focal point, and up.
///
/// `fwd_axis` and `up_axis` are the object's local forward and up axes. The rotation
/// maps `fwd_axis` onto the direction from `eye` to `center` and `up_axis` onto `up`.
///
/// # Panics
///
/// Will panic if `up`, `fwd_axis` or `up_axis` are not normalized, if `fwd_axis` and
/// `up_axis` are not perpendicular, or if `center` is
/// equal to `eye`, or if the direction from `eye` to `center` is parallel to `up`, when
/// `glam_assert` is enabled.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
pub fn look_at_quat(
    eye: DVec3,
    center: DVec3,
    up: DVec3,
    fwd_axis: DVec3,
    up_axis: DVec3,
) -> DQuat {
    look_to_quat((center - eye).normalize(), up, fwd_axis, up_axis)
}

/// Returns a `DQuat` model rotation from forward direction and up.
///
/// `fwd_axis` and `up_axis` are the object's local forward and up axes. The rotation
/// maps `fwd_axis` onto `dir` and `up_axis` onto `up`.
///
/// # Panics
///
/// Will panic if any argument is not normalized, if `fwd_axis` and `up_axis` are not
/// perpendicular, or if `dir` and `up` are parallel, when `glam_assert` is enabled.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
pub fn look_to_quat(dir: DVec3, up: DVec3, fwd_axis: DVec3, up_axis: DVec3) -> DQuat {
    DQuat::from_mat3(&look_to_rotation(dir, up, fwd_axis, up_axis))
}

/// Returns a `DMat3` model rotation from eye, focal point, and up.
///
/// `fwd_axis` and `up_axis` are the object's local forward and up axes. The rotation
/// maps `fwd_axis` onto the direction from `eye` to `center` and `up_axis` onto `up`.
///
/// # Panics
///
/// Will panic if `up`, `fwd_axis` or `up_axis` are not normalized, if `fwd_axis` and
/// `up_axis` are not perpendicular, or if `center` is
/// equal to `eye`, or if the direction from `eye` to `center` is parallel to `up`, when
/// `glam_assert` is enabled.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
pub fn look_at_mat3(
    eye: DVec3,
    center: DVec3,
    up: DVec3,
    fwd_axis: DVec3,
    up_axis: DVec3,
) -> DMat3 {
    look_to_mat3((center - eye).normalize(), up, fwd_axis, up_axis)
}

/// Returns a `DMat3` model rotation from forward direction and up.
///
/// `fwd_axis` and `up_axis` are the object's local forward and up axes. The rotation
/// maps `fwd_axis` onto `dir` and `up_axis` onto `up`.
///
/// # Panics
///
/// Will panic if any argument is not normalized, if `fwd_axis` and `up_axis` are not
/// perpendicular, or if `dir` and `up` are parallel, when `glam_assert` is enabled.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
pub fn look_to_mat3(dir: DVec3, up: DVec3, fwd_axis: DVec3, up_axis: DVec3) -> DMat3 {
    look_to_rotation(dir, up, fwd_axis, up_axis)
}

/// Returns a `DMat4` model transform from eye, focal point, and up.
///
/// `fwd_axis` and `up_axis` are the object's local forward and up axes. The rotation
/// maps `fwd_axis` onto the direction from `eye` to `center` and `up_axis` onto `up`.
/// The translation of the returned matrix is `eye`.
///
/// # Panics
///
/// Will panic if `up`, `fwd_axis` or `up_axis` are not normalized, if `fwd_axis` and
/// `up_axis` are not perpendicular, or if `center` is
/// equal to `eye`, or if the direction from `eye` to `center` is parallel to `up`, when
/// `glam_assert` is enabled.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
pub fn look_at_mat4(
    eye: DVec3,
    center: DVec3,
    up: DVec3,
    fwd_axis: DVec3,
    up_axis: DVec3,
) -> DMat4 {
    look_to_mat4(eye, (center - eye).normalize(), up, fwd_axis, up_axis)
}

/// Returns a `DMat4` model transform from eye, forward direction, and up.
///
/// `fwd_axis` and `up_axis` are the object's local forward and up axes. The rotation
/// maps `fwd_axis` onto `dir` and `up_axis` onto `up`. The translation of the returned
/// matrix is `eye`.
///
/// # Panics
///
/// Will panic if any argument is not normalized, if `fwd_axis` and `up_axis` are not
/// perpendicular, or if `dir` and `up` are parallel, when `glam_assert` is enabled.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
pub fn look_to_mat4(eye: DVec3, dir: DVec3, up: DVec3, fwd_axis: DVec3, up_axis: DVec3) -> DMat4 {
    let rotation = look_to_rotation(dir, up, fwd_axis, up_axis);
    DMat4::from_cols(
        rotation.x_axis.extend(0.0),
        rotation.y_axis.extend(0.0),
        rotation.z_axis.extend(0.0),
        eye.extend(1.0),
    )
}

/// Returns an `DAffine3` model transform from eye, focal point, and up.
///
/// `fwd_axis` and `up_axis` are the object's local forward and up axes. The rotation
/// maps `fwd_axis` onto the direction from `eye` to `center` and `up_axis` onto `up`.
/// The translation of the returned transform is `eye`.
///
/// # Panics
///
/// Will panic if `up`, `fwd_axis` or `up_axis` are not normalized, if `fwd_axis` and
/// `up_axis` are not perpendicular, or if `center` is
/// equal to `eye`, or if the direction from `eye` to `center` is parallel to `up`, when
/// `glam_assert` is enabled.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
pub fn look_at_affine3(
    eye: DVec3,
    center: DVec3,
    up: DVec3,
    fwd_axis: DVec3,
    up_axis: DVec3,
) -> DAffine3 {
    look_to_affine3(eye, (center - eye).normalize(), up, fwd_axis, up_axis)
}

/// Returns an `DAffine3` model transform from eye, forward direction, and up.
///
/// `fwd_axis` and `up_axis` are the object's local forward and up axes. The rotation
/// maps `fwd_axis` onto `dir` and `up_axis` onto `up`. The translation of the returned
/// transform is `eye`.
///
/// # Panics
///
/// Will panic if any argument is not normalized, if `fwd_axis` and `up_axis` are not
/// perpendicular, or if `dir` and `up` are parallel, when `glam_assert` is enabled.
#[inline]
#[must_use]
#[cfg_attr(any(debug_assertions, feature = "glam-assert"), track_caller)]
pub fn look_to_affine3(
    eye: DVec3,
    dir: DVec3,
    up: DVec3,
    fwd_axis: DVec3,
    up_axis: DVec3,
) -> DAffine3 {
    DAffine3::from_mat3_translation(look_to_rotation(dir, up, fwd_axis, up_axis), eye)
}
