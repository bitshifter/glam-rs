#[macro_use]
mod support;

macro_rules! impl_camera_tests {
    ($t:ident, $affine3:ident, $mat4:ident, $mat3:ident, $vec4:ident, $vec3:ident, $quat:ident, $camera:ident) => {
        struct AxisConfig {
            forward: $vec3,
            right: $vec3,
            up: $vec3,
        }

        struct NdcConfig {
            z_near: $t,
            z_far: $t,
            y_down: bool,
        }

        // Camera eye position used by all pipeline tests in this module.
        const EYE: $vec3 = $vec3::ZERO;

        const NDC_OPENGL: NdcConfig = NdcConfig {
            z_near: -1.0,
            z_far: 1.0,
            y_down: false,
        };
        const NDC_VULKAN: NdcConfig = NdcConfig {
            z_near: 0.0,
            z_far: 1.0,
            y_down: true,
        };
        const NDC_DIRECTX: NdcConfig = NdcConfig {
            z_near: 0.0,
            z_far: 1.0,
            y_down: false,
        };

        /// Right-handed Y-up. Forward is -Z, right is +X.
        /// Standard OpenGL convention used by Maya, Godot, and Bevy.
        const RH_YUP_AXES: AxisConfig = AxisConfig {
            forward: $vec3::NEG_Z,
            right: $vec3::X,
            up: $vec3::Y,
        };

        /// Left-handed Y-up. Forward is +Z, right is +X.
        /// DirectX convention used by Unity 3D.
        const LH_YUP_AXES: AxisConfig = AxisConfig {
            forward: $vec3::Z,
            right: $vec3::X,
            up: $vec3::Y,
        };

        /// Right-handed Z-up. Forward is +Y, right is +X.
        /// Convention used by Blender.
        #[allow(unused)]
        const RH_ZUP_AXES: AxisConfig = AxisConfig {
            forward: $vec3::Y,
            right: $vec3::X,
            up: $vec3::Z,
        };

        /// Left-handed Z-up. Forward is +X, right is +Y
        /// Convention used by Unreal Engine.
        #[allow(unused)]
        const LH_ZUP_AXES: AxisConfig = AxisConfig {
            forward: $vec3::X,
            right: $vec3::Y,
            up: $vec3::Z,
        };

        fn check_view(axes: &AxisConfig, view: &$mat4) {
            // Assumes the camera is at EYE (world origin); see module-level EYE constant.
            // Right maps to +X in view space
            let p = (view * (axes.right * 5.0).to_homogeneous()).project();
            assert_approx_eq!(p.x, 5.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, 0.0, 1e-6);

            // Up maps to +Y in view space
            let p = (view * (axes.up * 5.0).to_homogeneous()).project();
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 5.0, 1e-6);
            assert_approx_eq!(p.z, 0.0, 1e-6);

            // Forward maps to Z in view space, sign given by the handedness of the axis frame.
            let fwd = (view * (axes.forward * 5.0).to_homogeneous()).project();
            let expected_z = handedness_sign(axes) * 5.0;
            assert_approx_eq!(fwd.x, 0.0, 1e-6);
            assert_approx_eq!(fwd.y, 0.0, 1e-6);
            assert_approx_eq!(fwd.z, expected_z, 1e-6);

            // Point behind the camera: opposite Z sign from forward
            let behind = (view * (-axes.forward * 5.0).to_homogeneous()).project();
            assert_approx_eq!(behind.x, 0.0, 1e-6);
            assert_approx_eq!(behind.y, 0.0, 1e-6);
            assert_approx_eq!(behind.z, -expected_z, 1e-6);

            // Camera eye maps to view-space origin
            let p = (view * EYE.to_homogeneous()).project();
            assert_approx_eq!(p, $vec3::ZERO, 1e-6);
        }

        fn check_proj_direction(axes: &AxisConfig, ndc: &NdcConfig, view: &$mat4, proj: &$mat4) {
            // Point directly forward: should map to NDC centre (x=0, y=0)
            let pt = (proj * (view * (axes.forward * 5.0).to_homogeneous())).project();
            assert_approx_eq!(pt.x, 0.0, 1e-6);
            assert_approx_eq!(pt.y, 0.0, 1e-6);

            // Point offset to the right: should map to positive x in NDC
            let pt = (proj * (view * (axes.forward * 5.0 + axes.right).to_homogeneous())).project();
            assert!(pt.x > 0.0);

            // Point offset to the left: should map to negative x in NDC
            let pt = (proj * (view * (axes.forward * 5.0 - axes.right).to_homogeneous())).project();
            assert!(pt.x < 0.0);

            // Point offset upward: y sign depends on whether NDC Y is flipped
            let pt = (proj * (view * (axes.forward * 5.0 + axes.up).to_homogeneous())).project();
            if ndc.y_down {
                assert!(pt.y < 0.0);
            } else {
                assert!(pt.y > 0.0);
            }

            // Point offset downward: opposite y sign from upward
            let pt = (proj * (view * (axes.forward * 5.0 - axes.up).to_homogeneous())).project();
            if ndc.y_down {
                assert!(pt.y > 0.0);
            } else {
                assert!(pt.y < 0.0);
            }
        }

        fn check_proj_near_far(axes: &AxisConfig, ndc: &NdcConfig, view: &$mat4, proj: &$mat4) {
            // Point at the near plane: should map to the near NDC depth
            let pt = (proj * (view * (axes.forward * 1.0).to_homogeneous())).project();
            assert_approx_eq!(pt.x, 0.0, 1e-6);
            assert_approx_eq!(pt.y, 0.0, 1e-6);
            assert_approx_eq!(pt.z, ndc.z_near, 1e-6);

            // Point at the far plane: should map to the far NDC depth
            let pt = (proj * (view * (axes.forward * 10.0).to_homogeneous())).project();
            assert_approx_eq!(pt.x, 0.0, 1e-6);
            assert_approx_eq!(pt.y, 0.0, 1e-6);
            assert_approx_eq!(pt.z, ndc.z_far, 1e-6);
        }

        fn check_proj_near(axes: &AxisConfig, ndc: &NdcConfig, view: &$mat4, proj: &$mat4) {
            // For infinite projections, only the near plane is testable.
            // (Far plane is at infinity, which cannot be represented.)
            let pt = (proj * (view * (axes.forward * 1.0).to_homogeneous())).project();
            assert_approx_eq!(pt.x, 0.0, 1e-6);
            assert_approx_eq!(pt.y, 0.0, 1e-6);
            assert_approx_eq!(pt.z, ndc.z_near, 1e-6);
        }

        fn check_proj_reverse_near(axes: &AxisConfig, ndc: &NdcConfig, view: &$mat4, proj: &$mat4) {
            // Reverse-z: near plane maps to the far NDC depth, infinity maps to near NDC depth.
            let pt = (proj * (view * (axes.forward * 1.0).to_homogeneous())).project();
            assert_approx_eq!(pt.x, 0.0, 1e-6);
            assert_approx_eq!(pt.y, 0.0, 1e-6);
            assert_approx_eq!(pt.z, ndc.z_far, 1e-6);
        }

        fn check_view_rotation_mat3(axes: &AxisConfig, rot: &$mat3) {
            let expected_z = handedness_sign(axes) * 5.0;

            let p = *rot * (axes.right * 5.0);
            assert_approx_eq!(p.x, 5.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, 0.0, 1e-6);

            let p = *rot * (axes.up * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 5.0, 1e-6);
            assert_approx_eq!(p.z, 0.0, 1e-6);

            let p = *rot * (axes.forward * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, expected_z, 1e-6);

            let p = *rot * (-axes.forward * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, -expected_z, 1e-6);
        }

        fn handedness_sign(axes: &AxisConfig) -> $t {
            // Right · (Up × Forward) > 0 → LH (forward → +Z), < 0 → RH (forward → -Z)
            axes.right.dot(axes.up.cross(axes.forward)).signum()
        }

        mod view_rh {
            use super::*;
            use glam::$camera::rh::view;

            mod yup {
                use super::*;
                impl_view_tests!(RH_YUP_AXES, $mat4);
            }

            mod zup {
                use super::*;
                impl_view_tests!(RH_ZUP_AXES, $mat4);
            }
        }

        mod view_lh {
            use super::*;
            use glam::$camera::lh::view;

            mod yup {
                use super::*;
                impl_view_tests!(LH_YUP_AXES, $mat4);
            }

            mod zup {
                use super::*;
                impl_view_tests!(LH_ZUP_AXES, $mat4);
            }
        }

        mod proj_rh {
            use super::*;
            use glam::$camera::rh::proj;

            glam_test!(test_transform_opengl_perspective, {
                let fov = $t::to_radians(90.0);
                let p = proj::opengl::perspective(fov, 2.0, 5.0, 15.0);
                let original = $vec3::new(5.0, 5.0, -15.0);
                let projected = p * original.extend(1.0);
                assert_approx_eq!(projected, $vec4::new(2.5, 5.0, 15.0, 15.0), 1.0e-6);
            });

            glam_test!(test_transform_vulkan_perspective, {
                let fov = $t::to_radians(90.0);
                let p = proj::vulkan::perspective(fov, 2.0, 5.0, 15.0);
                let original = $vec3::new(5.0, 5.0, -15.0);
                let projected = p * original.extend(1.0);
                assert_approx_eq!(projected, $vec4::new(2.5, -5.0, 15.0, 15.0), 1.0e-6);
            });

            glam_test!(test_transform_directx_perspective, {
                let fov = $t::to_radians(90.0);
                let p = proj::directx::perspective(fov, 2.0, 5.0, 15.0);
                let original = $vec3::new(5.0, 5.0, -15.0);
                let projected = p * original.extend(1.0);
                assert_approx_eq!(projected, $vec4::new(2.5, 5.0, 15.0, 15.0), 1.0e-6);
            });
        }

        mod proj_lh {
            use super::*;
            use glam::$camera::lh::proj;

            glam_test!(test_perspective_opengl, {
                let fov = $t::to_radians(90.0);
                let aspect = 2.0;
                let near = 5.0;
                let far = 15.0;

                let p = proj::opengl::perspective(fov, aspect, near, far);
                assert_approx_eq!(p.x_axis, $vec4::new(0.5, 0.0, 0.0, 0.0));
                assert_approx_eq!(p.y_axis, $vec4::new(0.0, 1.0, 0.0, 0.0));
                assert_approx_eq!(p.z_axis, $vec4::new(0.0, 0.0, 2.0, 1.0));
                assert_approx_eq!(p.w_axis, $vec4::new(0.0, 0.0, -15.0, 0.0));
            });

            glam_test!(test_orthographic_opengl, {
                let p = proj::opengl::orthographic(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
                assert_approx_eq!(p.z_axis, $vec4::new(0.0, 0.0, 0.1, 0.0));
                assert_approx_eq!(p.w_axis, $vec4::new(0.0, 0.0, 0.0, 1.0));
            });

            glam_test!(test_frustum_opengl, {
                let fov = $t::to_radians(90.0);
                let aspect = 2.0;
                let near = 5.0;
                let far = 15.0;
                let f = (0.5 * fov).tan();
                let height = near * f;
                let width = height * aspect;

                let p = proj::opengl::frustum(-width, width, -height, height, near, far);
                assert_approx_eq!(p.x_axis, $vec4::new(0.5, 0.0, 0.0, 0.0));
                assert_approx_eq!(p.y_axis, $vec4::new(0.0, 1.0, 0.0, 0.0));
                assert_approx_eq!(p.z_axis, $vec4::new(0.0, 0.0, 2.0, 1.0));
                assert_approx_eq!(p.w_axis, $vec4::new(0.0, 0.0, -15.0, 0.0));
            });

            glam_test!(test_transform_opengl_perspective, {
                let fov = $t::to_radians(90.0);
                let p = proj::opengl::perspective(fov, 2.0, 5.0, 15.0);
                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = p * original.extend(1.0);
                assert_approx_eq!(projected, $vec4::new(2.5, 5.0, 15.0, 15.0), 1.0e-6);
            });

            glam_test!(test_transform_vulkan_perspective, {
                let fov = $t::to_radians(90.0);
                let p = proj::vulkan::perspective(fov, 2.0, 5.0, 15.0);
                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = p * original.extend(1.0);
                assert_approx_eq!(projected, $vec4::new(2.5, -5.0, 15.0, 15.0), 1.0e-6);
            });

            glam_test!(test_transform_directx_perspective, {
                let fov = $t::to_radians(90.0);
                let p = proj::directx::perspective(fov, 2.0, 5.0, 15.0);
                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = p * original.extend(1.0);
                assert_approx_eq!(projected, $vec4::new(2.5, 5.0, 15.0, 15.0), 1.0e-6);
            });
        }

        mod pipeline_rh {
            use super::*;
            use glam::$camera::rh::{proj, view};

            mod yup {
                use super::*;
                impl_pipeline_tests!($t, RH_YUP_AXES);
            }
            mod zup {
                use super::*;
                impl_pipeline_tests!($t, RH_ZUP_AXES);
            }

            mod gltf {
                use super::*;
                // glTF: +Y up, +Z forward, -X right (right-handed)
                const GLTF_AXES: AxisConfig = AxisConfig {
                    forward: $vec3::Z,
                    right: $vec3::NEG_X,
                    up: $vec3::Y,
                };
                impl_pipeline_tests!($t, GLTF_AXES);
            }
        }

        mod pipeline_lh {
            use super::*;
            use glam::$camera::lh::{proj, view};

            mod yup {
                use super::*;
                impl_pipeline_tests!($t, LH_YUP_AXES);
            }
            mod zup {
                use super::*;
                impl_pipeline_tests!($t, LH_ZUP_AXES);
            }
        }

        mod deprecated {
            #![allow(deprecated)]

            use super::*;

            glam_test!(test_quat_look_at, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);

                let point = $vec3::new(1.0, 0.0, 0.0);

                let deprecated_lh = $quat::look_at_lh(eye, center, up);
                let deprecated_rh = $quat::look_at_rh(eye, center, up);
                let lh = $camera::lh::view::look_at_quat(eye, center, up);
                let rh = $camera::rh::view::look_at_quat(eye, center, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh * point, $vec3::new(0.0, 1.0, 0.0));
                assert_approx_eq!(rh * point, $vec3::new(0.0, 1.0, 0.0));

                let dir = (center - eye).normalize();

                let deprecated_lh = $quat::look_to_lh(dir, up);
                let deprecated_rh = $quat::look_to_rh(dir, up);
                let lh = $camera::lh::view::look_to_quat(dir, up);
                let rh = $camera::rh::view::look_to_quat(dir, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh * point, $vec3::new(0.0, 1.0, 0.0));
                assert_approx_eq!(rh * point, $vec3::new(0.0, 1.0, 0.0));

                should_glam_assert!({ $camera::lh::view::look_to_quat($vec3::ONE, $vec3::ZERO) });
                should_glam_assert!({ $camera::lh::view::look_to_quat($vec3::ZERO, $vec3::ONE) });
                should_glam_assert!({ $camera::rh::view::look_to_quat($vec3::ONE, $vec3::ZERO) });
                should_glam_assert!({ $camera::rh::view::look_to_quat($vec3::ZERO, $vec3::ONE) });
            });

            glam_test!(test_mat3_look_at, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);

                let point = $vec3::new(1.0, 0.0, 0.0);

                let deprecated_lh = $mat3::look_at_lh(eye, center, up);
                let deprecated_rh = $mat3::look_at_rh(eye, center, up);
                let lh = $camera::lh::view::look_at_mat3(eye, center, up);
                let rh = $camera::rh::view::look_at_mat3(eye, center, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh * point, $vec3::new(0.0, 1.0, 0.0));
                assert_approx_eq!(rh * point, $vec3::new(0.0, 1.0, 0.0));

                let dir = (center - eye).normalize();
                let deprecated_lh = $mat3::look_to_lh(dir, up);
                let deprecated_rh = $mat3::look_to_rh(dir, up);
                let lh = $camera::lh::view::look_to_mat3(dir, up);
                let rh = $camera::rh::view::look_to_mat3(dir, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh * point, $vec3::new(0.0, 1.0, 0.0));
                assert_approx_eq!(rh * point, $vec3::new(0.0, 1.0, 0.0));

                should_glam_assert!({ $camera::lh::view::look_to_mat3($vec3::ONE, $vec3::ZERO) });
                should_glam_assert!({ $camera::lh::view::look_to_mat3($vec3::ZERO, $vec3::ONE) });
                should_glam_assert!({ $camera::rh::view::look_to_mat3($vec3::ONE, $vec3::ZERO) });
                should_glam_assert!({ $camera::rh::view::look_to_mat3($vec3::ZERO, $vec3::ONE) });
            });

            glam_test!(test_affine3_look_at, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);

                let point = $vec3::new(1.0, 0.0, 0.0);

                let deprecated_lh = $affine3::look_at_lh(eye, center, up);
                let deprecated_rh = $affine3::look_at_rh(eye, center, up);
                let lh = $camera::lh::view::look_at_affine3(eye, center, up);
                let rh = $camera::rh::view::look_at_affine3(eye, center, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
                assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

                let dir = (center - eye).normalize();
                let deprecated_lh = $affine3::look_to_lh(eye, dir, up);
                let deprecated_rh = $affine3::look_to_rh(eye, dir, up);
                let lh = $camera::lh::view::look_to_affine3(eye, dir, up);
                let rh = $camera::rh::view::look_to_affine3(eye, dir, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
                assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

                should_glam_assert!({
                    $camera::lh::view::look_at_affine3($vec3::ONE, $vec3::ZERO, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::rh::view::look_at_affine3($vec3::ONE, $vec3::ZERO, $vec3::ZERO)
                });
            });

            glam_test!(test_mat4_look_at, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);

                let point = $vec3::new(1.0, 0.0, 0.0);

                let deprecated_lh = $mat4::look_at_lh(eye, center, up);
                let deprecated_rh = $mat4::look_at_rh(eye, center, up);
                let lh = $camera::lh::view::look_at_mat4(eye, center, up);
                let rh = $camera::rh::view::look_at_mat4(eye, center, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
                assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

                let dir = (center - eye).normalize();
                let deprecated_lh = $mat4::look_to_lh(eye, dir, up);
                let deprecated_rh = $mat4::look_to_rh(eye, dir, up);
                let lh = $camera::lh::view::look_to_mat4(eye, dir, up);
                let rh = $camera::rh::view::look_to_mat4(eye, dir, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
                assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

                should_glam_assert!({
                    $camera::lh::view::look_to_mat4($vec3::ZERO, $vec3::ONE, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::lh::view::look_to_mat4($vec3::ZERO, $vec3::ZERO, $vec3::ONE)
                });
                should_glam_assert!({
                    $camera::rh::view::look_to_mat4($vec3::ZERO, $vec3::ONE, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::rh::view::look_to_mat4($vec3::ZERO, $vec3::ZERO, $vec3::ONE)
                });
            });

            glam_test!(test_mat4_frustum_gl_rh, {
                let fov_y_radians = $t::to_radians(90.0);
                let aspect_ratio = 2.0;
                let z_near = 5.0;
                let z_far = 15.0;
                let f = (0.5 * fov_y_radians).tan();
                let height = z_near * f;
                let width = height * aspect_ratio;
                let deprecated_frustum =
                    $mat4::frustum_rh_gl(-width, width, -height, height, z_near, z_far);
                let deprecated_perspective =
                    $mat4::perspective_rh_gl(fov_y_radians, aspect_ratio, z_near, z_far);
                let frustum = $camera::rh::proj::opengl::frustum(
                    -width, width, -height, height, z_near, z_far,
                );
                let perspective = $camera::rh::proj::opengl::perspective(
                    fov_y_radians,
                    aspect_ratio,
                    z_near,
                    z_far,
                );
                assert_approx_eq!(deprecated_frustum, frustum);
                assert_approx_eq!(deprecated_perspective, perspective);
                assert_approx_eq!(frustum, perspective);

                should_glam_assert!({
                    $camera::rh::proj::opengl::frustum(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0)
                });
                should_glam_assert!({
                    $camera::rh::proj::opengl::frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_frustum_lh, {
                let fov_y_radians = $t::to_radians(90.0);
                let aspect_ratio = 2.0;
                let z_near = 5.0;
                let z_far = 15.0;
                let f = (0.5 * fov_y_radians).tan();
                let height = z_near * f;
                let width = height * aspect_ratio;
                let deprecated_frustum =
                    $mat4::frustum_lh(-width, width, -height, height, z_near, z_far);
                let deprecated_perspective =
                    $mat4::perspective_lh(fov_y_radians, aspect_ratio, z_near, z_far);
                let frustum = $camera::lh::proj::directx::frustum(
                    -width, width, -height, height, z_near, z_far,
                );
                let perspective = $camera::lh::proj::directx::perspective(
                    fov_y_radians,
                    aspect_ratio,
                    z_near,
                    z_far,
                );
                assert_approx_eq!(deprecated_frustum, frustum);
                assert_approx_eq!(deprecated_perspective, perspective);
                assert_approx_eq!(frustum, perspective);

                should_glam_assert!({
                    $camera::lh::proj::directx::frustum(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0)
                });
                should_glam_assert!({
                    $camera::lh::proj::directx::frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_frustum_rh, {
                let fov_y_radians = $t::to_radians(90.0);
                let aspect_ratio = 2.0;
                let z_near = 5.0;
                let z_far = 15.0;
                let f = (0.5 * fov_y_radians).tan();
                let height = z_near * f;
                let width = height * aspect_ratio;
                let deprecated_frustum =
                    $mat4::frustum_rh(-width, width, -height, height, z_near, z_far);
                let deprecated_perspective =
                    $mat4::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far);
                let frustum = $camera::rh::proj::directx::frustum(
                    -width, width, -height, height, z_near, z_far,
                );
                let perspective = $camera::rh::proj::directx::perspective(
                    fov_y_radians,
                    aspect_ratio,
                    z_near,
                    z_far,
                );
                assert_approx_eq!(deprecated_frustum, frustum);
                assert_approx_eq!(deprecated_perspective, perspective);
                assert_approx_eq!(frustum, perspective);

                should_glam_assert!({
                    $camera::rh::proj::directx::frustum(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0)
                });
                should_glam_assert!({
                    $camera::rh::proj::directx::frustum(-1.0, 1.0, -1.0, 1.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_perspective_gl_rh, {
                let deprecated_projection =
                    $mat4::perspective_rh_gl($t::to_radians(90.0), 2.0, 5.0, 15.0);
                let projection =
                    $camera::rh::proj::opengl::perspective($t::to_radians(90.0), 2.0, 5.0, 15.0);
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, -15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 15.0, 15.0), projected);

                let original = $vec3::new(5.0, 5.0, -5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, -5.0, 5.0), projected);
            });

            glam_test!(test_mat4_perspective_lh, {
                let deprecated_projection =
                    $mat4::perspective_lh($t::to_radians(90.0), 2.0, 5.0, 15.0);
                let projection =
                    $camera::lh::proj::directx::perspective($t::to_radians(90.0), 2.0, 5.0, 15.0);
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 15.0, 15.0), projected, 1e-6);

                let original = $vec3::new(5.0, 5.0, 5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 0.0, 5.0), projected, 1e-6);

                should_glam_assert!({
                    $camera::lh::proj::directx::perspective(0.0, 1.0, 1.0, 0.0)
                });
                should_glam_assert!({
                    $camera::lh::proj::directx::perspective(0.0, 1.0, 0.0, 1.0)
                });
            });

            glam_test!(test_mat4_perspective_infinite_lh, {
                let deprecated_projection =
                    $mat4::perspective_infinite_lh($t::to_radians(90.0), 2.0, 5.0);
                let projection = $camera::lh::proj::directx::perspective_infinite(
                    $t::to_radians(90.0),
                    2.0,
                    5.0,
                );
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 10.0, 15.0), projected, 1e-6);

                let original = $vec3::new(5.0, 5.0, 5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 0.0, 5.0), projected, 1e-6);

                should_glam_assert!({
                    $camera::lh::proj::directx::perspective_infinite(0.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_perspective_infinite_reverse_lh, {
                let deprecated_projection =
                    $mat4::perspective_infinite_reverse_lh($t::to_radians(90.0), 2.0, 5.0);
                let projection = $camera::lh::proj::directx::perspective_infinite_reverse(
                    $t::to_radians(90.0),
                    2.0,
                    5.0,
                );
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, 15.0), projected, 1e-6);

                let original = $vec3::new(5.0, 5.0, 5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, 5.0), projected, 1e-6);

                should_glam_assert!({
                    $camera::lh::proj::directx::perspective_infinite_reverse(0.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_perspective_rh, {
                let deprecated_projection =
                    $mat4::perspective_rh($t::to_radians(90.0), 2.0, 5.0, 15.0);
                let projection =
                    $camera::rh::proj::directx::perspective($t::to_radians(90.0), 2.0, 5.0, 15.0);
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, -30.0, -15.0), projected, 1e-6);

                let original = $vec3::new(5.0, 5.0, 5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, -15.0, -5.0), projected, 1e-6);

                should_glam_assert!({
                    $camera::lh::proj::directx::perspective(0.0, 1.0, 1.0, 0.0)
                });
                should_glam_assert!({
                    $camera::lh::proj::directx::perspective(0.0, 1.0, 0.0, 1.0)
                });
            });

            glam_test!(test_mat4_perspective_infinite_rh, {
                let deprecated_projection =
                    $mat4::perspective_infinite_rh($t::to_radians(90.0), 2.0, 5.0);
                let projection = $camera::rh::proj::directx::perspective_infinite(
                    $t::to_radians(90.0),
                    2.0,
                    5.0,
                );
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, -20.0, -15.0), projected);

                let original = $vec3::new(5.0, 5.0, 5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, -10.0, -5.0), projected);

                should_glam_assert!({
                    $camera::rh::proj::directx::perspective_infinite(0.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_perspective_infinite_reverse_rh, {
                let deprecated_projection =
                    $mat4::perspective_infinite_reverse_rh($t::to_radians(90.0), 2.0, 5.0);
                let projection = $camera::rh::proj::directx::perspective_infinite_reverse(
                    $t::to_radians(90.0),
                    2.0,
                    5.0,
                );
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, -15.0), projected);

                let original = $vec3::new(5.0, 5.0, 5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, -5.0), projected);

                should_glam_assert!({
                    $camera::rh::proj::directx::perspective_infinite_reverse(0.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_orthographic_gl_rh, {
                let deprecated_projection =
                    $mat4::orthographic_rh_gl(-10.0, 10.0, -5.0, 5.0, 1.0, 11.0);
                let projection =
                    $camera::rh::proj::opengl::orthographic(-10.0, 10.0, -5.0, 5.0, 1.0, 11.0);
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
                let projected = projection.mul_vec4(original);
                assert_approx_eq!(projected, $vec4::new(0.5, 1.0, -0.2, 1.0));
            });

            glam_test!(test_mat4_orthographic_rh, {
                let deprecated_projection =
                    $mat4::orthographic_rh(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
                let projection =
                    $camera::rh::proj::directx::orthographic(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
                let projected = projection.mul_vec4(original);
                assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.75, 1.0));

                let original = $vec4::new(5.0, 5.0, 5.0, 1.0);
                let projected = projection.mul_vec4(original);
                assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.25, 1.0));
            });

            glam_test!(test_mat4_orthographic_lh, {
                let deprecated_projection =
                    $mat4::orthographic_lh(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
                let projection =
                    $camera::lh::proj::directx::orthographic(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
                let projected = projection.mul_vec4(original);
                assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.25, 1.0));

                let original = $vec4::new(5.0, 5.0, 5.0, 1.0);
                let projected = projection.mul_vec4(original);
                assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.75, 1.0));
            });
        }
    };
}

