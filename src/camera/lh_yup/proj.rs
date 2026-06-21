// Generated from camera_proj.rs.tera template. Edit the template, not the generated file.

pub mod opengl {
    use crate::{camera::camera_impl, Mat4};

    /// Creates a perspective projection matrix with `[-1, 1]` depth range.
    ///
    /// Expects a left-handed view space.
    ///
    /// This is the OpenGL `gluPerspective` equivalent.
    #[inline]
    #[must_use]
    pub fn perspective(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
        camera_impl::perspective::<false, false, false>(vertical_fov, aspect_ratio, z_near, z_far)
    }

    /// Creates an orthographic projection matrix with `[-1, 1]` depth range.
    ///
    /// Expects a left-handed view space.
    ///
    /// This is the OpenGL `glOrtho` equivalent.
    #[inline]
    #[must_use]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        camera_impl::orthographic::<false, false, false>(left, right, bottom, top, near, far)
    }

    /// Creates a perspective projection matrix with `[-1, 1]` depth range.
    ///
    /// Expects a left-handed view space.
    ///
    /// This is the OpenGL `glFrustum` equivalent.
    /// See <https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml>
    #[inline]
    #[must_use]
    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        camera_impl::frustum::<false, false, false>(left, right, bottom, top, near, far)
    }
}

pub mod vulkan {
    use crate::{camera::camera_impl, Mat4};

    /// Creates a perspective projection matrix with `[0, 1]` depth range and Y-flip (Y-down NDC).
    ///
    /// Expects a left-handed view space.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
        camera_impl::perspective::<false, true, true>(vertical_fov, aspect_ratio, z_near, z_far)
    }

    /// Creates an infinite perspective projection matrix with `[0, 1]` depth range and Y-flip (Y-down NDC).
    ///
    /// Expects a left-handed view space.
    /// Like `perspective`, but with an infinite value for `far`. Points near `near`
    /// map to depth `0`, and as they approach infinity depth approaches `1`.
    ///
    /// # Panics
    ///
    /// Will panic if `near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective_infinite(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
        camera_impl::perspective_infinite::<false, true, true>(vertical_fov, aspect_ratio, z_near)
    }

    /// Creates an infinite perspective projection matrix with `[0, 1]` depth range and Y-flip (Y-down NDC),
    /// with reversed depth.
    ///
    /// Expects a left-handed view space.
    /// Maps `near` to depth `1` and infinity to depth `0`.
    ///
    /// # Panics
    ///
    /// Will panic if `near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective_infinite_reverse(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
        camera_impl::perspective_infinite_reverse::<false, true>(vertical_fov, aspect_ratio, z_near)
    }

    /// Creates an orthographic projection matrix with `[0, 1]` depth range and Y-flip (Y-down NDC).
    ///
    /// Expects a left-handed view space.
    #[inline]
    #[must_use]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        camera_impl::orthographic::<false, true, true>(left, right, bottom, top, near, far)
    }

    /// Creates a perspective projection from a frustum with `[0, 1]` depth range and Y-flip (Y-down NDC).
    ///
    /// Expects a left-handed view space.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        camera_impl::frustum::<false, true, true>(left, right, bottom, top, near, far)
    }
}

pub mod directx {
    use crate::{camera::camera_impl, Mat4};

    /// Creates a perspective projection matrix with `[0, 1]` depth range.
    ///
    /// Expects a left-handed view space.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
        camera_impl::perspective::<false, true, false>(vertical_fov, aspect_ratio, z_near, z_far)
    }

    /// Creates an infinite perspective projection matrix with `[0, 1]` depth range.
    ///
    /// Expects a left-handed view space.
    /// Like `perspective`, but with an infinite value for `far`. Points near `near`
    /// map to depth `0`, and as they approach infinity depth approaches `1`.
    ///
    /// # Panics
    ///
    /// Will panic if `near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective_infinite(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
        camera_impl::perspective_infinite::<false, true, false>(vertical_fov, aspect_ratio, z_near)
    }

    /// Creates an infinite perspective projection matrix with `[0, 1]` depth range,
    /// with reversed depth.
    ///
    /// Expects a left-handed view space.
    /// Maps `near` to depth `1` and infinity to depth `0`.
    ///
    /// # Panics
    ///
    /// Will panic if `near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn perspective_infinite_reverse(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
        camera_impl::perspective_infinite_reverse::<false, false>(
            vertical_fov,
            aspect_ratio,
            z_near,
        )
    }

    /// Creates an orthographic projection matrix with `[0, 1]` depth range.
    ///
    /// Expects a left-handed view space.
    #[inline]
    #[must_use]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        camera_impl::orthographic::<false, true, false>(left, right, bottom, top, near, far)
    }

    /// Creates a perspective projection from a frustum with `[0, 1]` depth range.
    ///
    /// Expects a left-handed view space.
    ///
    /// # Panics
    ///
    /// Will panic if `near` or `far` are less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        camera_impl::frustum::<false, true, false>(left, right, bottom, top, near, far)
    }
}
