#[macro_use]
mod support;

macro_rules! impl_camera_tests {
    ($t:ident, $affine3:ident, $mat4:ident, $mat3:ident, $vec4:ident, $vec3:ident, $quat:ident, $camera:ident) => {
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
                let lh = $camera::lh_yup::view::look_at_quat(eye, center, up);
                let rh = $camera::rh_yup::view::look_at_quat(eye, center, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh * point, $vec3::new(0.0, 1.0, 0.0));
                assert_approx_eq!(rh * point, $vec3::new(0.0, 1.0, 0.0));

                let dir = (center - eye).normalize();

                let deprecated_lh = $quat::look_to_lh(dir, up);
                let deprecated_rh = $quat::look_to_rh(dir, up);
                let lh = $camera::lh_yup::view::look_to_quat(dir, up);
                let rh = $camera::rh_yup::view::look_to_quat(dir, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh * point, $vec3::new(0.0, 1.0, 0.0));
                assert_approx_eq!(rh * point, $vec3::new(0.0, 1.0, 0.0));

                should_glam_assert!({
                    $camera::lh_yup::view::look_to_quat($vec3::ONE, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::lh_yup::view::look_to_quat($vec3::ZERO, $vec3::ONE)
                });
                should_glam_assert!({
                    $camera::rh_yup::view::look_to_quat($vec3::ONE, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::rh_yup::view::look_to_quat($vec3::ZERO, $vec3::ONE)
                });
            });

            glam_test!(test_mat3_look_at, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);

                let point = $vec3::new(1.0, 0.0, 0.0);

                let deprecated_lh = $mat3::look_at_lh(eye, center, up);
                let deprecated_rh = $mat3::look_at_rh(eye, center, up);
                let lh = $camera::lh_yup::view::look_at_mat3(eye, center, up);
                let rh = $camera::rh_yup::view::look_at_mat3(eye, center, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh * point, $vec3::new(0.0, 1.0, 0.0));
                assert_approx_eq!(rh * point, $vec3::new(0.0, 1.0, 0.0));

                let dir = (center - eye).normalize();
                let deprecated_lh = $mat3::look_to_lh(dir, up);
                let deprecated_rh = $mat3::look_to_rh(dir, up);
                let lh = $camera::lh_yup::view::look_to_mat3(dir, up);
                let rh = $camera::rh_yup::view::look_to_mat3(dir, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh * point, $vec3::new(0.0, 1.0, 0.0));
                assert_approx_eq!(rh * point, $vec3::new(0.0, 1.0, 0.0));

                should_glam_assert!({
                    $camera::lh_yup::view::look_to_mat3($vec3::ONE, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::lh_yup::view::look_to_mat3($vec3::ZERO, $vec3::ONE)
                });
                should_glam_assert!({
                    $camera::rh_yup::view::look_to_mat3($vec3::ONE, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::rh_yup::view::look_to_mat3($vec3::ZERO, $vec3::ONE)
                });
            });

            glam_test!(test_affine3_look_at, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);

                let point = $vec3::new(1.0, 0.0, 0.0);

                let deprecated_lh = $affine3::look_at_lh(eye, center, up);
                let deprecated_rh = $affine3::look_at_rh(eye, center, up);
                let lh = $camera::lh_yup::view::look_at_affine3(eye, center, up);
                let rh = $camera::rh_yup::view::look_at_affine3(eye, center, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
                assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

                let dir = (center - eye).normalize();
                let deprecated_lh = $affine3::look_to_lh(eye, dir, up);
                let deprecated_rh = $affine3::look_to_rh(eye, dir, up);
                let lh = $camera::lh_yup::view::look_to_affine3(eye, dir, up);
                let rh = $camera::rh_yup::view::look_to_affine3(eye, dir, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
                assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

                should_glam_assert!({
                    $camera::lh_yup::view::look_at_affine3($vec3::ONE, $vec3::ZERO, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::rh_yup::view::look_at_affine3($vec3::ONE, $vec3::ZERO, $vec3::ZERO)
                });
            });

            glam_test!(test_mat4_look_at, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);

                let point = $vec3::new(1.0, 0.0, 0.0);

                let deprecated_lh = $mat4::look_at_lh(eye, center, up);
                let deprecated_rh = $mat4::look_at_rh(eye, center, up);
                let lh = $camera::lh_yup::view::look_at_mat4(eye, center, up);
                let rh = $camera::rh_yup::view::look_at_mat4(eye, center, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
                assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

                let dir = (center - eye).normalize();
                let deprecated_lh = $mat4::look_to_lh(eye, dir, up);
                let deprecated_rh = $mat4::look_to_rh(eye, dir, up);
                let lh = $camera::lh_yup::view::look_to_mat4(eye, dir, up);
                let rh = $camera::rh_yup::view::look_to_mat4(eye, dir, up);
                assert_approx_eq!(deprecated_lh, lh);
                assert_approx_eq!(deprecated_rh, rh);

                assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
                assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

                should_glam_assert!({
                    $camera::lh_yup::view::look_to_mat4($vec3::ZERO, $vec3::ONE, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::lh_yup::view::look_to_mat4($vec3::ZERO, $vec3::ZERO, $vec3::ONE)
                });
                should_glam_assert!({
                    $camera::rh_yup::view::look_to_mat4($vec3::ZERO, $vec3::ONE, $vec3::ZERO)
                });
                should_glam_assert!({
                    $camera::rh_yup::view::look_to_mat4($vec3::ZERO, $vec3::ZERO, $vec3::ONE)
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
                let frustum = $camera::rh_yup::proj::opengl::frustum(
                    -width, width, -height, height, z_near, z_far,
                );
                let perspective = $camera::rh_yup::proj::opengl::perspective(
                    fov_y_radians,
                    aspect_ratio,
                    z_near,
                    z_far,
                );
                assert_approx_eq!(deprecated_frustum, frustum);
                assert_approx_eq!(deprecated_perspective, perspective);
                assert_approx_eq!(frustum, perspective);
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
                let frustum = $camera::lh_yup::proj::directx::frustum(
                    -width, width, -height, height, z_near, z_far,
                );
                let perspective = $camera::lh_yup::proj::directx::perspective(
                    fov_y_radians,
                    aspect_ratio,
                    z_near,
                    z_far,
                );
                assert_approx_eq!(deprecated_frustum, frustum);
                assert_approx_eq!(deprecated_perspective, perspective);
                assert_approx_eq!(frustum, perspective);
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
                let frustum = $camera::rh_yup::proj::directx::frustum(
                    -width, width, -height, height, z_near, z_far,
                );
                let perspective = $camera::rh_yup::proj::directx::perspective(
                    fov_y_radians,
                    aspect_ratio,
                    z_near,
                    z_far,
                );
                assert_approx_eq!(deprecated_frustum, frustum);
                assert_approx_eq!(deprecated_perspective, perspective);
                assert_approx_eq!(frustum, perspective);
            });

            glam_test!(test_mat4_perspective_gl_rh, {
                let deprecated_projection =
                    $mat4::perspective_rh_gl($t::to_radians(90.0), 2.0, 5.0, 15.0);
                let projection = $camera::rh_yup::proj::opengl::perspective(
                    $t::to_radians(90.0),
                    2.0,
                    5.0,
                    15.0,
                );
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
                let projection = $camera::lh_yup::proj::directx::perspective(
                    $t::to_radians(90.0),
                    2.0,
                    5.0,
                    15.0,
                );
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 15.0, 15.0), projected, 1e-6);

                let original = $vec3::new(5.0, 5.0, 5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, 0.0, 5.0), projected, 1e-6);

                should_glam_assert!({
                    $camera::lh_yup::proj::directx::perspective(0.0, 1.0, 1.0, 0.0)
                });
                should_glam_assert!({
                    $camera::lh_yup::proj::directx::perspective(0.0, 1.0, 0.0, 1.0)
                });
            });

            glam_test!(test_mat4_perspective_infinite_lh, {
                let deprecated_projection =
                    $mat4::perspective_infinite_lh($t::to_radians(90.0), 2.0, 5.0);
                let projection = $camera::lh_yup::proj::directx::perspective_infinite(
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
                    $camera::lh_yup::proj::directx::perspective_infinite(0.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_perspective_infinite_reverse_lh, {
                let deprecated_projection =
                    $mat4::perspective_infinite_reverse_lh($t::to_radians(90.0), 2.0, 5.0);
                let projection = $camera::lh_yup::proj::directx::perspective_infinite_reverse(
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
                    $camera::lh_yup::proj::directx::perspective_infinite_reverse(0.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_perspective_rh, {
                let deprecated_projection =
                    $mat4::perspective_rh($t::to_radians(90.0), 2.0, 5.0, 15.0);
                let projection = $camera::rh_yup::proj::directx::perspective(
                    $t::to_radians(90.0),
                    2.0,
                    5.0,
                    15.0,
                );
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec3::new(5.0, 5.0, 15.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, -30.0, -15.0), projected, 1e-6);

                let original = $vec3::new(5.0, 5.0, 5.0);
                let projected = projection * original.extend(1.0);
                assert_approx_eq!($vec4::new(2.5, 5.0, -15.0, -5.0), projected, 1e-6);

                should_glam_assert!({
                    $camera::lh_yup::proj::directx::perspective(0.0, 1.0, 1.0, 0.0)
                });
                should_glam_assert!({
                    $camera::lh_yup::proj::directx::perspective(0.0, 1.0, 0.0, 1.0)
                });
            });

            glam_test!(test_mat4_perspective_infinite_rh, {
                let deprecated_projection =
                    $mat4::perspective_infinite_rh($t::to_radians(90.0), 2.0, 5.0);
                let projection = $camera::rh_yup::proj::directx::perspective_infinite(
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
                    $camera::rh_yup::proj::directx::perspective_infinite(0.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_perspective_infinite_reverse_rh, {
                let deprecated_projection =
                    $mat4::perspective_infinite_reverse_rh($t::to_radians(90.0), 2.0, 5.0);
                let projection = $camera::rh_yup::proj::directx::perspective_infinite_reverse(
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
                    $camera::rh_yup::proj::directx::perspective_infinite_reverse(0.0, 1.0, 0.0)
                });
            });

            glam_test!(test_mat4_orthographic_gl_rh, {
                let deprecated_projection =
                    $mat4::orthographic_rh_gl(-10.0, 10.0, -5.0, 5.0, 1.0, 11.0);
                let projection =
                    $camera::rh_yup::proj::opengl::orthographic(-10.0, 10.0, -5.0, 5.0, 1.0, 11.0);
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
                let projected = projection.mul_vec4(original);
                assert_approx_eq!(projected, $vec4::new(0.5, 1.0, -0.2, 1.0));
            });

            glam_test!(test_mat4_orthographic_rh, {
                let deprecated_projection =
                    $mat4::orthographic_rh(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
                let projection = $camera::rh_yup::proj::directx::orthographic(
                    -10.0, 10.0, -5.0, 5.0, -10.0, 10.0,
                );
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
                let projection = $camera::lh_yup::proj::directx::orthographic(
                    -10.0, 10.0, -5.0, 5.0, -10.0, 10.0,
                );
                assert_approx_eq!(deprecated_projection, projection);

                let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
                let projected = projection.mul_vec4(original);
                assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.25, 1.0));

                let original = $vec4::new(5.0, 5.0, 5.0, 1.0);
                let projected = projection.mul_vec4(original);
                assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.75, 1.0));
            });
        }

        mod view_rh_yup {
            use super::*;
            use glam::$camera::rh_yup::view;

            glam_test!(test_look_at_mat4, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);
                let point = $vec3::new(1.0, 0.0, 0.0);

                let m = view::look_at_mat4(eye, center, up);
                assert_approx_eq!(m.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));
            });
        }

        mod view_lh_yup {
            use super::*;
            use glam::$camera::lh_yup::view;

            glam_test!(test_look_at_mat4, {
                let eye = $vec3::new(0.0, 0.0, -5.0);
                let center = $vec3::new(0.0, 0.0, 0.0);
                let up = $vec3::new(1.0, 0.0, 0.0);
                let point = $vec3::new(1.0, 0.0, 0.0);

                let m = view::look_at_mat4(eye, center, up);
                assert_approx_eq!(m.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
            });
        }

        mod proj_rh_yup {
            use super::*;
            use glam::$camera::rh_yup::proj;

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

        mod proj_lh_yup {
            use super::*;
            use glam::$camera::lh_yup::proj;

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

        fn check_view_proj_pipeline(
            forward: $vec3,
            right: $vec3,
            up: $vec3,
            view: $mat4,
            proj: $mat4,
            ndc_z_near: $t,
            ndc_z_far: $t,
            flip_y: bool,
        ) {
            // Point directly forward: should map to NDC centre (x=0, y=0)
            let ndc = (proj * (view * (forward * 5.0).to_homogeneous())).project();
            assert_approx_eq!(ndc.x, 0.0, 1e-6);
            assert_approx_eq!(ndc.y, 0.0, 1e-6);
            assert!(ndc.z > ndc_z_near && ndc.z < ndc_z_far);

            // Point offset to the right: should map to positive x in NDC
            let ndc = (proj * (view * (forward * 5.0 + right).to_homogeneous())).project();
            assert!(ndc.x > 0.0);
            assert_approx_eq!(ndc.y, 0.0, 1e-6);

            // Point offset to the left: should map to negative x in NDC
            let ndc = (proj * (view * (forward * 5.0 - right).to_homogeneous())).project();
            assert!(ndc.x < 0.0);
            assert_approx_eq!(ndc.y, 0.0, 1e-6);

            // Point offset upward: y sign depends on whether NDC Y is flipped
            let ndc = (proj * (view * (forward * 5.0 + up).to_homogeneous())).project();
            if flip_y {
                assert!(ndc.y < 0.0);
            } else {
                assert!(ndc.y > 0.0);
            }

            // Point offset downward: opposite y sign from upward
            let ndc = (proj * (view * (forward * 5.0 - up).to_homogeneous())).project();
            if flip_y {
                assert!(ndc.y > 0.0);
            } else {
                assert!(ndc.y < 0.0);
            }

            // Point at the near plane: should map to the near NDC depth
            let ndc = (proj * (view * (forward * 1.0).to_homogeneous())).project();
            assert_approx_eq!(ndc.x, 0.0, 1e-6);
            assert_approx_eq!(ndc.y, 0.0, 1e-6);
            assert_approx_eq!(ndc.z, ndc_z_near, 1e-6);

            // Point at the far plane: should map to the far NDC depth
            let ndc = (proj * (view * (forward * 10.0).to_homogeneous())).project();
            assert_approx_eq!(ndc.x, 0.0, 1e-6);
            assert_approx_eq!(ndc.y, 0.0, 1e-6);
            assert_approx_eq!(ndc.z, ndc_z_far, 1e-6);
        }

        /// Right-handed Y-up coordinate system.
        ///
        /// Forward is -Z (view space looks down -Z), up is +Y. This is the
        /// standard OpenGL convention used by Maya, Godot, and Bevy.
        mod pipeline_rh_yup {
            use super::*;
            use glam::$camera::rh_yup::{proj, view};

            const FWD: $vec3 = $vec3::NEG_Z;
            const RIGHT: $vec3 = $vec3::X;
            const UP: $vec3 = $vec3::Y;

            glam_test!(test_opengl_perspective, {
                let v = view::look_at_mat4($vec3::ZERO, FWD * 5.0, UP);
                let p = proj::opengl::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, -1.0, 1.0, false);
            });

            glam_test!(test_vulkan_perspective, {
                let v = view::look_at_mat4($vec3::ZERO, FWD * 5.0, UP);
                let p = proj::vulkan::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, 0.0, 1.0, true);
            });

            glam_test!(test_directx_perspective, {
                let v = view::look_at_mat4($vec3::ZERO, FWD * 5.0, UP);
                let p = proj::directx::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, 0.0, 1.0, false);
            });

            glam_test!(test_opengl_affine3, {
                let a = view::look_at_affine3($vec3::ZERO, FWD * 5.0, UP);
                let v = $mat4::from(a);
                let p = proj::opengl::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, -1.0, 1.0, false);
            });

            glam_test!(test_opengl_look_to, {
                let v = view::look_to_mat4($vec3::ZERO, FWD, UP);
                let p = proj::opengl::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, -1.0, 1.0, false);
            });
        }

        /// Left-handed Y-up coordinate system.
        ///
        /// Forward is +Z (view space looks down +Z), up is +Y. This is the
        /// DirectX convention used by Unity 3D.
        mod pipeline_lh_yup {
            use super::*;
            use glam::$camera::lh_yup::{proj, view};

            const FWD: $vec3 = $vec3::Z;
            const RIGHT: $vec3 = $vec3::X;
            const UP: $vec3 = $vec3::Y;

            glam_test!(test_opengl_perspective, {
                let v = view::look_at_mat4($vec3::ZERO, FWD * 5.0, UP);
                let p = proj::opengl::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, -1.0, 1.0, false);
            });

            glam_test!(test_vulkan_perspective, {
                let v = view::look_at_mat4($vec3::ZERO, FWD * 5.0, UP);
                let p = proj::vulkan::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, 0.0, 1.0, true);
            });

            glam_test!(test_directx_perspective, {
                let v = view::look_at_mat4($vec3::ZERO, FWD * 5.0, UP);
                let p = proj::directx::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, 0.0, 1.0, false);
            });

            glam_test!(test_opengl_affine3, {
                let a = view::look_at_affine3($vec3::ZERO, FWD * 5.0, UP);
                let v = $mat4::from(a);
                let p = proj::opengl::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, -1.0, 1.0, false);
            });

            glam_test!(test_opengl_look_to, {
                let v = view::look_to_mat4($vec3::ZERO, FWD, UP);
                let p = proj::opengl::perspective($t::to_radians(90.0), 1.0, 1.0, 10.0);
                check_view_proj_pipeline(FWD, RIGHT, UP, v, p, -1.0, 1.0, false);
            });
        }
    };
}