macro_rules! impl_view_tests {
    ($axes:ident, $mat4:ident) => {
        glam_test!(test_look_at_mat4, {
            let m = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            check_view(&$axes, &m);
        });

        glam_test!(test_look_to_mat4, {
            let m = view::look_to_mat4(EYE, $axes.forward, $axes.up);
            check_view(&$axes, &m);
        });

        glam_test!(test_look_at_affine3, {
            let a = view::look_at_affine3(EYE, $axes.forward * 5.0, $axes.up);
            let m = $mat4::from(a);
            check_view(&$axes, &m);
        });

        glam_test!(test_look_to_affine3, {
            let a = view::look_to_affine3(EYE, $axes.forward, $axes.up);
            let m = $mat4::from(a);
            check_view(&$axes, &m);
        });

        glam_test!(test_look_at_mat3, {
            let rot = view::look_at_mat3(EYE, $axes.forward * 5.0, $axes.up);
            check_view_rotation_mat3(&$axes, &rot);
        });

        glam_test!(test_look_to_mat3, {
            let rot = view::look_to_mat3($axes.forward, $axes.up);
            check_view_rotation_mat3(&$axes, &rot);
        });

        glam_test!(test_look_at_quat, {
            let q = view::look_at_quat(EYE, $axes.forward * 5.0, $axes.up);
            let expected_z = handedness_sign(&$axes) * 5.0;

            let p = q * ($axes.right * 5.0);
            assert_approx_eq!(p.x, 5.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, 0.0, 1e-6);

            let p = q * ($axes.up * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 5.0, 1e-6);
            assert_approx_eq!(p.z, 0.0, 1e-6);

            let p = q * ($axes.forward * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, expected_z, 1e-6);

            let p = q * (-$axes.forward * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, -expected_z, 1e-6);
        });

        glam_test!(test_look_to_quat, {
            let q = view::look_to_quat($axes.forward, $axes.up);
            let expected_z = handedness_sign(&$axes) * 5.0;

            let p = q * ($axes.right * 5.0);
            assert_approx_eq!(p.x, 5.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, 0.0, 1e-6);

            let p = q * ($axes.up * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 5.0, 1e-6);
            assert_approx_eq!(p.z, 0.0, 1e-6);

            let p = q * ($axes.forward * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, expected_z, 1e-6);

            let p = q * (-$axes.forward * 5.0);
            assert_approx_eq!(p.x, 0.0, 1e-6);
            assert_approx_eq!(p.y, 0.0, 1e-6);
            assert_approx_eq!(p.z, -expected_z, 1e-6);
        });
    };
}

