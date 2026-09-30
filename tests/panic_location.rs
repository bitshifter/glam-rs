//! Verifies that `glam_assert!` panics are reported at the caller's location.
//!
//! `#[track_caller]` only propagates through a fully annotated call chain, so
//! these checks also cover delegating public APIs and private helpers.

#![cfg(all(
    any(
        feature = "glam-assert",
        all(debug_assertions, feature = "debug-glam-assert")
    ),
    feature = "std",
    panic = "unwind"
))]

use glam::{Affine3, EulerRot, Mat3, Quat, Vec3, Vec4};
use std::panic;
use std::sync::Mutex;

static LOCATION: Mutex<Option<String>> = Mutex::new(None);

/// Asserts that `f` panics and that the panic is reported in this file.
///
/// Uses a global panic hook, so keep this in a test binary with a single test.
fn assert_panics_at_caller<R>(name: &str, f: impl FnOnce() -> R + panic::UnwindSafe) {
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|info| {
        if let Some(location) = info.location() {
            *LOCATION.lock().unwrap() = Some(location.file().to_string());
        }
    }));
    let result = panic::catch_unwind(f);
    panic::set_hook(previous_hook);

    assert!(result.is_err(), "{name} did not panic");
    let location = LOCATION.lock().unwrap().take().unwrap();
    assert!(
        location.ends_with("tests/panic_location.rs"),
        "{name} panicked at {location}"
    );
}

#[test]
#[allow(deprecated, clippy::op_ref)]
fn test_panics_report_caller_location() {
    // Direct `glam_assert!` in a public API.
    assert_panics_at_caller("Vec3::normalize", || Vec3::ZERO.normalize());

    // Public API delegating to another public API.
    assert_panics_at_caller("Vec3::reject_from", || Vec3::X.reject_from(Vec3::ZERO));

    // Public API delegating to a private helper (`inverse_checked`).
    assert_panics_at_caller("Mat3::inverse", || {
        Mat3::from_cols(Vec3::splat(1e30), Vec3::splat(1e30), Vec3::splat(1e30)).inverse()
    });

    // Public API delegating into another type (`Quat::from_axis_angle`).
    assert_panics_at_caller("Vec3::rotate_axis", || Vec3::X.rotate_axis(Vec3::ZERO, 0.0));

    // Trait implementations, by value and by reference.
    assert_panics_at_caller("Quat * Vec3", || {
        Quat::from_xyzw(0.0, 0.0, 0.0, 0.0) * Vec3::X
    });
    assert_panics_at_caller("&Quat * &Vec3", || {
        &Quat::from_xyzw(0.0, 0.0, 0.0, 0.0) * &Vec3::X
    });

    // Deprecated wrapper delegating to another deprecated method.
    assert_panics_at_caller("Mat3::look_at_rh", || {
        Mat3::look_at_rh(Vec3::X, Vec3::ZERO, Vec3::splat(1.0))
    });

    // Delegation through the private `ToEuler` trait (`Quat` -> `Mat3::from_quat`).
    assert_panics_at_caller("Quat::to_euler", || {
        Quat::from_xyzw(0.0, 0.0, 0.0, 2.0).to_euler(EulerRot::XYZ)
    });

    // Private helper (`lerp_impl`) normalizing a zero interpolation result.
    assert_panics_at_caller("Quat::slerp", || Quat::IDENTITY.slerp(Quat::IDENTITY, 1e38));

    // Affine type delegating into the matrix type.
    assert_panics_at_caller("Affine3::from_quat", || {
        Affine3::from_quat(Quat::from_xyzw(0.0, 0.0, 0.0, 2.0))
    });

    // Homogeneous projection (`Vec4::project` -> `Vec3::from_homogeneous`).
    assert_panics_at_caller("Vec4::project", || Vec4::new(1.0, 2.0, 3.0, 0.0).project());

    // Plain `assert!` and indexing panics (always active, not feature gated).
    assert_panics_at_caller("Vec3::from_slice", || Vec3::from_slice(&[1.0, 2.0]));
    assert_panics_at_caller("Mat3::col", || Mat3::IDENTITY.col(3));
    assert_panics_at_caller("Mat3::from_cols_slice", || Mat3::from_cols_slice(&[0.0; 8]));
    assert_panics_at_caller("BVec3::test", || glam::BVec3::new(true, true, true).test(3));
}
