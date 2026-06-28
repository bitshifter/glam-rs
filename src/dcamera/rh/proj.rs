// Generated from camera_proj.rs.tera template. Edit the template, not the generated file.

//! Projection matrix constructors.
//!
//! Expects right-handed Y-up view space input.
//!
//! Each sub-module targets a specific graphics API convention:
//!
//! * [`opengl`] - NDC Z range **[-1, 1]**, Y-up
//! * [`directx`] - NDC Z range **[0, 1]**, Y-up
//! * [`vulkan`] - NDC Z range **[0, 1]**, Y-down

#[doc(alias = "webgl")]
pub mod opengl {
    //! OpenGL NDC convention: Z range **[-1, 1]**, Y-up.
    //!
    //! Expects a right-handed Y-up view space input.

    use crate::{dcamera::camera_impl, DMat4};

    /// Creates a perspective projection matrix for use with OpenGL.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [-1, 1] and Y-up.
    ///
    /// This is the OpenGL `gluPerspective` equivalent.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective(vertical_fov: f64, aspect_ratio: f64, near: f64, far: f64) -> DMat4 {
        camera_impl::perspective::<true, false, false>(vertical_fov, aspect_ratio, near, far)
    }

    /// Creates an orthographic projection matrix for use with OpenGL.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [-1, 1] and Y-up.
    ///
    /// This is the OpenGL `glOrtho` equivalent.
    #[inline]
    #[must_use]
    pub fn orthographic(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> DMat4 {
        camera_impl::orthographic::<true, false, false>(left, right, bottom, top, near, far)
    }

    /// Creates a perspective projection matrix from a frustum for use with OpenGL.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [-1, 1] and Y-up.
    ///
    /// This is the OpenGL `glFrustum` equivalent.
    /// See <https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml>
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn frustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> DMat4 {
        camera_impl::frustum::<true, false, false>(left, right, bottom, top, near, far)
    }
}

pub mod vulkan {
    //! Vulkan NDC convention: Z range **[0, 1]**, Y-down.
    //!
    //! Expects a right-handed Y-up view space input.
    //!
    //! Includes standard, infinite-far, and reverse-depth variants.

    use crate::{dcamera::camera_impl, DMat4};

    /// Creates a perspective projection matrix for use with Vulkan.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-down.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective(vertical_fov: f64, aspect_ratio: f64, near: f64, far: f64) -> DMat4 {
        camera_impl::perspective::<true, true, true>(vertical_fov, aspect_ratio, near, far)
    }

    /// Creates an infinite perspective projection matrix for use with Vulkan.
    ///
    /// Like `perspective`, but with an infinite value for `far`. Points at distance
    /// `near` map to depth `0`; as distance approaches infinity, depth approaches `1`.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-down.
    ///
    /// # Panics
    ///
    /// Will panic if `near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective_infinite(vertical_fov: f64, aspect_ratio: f64, near: f64) -> DMat4 {
        camera_impl::perspective_infinite::<true, true, true>(vertical_fov, aspect_ratio, near)
    }

    /// Creates an infinite perspective projection matrix with reversed depth for use with
    /// Vulkan.
    ///
    /// Maps `near` to depth `1` and infinity to depth `0`.
    ///
    /// Reversed Z improves depth precision when used with a floating-point depth buffer.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-down.
    ///
    /// # Panics
    ///
    /// Will panic if `near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective_infinite_reverse(vertical_fov: f64, aspect_ratio: f64, near: f64) -> DMat4 {
        camera_impl::perspective_infinite_reverse::<true, true>(vertical_fov, aspect_ratio, near)
    }

    /// Creates an orthographic projection matrix for use with Vulkan.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-down.
    #[inline]
    #[must_use]
    pub fn orthographic(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> DMat4 {
        camera_impl::orthographic::<true, true, true>(left, right, bottom, top, near, far)
    }

    /// Creates a perspective projection from a frustum for use with Vulkan.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-down.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn frustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> DMat4 {
        camera_impl::frustum::<true, true, true>(left, right, bottom, top, near, far)
    }
}

#[doc(alias = "webgpu")]
pub mod directx {
    //! DirectX and WebGPU NDC convention: Z range **[0, 1]**, Y-up.
    //!
    //! Expects a right-handed Y-up view space input.
    //!
    //! Includes standard, infinite-far, and reverse-depth variants.

    use crate::{dcamera::camera_impl, DMat4};

    /// Creates a perspective projection matrix for use with DirectX and WebGPU.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-up.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective(vertical_fov: f64, aspect_ratio: f64, near: f64, far: f64) -> DMat4 {
        camera_impl::perspective::<true, true, false>(vertical_fov, aspect_ratio, near, far)
    }

    /// Creates an infinite perspective projection matrix for use with DirectX and WebGPU.
    ///
    /// Like `perspective`, but with an infinite value for `far`. Points at distance
    /// `near` map to depth `0`; as distance approaches infinity, depth approaches `1`.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-up.
    ///
    /// # Panics
    ///
    /// Will panic if `near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective_infinite(vertical_fov: f64, aspect_ratio: f64, near: f64) -> DMat4 {
        camera_impl::perspective_infinite::<true, true, false>(vertical_fov, aspect_ratio, near)
    }

    /// Creates an infinite perspective projection matrix with reversed depth for use with
    /// DirectX and WebGPU.
    ///
    /// Maps `near` to depth `1` and infinity to depth `0`.
    ///
    /// Reversed Z improves depth precision when used with a floating-point depth buffer.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-up.
    ///
    /// # Panics
    ///
    /// Will panic if `near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective_infinite_reverse(vertical_fov: f64, aspect_ratio: f64, near: f64) -> DMat4 {
        camera_impl::perspective_infinite_reverse::<true, false>(vertical_fov, aspect_ratio, near)
    }

    /// Creates an orthographic projection matrix for use with DirectX and WebGPU.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-up.
    #[inline]
    #[must_use]
    pub fn orthographic(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> DMat4 {
        camera_impl::orthographic::<true, true, false>(left, right, bottom, top, near, far)
    }

    /// Creates a perspective projection from a frustum for use with DirectX and WebGPU.
    ///
    /// Expects a right-handed Y-up view space input.
    /// Outputs NDC with Z in [0, 1] and Y-up.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn frustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> DMat4 {
        camera_impl::frustum::<true, true, false>(left, right, bottom, top, near, far)
    }
}