macro_rules! impl_pipeline_tests {
    ($t:ident, $axes:ident) => {
        // ---- perspective ----
        glam_test!(test_opengl_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::opengl::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_OPENGL, &v, &p);
            check_proj_near_far(&$axes, &NDC_OPENGL, &v, &p);
        });

        glam_test!(test_vulkan_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::vulkan::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_VULKAN, &v, &p);
            check_proj_near_far(&$axes, &NDC_VULKAN, &v, &p);
        });

        glam_test!(test_directx_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::directx::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_DIRECTX, &v, &p);
            check_proj_near_far(&$axes, &NDC_DIRECTX, &v, &p);
        });

        // ---- orthographic ----
        glam_test!(test_opengl_orthographic_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::opengl::orthographic(-5.0, 5.0, -5.0, 5.0, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_OPENGL, &v, &p);
            check_proj_near_far(&$axes, &NDC_OPENGL, &v, &p);
        });

        glam_test!(test_vulkan_orthographic_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::vulkan::orthographic(-5.0, 5.0, -5.0, 5.0, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_VULKAN, &v, &p);
            check_proj_near_far(&$axes, &NDC_VULKAN, &v, &p);
        });

        glam_test!(test_directx_orthographic_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::directx::orthographic(-5.0, 5.0, -5.0, 5.0, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_DIRECTX, &v, &p);
            check_proj_near_far(&$axes, &NDC_DIRECTX, &v, &p);
        });

        // ---- frustum ----
        glam_test!(test_opengl_frustum_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let fov = $t::to_radians(90.0);
            let f = (0.5 * fov).tan();
            let height = 1.0 * f;
            let width = height * 1.0;
            let p = proj::opengl::frustum(-width, width, -height, height, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_OPENGL, &v, &p);
            check_proj_near_far(&$axes, &NDC_OPENGL, &v, &p);
        });

        glam_test!(test_vulkan_frustum_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let fov = $t::to_radians(90.0);
            let f = (0.5 * fov).tan();
            let height = 1.0 * f;
            let width = height * 1.0;
            let p = proj::vulkan::frustum(-width, width, -height, height, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_VULKAN, &v, &p);
            check_proj_near_far(&$axes, &NDC_VULKAN, &v, &p);
        });

        glam_test!(test_directx_frustum_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let fov = $t::to_radians(90.0);
            let f = (0.5 * fov).tan();
            let height = 1.0 * f;
            let width = height * 1.0;
            let p = proj::directx::frustum(-width, width, -height, height, 1.0, 10.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_DIRECTX, &v, &p);
            check_proj_near_far(&$axes, &NDC_DIRECTX, &v, &p);
        });

        // ---- perspective_infinite (Vulkan / DirectX only) ----
        glam_test!(test_vulkan_infinite_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::vulkan::perspective_infinite($t::to_radians(90.0), 1.0, 1.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_VULKAN, &v, &p);
            check_proj_near(&$axes, &NDC_VULKAN, &v, &p);
        });

        glam_test!(test_directx_infinite_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::directx::perspective_infinite($t::to_radians(90.0), 1.0, 1.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_DIRECTX, &v, &p);
            check_proj_near(&$axes, &NDC_DIRECTX, &v, &p);
        });

        // ---- perspective_infinite_reverse (Vulkan / DirectX only) ----
        glam_test!(test_vulkan_infinite_reverse_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::vulkan::perspective_infinite_reverse($t::to_radians(90.0), 1.0, 1.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_VULKAN, &v, &p);
            check_proj_reverse_near(&$axes, &NDC_VULKAN, &v, &p);
        });

        glam_test!(test_directx_infinite_reverse_pipeline, {
            let v = view::look_at_mat4(EYE, $axes.forward * 5.0, $axes.up);
            let p = proj::directx::perspective_infinite_reverse($t::to_radians(90.0), 1.0, 1.0);
            check_view(&$axes, &v);
            check_proj_direction(&$axes, &NDC_DIRECTX, &v, &p);
            check_proj_reverse_near(&$axes, &NDC_DIRECTX, &v, &p);
        });
    };
}

