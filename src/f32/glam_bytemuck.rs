use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
#[cfg(feature = "transform-types")]
use crate::{TransformRT, TransformSRT};
use bytemuck::{Pod, Zeroable};

unsafe impl Pod for Mat2 {}
unsafe impl Zeroable for Mat2 {}
unsafe impl Pod for Mat3 {}
unsafe impl Zeroable for Mat3 {}
unsafe impl Pod for Mat4 {}
unsafe impl Zeroable for Mat4 {}

unsafe impl Pod for Quat {}
unsafe impl Zeroable for Quat {}

unsafe impl Pod for Vec2 {}
unsafe impl Zeroable for Vec2 {}
unsafe impl Pod for Vec3 {}
unsafe impl Zeroable for Vec3 {}
unsafe impl Pod for Vec4 {}
unsafe impl Zeroable for Vec4 {}

#[cfg(feature = "transform-types")]
unsafe impl Pod for TransformRT {}
#[cfg(feature = "transform-types")]
unsafe impl Zeroable for TransformRT {}
#[cfg(feature = "transform-types")]
unsafe impl Pod for TransformSRT {}
#[cfg(feature = "transform-types")]
unsafe impl Zeroable for TransformSRT {}

#[cfg(test)]
mod test {
    use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
    use bytemuck;
    use core::mem;

    macro_rules! test_t {
        ($name:ident, $t:ty) => {
            #[test]
            fn $name() {
                let t = <$t>::default();
                let b = bytemuck::bytes_of(&t);
                assert_eq!(t.as_ref().as_ptr() as usize, b.as_ptr() as usize);
                assert_eq!(b.len(), mem::size_of_val(&t));
            }
        };
    }

    test_t!(mat2, Mat2);
    test_t!(mat3, Mat3);
    test_t!(mat4, Mat4);
    test_t!(quat, Quat);
    test_t!(vec2, Vec2);
    test_t!(vec3, Vec3);
    test_t!(vec4, Vec4);
}