mod camera {
    use glam::{camera, Affine3, Mat3, Mat4, Quat, Vec3, Vec4};
    impl_camera_tests!(f32, Affine3, Mat4, Mat3, Vec4, Vec3, Quat, camera);

    // mod view_rh_zup {
    //     use glam::camera::rh_zup::view;
    //     use glam::Vec3;

    //     glam_test!(test_look_at_mat4, {
    //         let eye = Vec3::new(0.0, 5.0, 0.0);
    //         let center = Vec3::ZERO;
    //         let up = Vec3::new(0.0, 0.0, 1.0);
    //         let point = Vec3::new(1.0, 0.0, 0.0);

    //         let m = view::look_at_mat4(eye, center, up);
    //         let p = m.transform_point3(point);
    //         assert_approx_eq!(p, Vec3::new(-1.0, 0.0, -5.0), 1.0e-6);
    //     });
    // }

    // mod view_lh_zup {
    //     use glam::camera::lh_zup::view;
    //     use glam::Vec3;

    //     glam_test!(test_look_at_mat4, {
    //         let eye = Vec3::new(0.0, 5.0, 0.0);
    //         let center = Vec3::ZERO;
    //         let up = Vec3::new(0.0, 0.0, 1.0);
    //         let point = Vec3::new(1.0, 0.0, 0.0);