mod camera {
    use glam::{camera, Affine3, Mat3, Mat4, Quat, Vec3, Vec4};
    impl_camera_tests!(f32, Affine3, Mat4, Mat3, Vec4, Vec3, Quat, camera);

    mod affine3a {
        use glam::camera::lh::view as lh_view;
        use glam::camera::rh::view as rh_view;
        use glam::Vec3;

        glam_test!(test_rh_look_at_affine3a, {
            let eye = Vec3::new(0.0, 0.0, -5.0);
            let center = Vec3::ZERO;
            let up = Vec3::new(1.0, 0.0, 0.0);
            let point = Vec3::new(1.0, 0.0, 0.0);

            let a = rh_view::look_at_affine3a(eye, center, up);
            assert_approx_eq!(a.transform_point3(point), Vec3::new(0.0, 1.0, -5.0));
        });

        glam_test!(test_rh_look_to_affine3a, {
            let eye = Vec3::new(0.0, 0.0, -5.0);
            let dir = Vec3::new(0.0, 0.0, 1.0);
            let up = Vec3::new(1.0, 0.0, 0.0);
            let point = Vec3::new(1.0, 0.0, 0.0);

            let a = rh_view::look_to_affine3a(eye, dir, up);
            assert_approx_eq!(a.transform_point3(point), Vec3::new(0.0, 1.0, -5.0));
        });

