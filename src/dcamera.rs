// Generated from camera_mod.rs.tera template. Edit the template, not the generated file.

//! View and projection transform helpers.
//!
//! This module provides view matrix (`look_at`, `look_to`) and projection
//! matrix (`perspective`, `orthographic`, `frustum`) constructors.
//!
//! ## Choosing a sub-module
//!
//! Pick [`lh`] or [`rh`] based on your **world space** convention:
//!
//! The view functions in each sub-module transform world-space points into
//! **Y-up view space** while preserving your world's handedness. The
//! corresponding projection functions expect view-space input in that same
//! handedness.
//!
//! The `view` sub-module outputs Y-up view space; the `proj` sub-module expects
//! its input in that same view space.
//!
//! Within each `proj` sub-module, pick the API-specific constructor:
//!
//! | Module     | NDC Z   | NDC Y |
//! |------------|---------|-------|
//! | `opengl`   | [-1, 1] | Up    |
//! | `directx`  | [0, 1]  | Up    |
//! | `vulkan`   | [0, 1]  | Down  |

mod camera_impl;

/// View functions produce right-handed view space transforms with
/// +Y = up, +X = right, and -Z = forward.
///
/// Projection functions expect right-handed view-space input.
pub mod rh {
    pub mod proj;
    pub mod view;
}

/// View functions produce left-handed view space transforms with
/// +Y = up, +X = right, and +Z = forward.
///
/// Projection functions expect left-handed view-space input.
pub mod lh {
    pub mod proj;
    pub mod view;
}
