use crate::{Mat4, Quat, Vec3, Vec3A, Vec3Swizzles, Vec4Swizzles};
use core::ops::Mul;

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub struct TransformSrt {
    pub scale: Vec3,
    pub rotation: Quat,
    pub translation: Vec3,
}

impl Default for TransformSrt {
    #[inline]
    fn default() -> Self {
        Self {
            scale: Vec3::ONE,
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub struct TransformRt {
    pub rotation: Quat,
    pub translation: Vec3,
}

impl Default for TransformRt {
    #[inline]
    fn default() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
        }
    }
}

impl TransformSrt {
    /// The identity transforms that does nothing.
    pub const IDENTITY: Self = Self {
        scale: Vec3::ONE,
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };

    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        Self {
            scale,
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn from_transform_rt(scale: Vec3, rt: &TransformRt) -> Self {
        Self {
            scale,
            rotation: rt.rotation,
            translation: rt.translation,
        }
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let scale = self.scale.recip();
        let rotation = self.rotation.conjugate();
        let translation = -(rotation * (self.translation * scale));
        Self {
            scale,
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let rotation = self.rotation.normalize();
        Self {
            scale: self.scale,
            rotation,
            translation: self.translation,
        }
    }

    #[inline]
    pub fn mul_transform(&self, other: &Self) -> Self {
        mul_srt_srt(self, other)
    }

    #[deprecated(
        since = "0.15.0",
        note = "Please use `transform_point3(other)` instead"
    )]
    #[inline]
    pub fn transform_vec3(&self, other: Vec3) -> Vec3 {
        self.transform_point3(other)
    }

    #[inline]
    pub fn transform_point3(&self, other: Vec3) -> Vec3 {
        (self.rotation * (other * self.scale)) + self.translation
    }

    #[inline]
    pub fn transform_vector3(&self, other: Vec3) -> Vec3 {
        self.rotation * (other * self.scale)
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Mat4`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f32) -> bool {
        self.scale.abs_diff_eq(other.scale, max_abs_diff)
            && self.rotation.abs_diff_eq(other.rotation, max_abs_diff)
            && self
                .translation
                .abs_diff_eq(other.translation, max_abs_diff)
    }
}

#[inline]
fn mul_srt_srt(lhs: &TransformSrt, rhs: &TransformSrt) -> TransformSrt {
    // Based on https://github.com/nfrechette/rtm `rtm::qvv_mul`
    let lhs_scale = Vec3A::from(lhs.scale);
    let rhs_scale = Vec3A::from(rhs.scale);
    let min_scale = lhs_scale.min(rhs_scale);
    let scale = lhs_scale * rhs_scale;

    if min_scale.cmplt(Vec3A::ZERO).any() {
        // If negative scale, we go through a matrix
        let lhs_mtx =
            Mat4::from_scale_rotation_translation(lhs.scale, lhs.rotation, lhs.translation);
        let rhs_mtx =
            Mat4::from_scale_rotation_translation(rhs.scale, rhs.rotation, rhs.translation);
        let mut result_mtx = lhs_mtx * rhs_mtx;

        let sign = scale.signum();
        result_mtx.x_axis = (Vec3A::from(result_mtx.x_axis).normalize() * sign.xxx()).extend(0.0);
        result_mtx.y_axis = (Vec3A::from(result_mtx.y_axis).normalize() * sign.yyy()).extend(0.0);
        result_mtx.z_axis = (Vec3A::from(result_mtx.z_axis).normalize() * sign.zzz()).extend(0.0);

        let scale = Vec3::from(scale);
        let rotation = Quat::from_rotation_mat4(&result_mtx);
        let translation = result_mtx.w_axis.xyz();
        TransformSrt {
            scale,
            rotation,
            translation,
        }
    } else {
        let scale = Vec3::from(scale);
        let rotation = lhs.rotation * rhs.rotation;
        let translation = Vec3::from(
            (rhs.rotation * (Vec3A::from(lhs.translation) * rhs_scale))
                + Vec3A::from(rhs.translation),
        );
        TransformSrt {
            scale,
            rotation,
            translation,
        }
    }
}

#[inline]
fn mul_rt_rt(lhs: &TransformRt, rhs: &TransformRt) -> TransformRt {
    let rotation = lhs.rotation * rhs.rotation;
    let translation = (rhs.rotation * lhs.translation) + rhs.translation;
    TransformRt {
        rotation,
        translation,
    }
}

impl TransformRt {
    /// The identity transforms that does nothing.
    pub const IDENTITY: Self = Self {
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };

    #[inline]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        Self {
            rotation,
            translation,
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.rotation.is_finite() && self.translation.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.rotation.is_nan() || self.translation.is_nan()
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.conjugate();
        let translation = -(rotation * self.translation);
        Self {
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let rotation = self.rotation.normalize();
        Self {
            rotation,
            translation: self.translation,
        }
    }

    #[inline]
    pub fn mul_transform(&self, other: &Self) -> Self {
        mul_rt_rt(self, other)
    }

    #[deprecated(
        since = "0.15.0",
        note = "Please use `transform_point3(other)` instead"
    )]
    #[inline]
    pub fn transform_vec3(self, other: Vec3) -> Vec3 {
        self.transform_point3(other)
    }

    #[inline]
    pub fn transform_point3(&self, other: Vec3) -> Vec3 {
        (self.rotation * other) + self.translation
    }

    #[inline]
    pub fn transform_vector3(&self, other: Vec3) -> Vec3 {
        self.rotation * other
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Mat4`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f32) -> bool {
        self.rotation.abs_diff_eq(other.rotation, max_abs_diff)
            && self
                .translation
                .abs_diff_eq(other.translation, max_abs_diff)
    }
}

impl Mul<Vec3> for TransformRt {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        self.transform_point3(other)
    }
}