        glam_test!(test_lh_look_at_affine3a, {
            let eye = Vec3::new(0.0, 0.0, -5.0);
            let center = Vec3::ZERO;
            let up = Vec3::new(1.0, 0.0, 0.0);
            let point = Vec3::new(1.0, 0.0, 0.0);

            let a = lh_view::look_at_affine3a(eye, center, up);
            assert_approx_eq!(a.transform_point3(point), Vec3::new(0.0, 1.0, 5.0));
        });

        glam_test!(test_lh_look_to_affine3a, {
            let eye = Vec3::new(0.0, 0.0, -5.0);
            let dir = Vec3::new(0.0, 0.0, 1.0);
            let up = Vec3::new(1.0, 0.0, 0.0);
            let point = Vec3::new(1.0, 0.0, 0.0);

            let a = lh_view::look_to_affine3a(eye, dir, up);
            assert_approx_eq!(a.transform_point3(point), Vec3::new(0.0, 1.0, 5.0));
        });
    }

    mod mat3a {
        use glam::camera::lh::view as lh_view;
        use glam::camera::rh::view as rh_view;
        use glam::{Vec3, Vec3A};

        glam_test!(test_rh_look_at_mat3a, {
            let eye = Vec3::new(0.0, 0.0, -5.0);
            let center = Vec3::ZERO;
            let up = Vec3::new(1.0, 0.0, 0.0);

            let rot = rh_view::look_at_mat3a(eye, center, up);
            assert_approx_eq!(rot * Vec3A::X, Vec3A::new(0.0, 1.0, 0.0));
        });