    //         let m = view::look_at_mat4(eye, center, up);
    //         let p = m.transform_point3(point);
    //         assert_approx_eq!(p, Vec3::new(1.0, 0.0, -5.0), 1.0e-6);
    //     });
    // }

    // mod proj_rh_zup {
    //     use glam::camera::rh_zup::proj;
    //     use glam::camera::rh_yup::proj as yup_proj;

    //     glam_test!(test_opengl_reexport, {
    //         let fov = f32::to_radians(90.0);
    //         let p1 = proj::opengl::perspective(fov, 2.0, 5.0, 15.0);
    //         let p2 = yup_proj::opengl::perspective(fov, 2.0, 5.0, 15.0);
    //         assert_approx_eq!(p1, p2);
    //     });
    // }

    mod affine3a {
        use glam::camera::rh_yup::view;
        use glam::Vec3;

        glam_test!(test_look_at_affine3a, {
            let eye = Vec3::new(0.0, 0.0, -5.0);
            let center = Vec3::ZERO;
            let up = Vec3::new(1.0, 0.0, 0.0);
            let point = Vec3::new(1.0, 0.0, 0.0);

            let a = view::look_at_affine3a(eye, center, up);
            assert_approx_eq!(a.transform_point3(point), Vec3::new(0.0, 1.0, -5.0));
        });
    }
}

