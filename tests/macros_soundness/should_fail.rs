unsafe fn some_random_unsafe_fn() {}

macro_rules! test_soundness {
    ($m:ident ! ( $($arg:expr),* )) => {
        let _ = glam::$m!($({
            some_random_unsafe_fn();
            $arg
        }),*);
    }
}

fn main() {
    test_soundness!(const_m128!([1.0; 4]));
    test_soundness!(const_vec2!([1.0; 2]));
    test_soundness!(const_vec3!([1.0; 3]));
    test_soundness!(const_vec3a!([1.0; 3]));
    test_soundness!(const_vec4!([1.0; 4]));
    test_soundness!(const_mat2!([1.0; 2], [1.0; 2]));
    test_soundness!(const_mat2!([1.0; 4]));
    test_soundness!(const_mat3!([1.0; 3], [1.0; 3], [1.0; 3]));
    test_soundness!(const_mat3!([1.0; 9]));
    test_soundness!(const_mat3a!([1.0; 3], [1.0; 3], [1.0; 3]));
    test_soundness!(const_mat3a!([1.0; 9]));
    test_soundness!(const_mat4!([1.0; 4], [1.0; 4], [1.0; 4], [1.0; 4]));
    test_soundness!(const_mat4!([1.0; 16]));
    test_soundness!(const_quat!([1.0; 4]));
    test_soundness!(const_dvec2!([1.0; 2]));
    test_soundness!(const_dvec3!([1.0; 3]));
    test_soundness!(const_dvec4!([1.0; 4]));
    test_soundness!(const_dmat2!([1.0; 2], [1.0; 2]));
    test_soundness!(const_dmat2!([1.0; 4]));
    test_soundness!(const_dmat3!([1.0; 3], [1.0; 3], [1.0; 3]));
    test_soundness!(const_dmat3!([1.0; 9]));
    test_soundness!(const_dmat4!([1.0; 4], [1.0; 4], [1.0; 4], [1.0; 4]));
    test_soundness!(const_dmat4!([1.0; 16]));
    test_soundness!(const_dquat!([1.0; 4]));
    test_soundness!(const_ivec2!([1; 2]));
    test_soundness!(const_ivec3!([1; 3]));
    test_soundness!(const_ivec4!([1; 4]));
    test_soundness!(const_uvec2!([1; 2]));
    test_soundness!(const_uvec3!([1; 3]));
    test_soundness!(const_uvec4!([1; 4]));
}