        glam_test!(test_rh_look_to_mat3a, {
            let dir = Vec3::new(0.0, 0.0, 1.0);
            let up = Vec3::new(1.0, 0.0, 0.0);

            let rot = rh_view::look_to_mat3a(dir, up);
            assert_approx_eq!(rot * Vec3A::X, Vec3A::new(0.0, 1.0, 0.0));
        });

        glam_test!(test_lh_look_at_mat3a, {
            let eye = Vec3::new(0.0, 0.0, -5.0);
            let center = Vec3::ZERO;
            let up = Vec3::new(1.0, 0.0, 0.0);

            let rot = lh_view::look_at_mat3a(eye, center, up);
            assert_approx_eq!(rot * Vec3A::X, Vec3A::new(0.0, 1.0, 0.0));
        });

        glam_test!(test_lh_look_to_mat3a, {
            let dir = Vec3::new(0.0, 0.0, 1.0);
            let up = Vec3::new(1.0, 0.0, 0.0);

            let rot = lh_view::look_to_mat3a(dir, up);
            assert_approx_eq!(rot * Vec3A::X, Vec3A::new(0.0, 1.0, 0.0));
        });
    }
}

#[cfg(feature = "f64")]
mod dcamera {
    use glam::{dcamera, DAffine3, DMat3, DMat4, DQuat, DVec3, DVec4};
    impl_camera_tests!(f64, DAffine3, DMat4, DMat3, DVec4, DVec3, DQuat, dcamera);
}