#[cfg(feature = "f64")]
mod dcamera {
    use glam::{dcamera, DAffine3, DMat3, DMat4, DQuat, DVec3, DVec4};
    impl_camera_tests!(f64, DAffine3, DMat4, DMat3, DVec4, DVec3, DQuat, dcamera);

    // mod view_rh_zup {
    //     use glam::dcamera::rh_zup::view;
    //     use glam::DVec3;

    //     glam_test!(test_look_at_mat4, {
    //         let eye = DVec3::new(0.0, 5.0, 0.0);
    //         let center = DVec3::ZERO;
    //         let up = DVec3::new(0.0, 0.0, 1.0);
    //         let point = DVec3::new(1.0, 0.0, 0.0);

    //         let m = view::look_at_mat4(eye, center, up);
    //         let p = m.transform_point3(point);
    //         assert_approx_eq!(p, DVec3::new(-1.0, 0.0, -5.0), 1.0e-12);
    //     });
    // }

    // mod view_lh_zup {
    //     use glam::dcamera::lh_zup::view;
    //     use glam::DVec3;

    //     glam_test!(test_look_at_mat4, {
    //         let eye = DVec3::new(0.0, 5.0, 0.0);
    //         let center = DVec3::ZERO;
    //         let up = DVec3::new(0.0, 0.0, 1.0);
    //         let point = DVec3::new(1.0, 0.0, 0.0);

    //         let m = view::look_at_mat4(eye, center, up);
    //         let p = m.transform_point3(point);
    //         assert_approx_eq!(p, DVec3::new(1.0, 0.0, -5.0), 1.0e-12);
    //     });
    // }
}
