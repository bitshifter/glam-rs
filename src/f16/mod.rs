//! Half precision (f16) vector and quaternion types.
//!
//! This module provides 16-bit floating point vector and quaternion types,
//! enabled by the `f16` cargo feature:
//!
//! * [`HVec2`], [`HVec3`] and [`HVec4`]
//! * [`HQuat`]
//!
//! The scalar type is [`half::f16`].

pub(crate) mod math;

mod hquat;
mod hvec2;
mod hvec3;
mod hvec4;

pub use hquat::{hquat, HQuat};
pub use hvec2::{hvec2, HVec2};
pub use hvec3::{hvec3, HVec3};
pub use hvec4::{hvec4, HVec4};

use crate::float::FloatExt;
use half::f16;

/// `PI` constant for `f16`, which is not provided by the `half` crate.
pub(crate) const PI: f16 = f16::from_f32_const(core::f32::consts::PI);

impl FloatExt for f16 {
    #[inline]
    fn lerp(self, rhs: Self, t: Self) -> Self {
        self + (rhs - self) * t
    }

    #[inline]
    fn inverse_lerp(a: Self, b: Self, v: Self) -> Self {
        (v - a) / (b - a)
    }

    #[inline]
    fn remap(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self {
        let t = Self::inverse_lerp(in_start, in_end, self);
        Self::lerp(out_start, out_end, t)
    }

    #[inline]
    fn fract_gl(self) -> Self {
        self - math::floor(self)
    }

    #[inline]
    fn step(self, value: Self) -> Self {
        if value < self {
            f16::ZERO
        } else {
            f16::ONE
        }
    }

    #[inline]
    fn saturate(self) -> Self {
        self.clamp(f16::ZERO, f16::ONE)
    }
}
