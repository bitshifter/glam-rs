use crate::{Affine3A, Mat4, Quat, Vec3, Vec3A, Vec3Swizzles};
use core::ops::Mul;

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/**
 * A transform containing non-uniform scale, rotation and translation.
 *
 * Scale and translation are stored as `Vec3A` for better performance.
 */
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Transform3A {
    pub translation: Vec3A,
    pub rotation: Quat,
    pub scale: Vec3A,
}

impl Default for Transform3A {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

/**
 * A transform containing rotation and translation.
 *
 * Translation is stored as a `Vec3A` for better performance.
 */
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Isometry3A {
    pub translation: Vec3A,
    pub rotation: Quat,
}

impl Default for Isometry3A {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform3A {
    /// The identity transforms that does nothing.
    pub const IDENTITY: Self = Self {
        scale: Vec3A::ONE,
        rotation: Quat::IDENTITY,
        translation: Vec3A::ZERO,
    };

    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        Self {
            scale: scale.into(),
            rotation,
            translation: translation.into(),
        }
    }

    #[inline]
    pub fn from_scale_isometry(scale: Vec3, rt: &Isometry3A) -> Self {
        Self {
            scale: scale.into(),
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
        self.transform_point3a(other.into()).into()
    }

    #[inline]
    pub fn transform_vector3(&self, other: Vec3) -> Vec3 {
        self.transform_vector3a(other.into()).into()
    }

    #[inline]
    pub fn transform_point3a(&self, other: Vec3A) -> Vec3A {
        (self.rotation * (other * self.scale)) + self.translation
    }

    #[inline]
    pub fn transform_vector3a(&self, other: Vec3A) -> Vec3A {
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
fn mul_srt_srt(lhs: &Transform3A, rhs: &Transform3A) -> Transform3A {
    // Based on https://github.com/nfrechette/rtm `rtm::qvv_mul`
    let min_scale = lhs.scale.min(rhs.scale);
    let scale = lhs.scale * rhs.scale;

    if min_scale.cmplt(Vec3A::ZERO).any() {
        // If negative scale, we go through a matrix
        let lhs_mtx = Affine3A::from_scale_rotation_translation(
            lhs.scale.into(),
            lhs.rotation,
            lhs.translation.into(),
        );
        let rhs_mtx = Affine3A::from_scale_rotation_translation(
            rhs.scale.into(),
            rhs.rotation,
            rhs.translation.into(),
        );
        let mut result_mtx = lhs_mtx * rhs_mtx;

        let sign = scale.signum();
        result_mtx.x_axis = result_mtx.x_axis.normalize() * sign.xxx();
        result_mtx.y_axis = result_mtx.y_axis.normalize() * sign.yyy();
        result_mtx.z_axis = result_mtx.z_axis.normalize() * sign.zzz();

        let scale = scale;
        let rotation = Quat::from_affine3(&result_mtx);
        let translation = result_mtx.w_axis.xyz();
        Transform3A {
            scale,
            rotation,
            translation,
        }
    } else {
        let rotation = lhs.rotation * rhs.rotation;
        let translation = (rhs.rotation * (lhs.translation * rhs.scale)) + rhs.translation;
        Transform3A {
            scale,
            rotation,
            translation,
        }
    }
}

#[inline]
fn mul_rt_rt(lhs: &Isometry3A, rhs: &Isometry3A) -> Isometry3A {
    let rotation = lhs.rotation * rhs.rotation;
    let translation = (rhs.rotation * lhs.translation) + rhs.translation;
    Isometry3A {
        rotation,
        translation,
    }
}

impl Isometry3A {
    /// The identity transforms that does nothing.
    pub const IDENTITY: Self = Self {
        rotation: Quat::IDENTITY,
        translation: Vec3A::ZERO,
    };

    #[inline]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        Self {
            rotation,
            translation: translation.into(),
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
        self.transform_point3a(other.into()).into()
    }

    #[inline]
    pub fn transform_vector3(&self, other: Vec3) -> Vec3 {
        self.transform_vector3a(other.into()).into()
    }

    #[inline]
    pub fn transform_point3a(&self, other: Vec3A) -> Vec3A {
        (self.rotation * other) + self.translation
    }

    #[inline]
    pub fn transform_vector3a(&self, other: Vec3A) -> Vec3A {
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

impl Mul<Isometry3A> for Isometry3A {
    type Output = Isometry3A;
    #[inline]
    fn mul(self, other: Isometry3A) -> Isometry3A {
        mul_rt_rt(&self, &other)
    }
}

impl Mul<Transform3A> for Transform3A {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        mul_srt_srt(&self, &other)
    }
}

impl Mul<Isometry3A> for Transform3A {
    type Output = Transform3A;
    #[inline]
    fn mul(self, other: Isometry3A) -> Self::Output {
        mul_srt_srt(&self, &other.into())
    }
}

impl Mul<Transform3A> for Isometry3A {
    type Output = Transform3A;
    #[inline]
    fn mul(self, other: Transform3A) -> Self::Output {
        mul_srt_srt(&self.into(), &other)
    }
}

impl From<Isometry3A> for Transform3A {
    #[inline]
    fn from(tr: Isometry3A) -> Self {
        Self {
            translation: tr.translation,
            rotation: tr.rotation,
            scale: Vec3A::ONE,
        }
    }
}

#[cfg(feature = "rand")]
impl Distribution<Isometry3A> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Isometry3A {
        Isometry3A::from_rotation_translation(
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
impl Distribution<Transform3A> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Transform3A {
        let mut gen_non_zero = || loop {
            let f: f32 = rng.gen_range(core::f32::MIN..=core::f32::MAX);
            if f.abs() > core::f32::MIN_POSITIVE {
                return f;
            }
        };
        Transform3A::from_scale_rotation_translation(
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

impl From<Transform3A> for Mat4 {
    #[inline]
    fn from(srt: Transform3A) -> Self {
        Self::from_scale_rotation_translation(
            srt.scale.into(),
            srt.rotation,
            srt.translation.into(),
        )
    }
}

impl From<Isometry3A> for Mat4 {
    #[inline]
    fn from(rt: Isometry3A) -> Self {
        Self::from_rotation_translation(rt.rotation, rt.translation.into())
    }
}

impl From<Transform3A> for Affine3A {
    #[inline]
    fn from(srt: Transform3A) -> Self {
        Self::from_scale_rotation_translation(
            srt.scale.into(),
            srt.rotation,
            srt.translation.into(),
        )
    }
}

impl From<Isometry3A> for Affine3A {
    #[inline]
    fn from(rt: Isometry3A) -> Self {
        Self::from_rotation_translation(rt.rotation, rt.translation.into())
    }
}
