// Generated from camera_mod.rs.tera template. Edit the template, not the generated file.

//! View and projection transform helpers.
//!
//! This module provides view matrix (`look_at`, `look_to`) and projection
//! matrix (`perspective`, `orthographic`, `frustum`) constructors.
//!
//! ## Choosing a sub-module
//!
//! Pick [`lh_yup`] or [`rh_yup`] based on your **world space** convention:
//!
//! | Module | World handedness | Up vector |
//! |--------|-----------------|----------|
//! | [`lh_yup`] | Left | Y |
//! | [`rh_yup`] | Right | Y |
//!
//! The view functions in each sub-module transform world-space points into
//! **Y-up view space** while preserving your world's handedness. The
//! corresponding projection functions expect view-space input in that same
//! handedness.
//!
//! Within each `proj` sub-module, pick the API-specific constructor:
//!
//! | Module     | NDC Z   | NDC Y |
//! |------------|---------|-------|
//! | `opengl`   | [-1, 1] | Up    |
//! | `directx`  | [0, 1]  | Up    |
//! | `vulkan`   | [0, 1]  | Down  |

mod camera_impl;

/// Right-handed, Y-up.
///
/// View functions produce Y-up view space while preserving right-handedness.
/// Projection functions in [`rh_yup::proj`] expect right-handed view-space input.
pub mod rh_yup {
    pub mod proj;
    pub mod view;
}

/// Left-handed, Y-up.
///
/// View functions produce Y-up view space while preserving left-handedness.
/// Projection functions in [`lh_yup::proj`] expect left-handed view-space input.
pub mod lh_yup {
    pub mod proj;
    pub mod view;
}
