// Generated from camera_mod.rs.tera template. Edit the template, not the generated file.

//! View and projection transform helpers.
//!
//! This module provides view matrix (`look_at`, `look_to`) and projection matrix (`perspective`,
//! `orthographic`, `frustum`) constructors.
//!
//! ## Choosing a sub-module
//!
//! Pick [`lh`] or [`rh`] modules based on your world space coordinate system.
//!
//! The view functions in each sub-module transform world space points into Y-up view space while
//! preserving your world's handedness.
//!
//! Note that while the view space methods will preserve the handedness of your world's coordinate
//! systems, view space is a different coordinate system and will not necessarily be the same as
//! your world coordinate system, especially if your world up is not +Y.
//!
//! There are other possible view space coordinate systems, notably Y-down; however, glam only
//! provides view and projection functions for left- and right-handed view space coordinate systems.
//!
//! The corresponding projection functions expect Y-up view space input with the matching
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

/// View functions produce right-handed view space transforms with
/// +Y = up, +X = right, and -Z = forward.
///
/// Projection functions expect right-handed Y-up view space input.
pub mod rh {
    pub mod proj;
    pub mod view;
}

/// View functions produce left-handed view space transforms with
/// +Y = up, +X = right, and +Z = forward.
///
/// Projection functions expect left-handed Y-up view space input.
pub mod lh {
    pub mod proj;
    pub mod view;
}