impl Mul<Vec3> for TransformSrt {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        self.transform_point3(other)
    }
}

impl Mul<TransformRt> for TransformRt {
    type Output = TransformRt;
    #[inline]
    fn mul(self, other: TransformRt) -> TransformRt {
        mul_rt_rt(&self, &other)
    }
}

impl Mul<TransformSrt> for TransformSrt {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        mul_srt_srt(&self, &other)
    }
}

impl Mul<TransformRt> for TransformSrt {
    type Output = TransformSrt;
    #[inline]
    fn mul(self, other: TransformRt) -> Self::Output {
        mul_srt_srt(&self, &other.into())
    }
}

impl Mul<TransformSrt> for TransformRt {
    type Output = TransformSrt;
    #[inline]
    fn mul(self, other: TransformSrt) -> Self::Output {
        mul_srt_srt(&self.into(), &other)
    }
}

impl From<TransformRt> for TransformSrt {
    #[inline]
    fn from(tr: TransformRt) -> Self {
        Self {
            translation: tr.translation,
            rotation: tr.rotation,
            scale: Vec3::ONE,
        }
    }
}

#[cfg(feature = "rand")]
impl Distribution<TransformRt> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TransformRt {
        TransformRt::from_rotation_translation(
            rng.gen::<Quat>(),
            Vec3::new(
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
            ),
        )
    }
}

#[cfg(feature = "rand")]
impl Distribution<TransformSrt> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TransformSrt {
        let mut gen_non_zero = || loop {
            let f: f32 = rng.gen_range(core::f32::MIN..=core::f32::MAX);
            if f.abs() > core::f32::MIN_POSITIVE {
                return f;
            }
        };
        TransformSrt::from_scale_rotation_translation(
            Vec3::new(gen_non_zero(), gen_non_zero(), gen_non_zero()),
            rng.gen::<Quat>(),
            Vec3::new(
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
            ),
        )
    }
}

impl From<TransformSrt> for Mat4 {
    #[inline]
    fn from(srt: TransformSrt) -> Self {
        Mat4::from_scale_rotation_translation(srt.scale, srt.rotation, srt.translation)
    }
}

impl From<TransformRt> for Mat4 {
    #[inline]
    fn from(rt: TransformRt) -> Self {
        Mat4::from_rotation_translation(rt.rotation, rt.translation)
    }
}
