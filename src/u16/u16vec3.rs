// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{
    BVec3, BVec3A, I16Vec3, I64Vec3, I8Vec3, IVec3, U16Vec2, U16Vec4, U64Vec3, U8Vec3, USizeVec3,
    UVec3,
};

use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

/// Creates a 3-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn u16vec3(x: u16, y: u16, z: u16) -> U16Vec3 {
    U16Vec3::new(x, y, z)
}

/// A 3-dimensional vector.
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
pub struct U16Vec3 {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl U16Vec3 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// All ones.
    pub const ONE: Self = Self::splat(1);

    /// All `u16::MIN`.
    pub const MIN: Self = Self::splat(u16::MIN);

    /// All `u16::MAX`.
    pub const MAX: Self = Self::splat(u16::MAX);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1, 0, 0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0, 1, 0);

    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0, 0, 1);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: u16) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Returns a vector containing each element of `self` modified by a mapping function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(u16) -> u16,
    {
        Self::new(f(self.x), f(self.y), f(self.z))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    #[must_use]
    pub fn select(mask: BVec3, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.test(0) { if_true.x } else { if_false.x },
            y: if mask.test(1) { if_true.y } else { if_false.y },
            z: if mask.test(2) { if_true.z } else { if_false.z },
        }
    }

    /// Creates a new vector from an array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [u16; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// `[x, y, z]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [u16; 3] {
        [self.x, self.y, self.z]
    }

    /// Creates a vector from the first 3 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[u16]) -> Self {
        assert!(slice.len() >= 3);
        Self::new(slice[0], slice[1], slice[2])
    }

    /// Writes the elements of `self` to the first 3 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [u16]) {
        slice[..3].copy_from_slice(&self.to_array());
    }

    /// Internal method for creating a 3D vector from a 4D vector, discarding `w`.
    #[allow(dead_code)]
    #[inline]
    #[must_use]
    pub(crate) fn from_vec4(v: U16Vec4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    /// Creates a 4D vector from `self` and the given `w` value.
    #[inline]
    #[must_use]
    pub fn extend(self, w: u16) -> U16Vec4 {
        U16Vec4::new(self.x, self.y, self.z, w)
    }

    /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].
    #[inline]
    #[must_use]
    pub fn truncate(self) -> U16Vec2 {
        use crate::swizzles::Vec3Swizzles;
        self.xy()
    }

    /// Creates a 3D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }

    /// Creates a 3D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: u16) -> Self {
        self.y = y;
        self
    }

    /// Creates a 3D vector from `self` with the given value of `z`.
    #[inline]
    #[must_use]
    pub fn with_z(mut self, z: u16) -> Self {
        self.z = z;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> u16 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot_into_vec(self, rhs: Self) -> Self {
        Self::splat(self.dot(rhs))
    }

    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: if self.x < rhs.x { self.x } else { rhs.x },
            y: if self.y < rhs.y { self.y } else { rhs.y },
            z: if self.z < rhs.z { self.z } else { rhs.z },
        }
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: if self.x > rhs.x { self.x } else { rhs.x },
            y: if self.y > rhs.y { self.y } else { rhs.y },
            z: if self.z > rhs.z { self.z } else { rhs.z },
        }
    }

    /// Component-wise clamping of values, similar to [`u16::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.cmple(max).all(), "clamp: expected min <= max");
        self.max(min).min(max)
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn min_element(self) -> u16 {
        let min = |a, b| if a < b { a } else { b };

        min(self.x, min(self.y, self.z))
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> u16 {
        let max = |a, b| if a > b { a } else { b };

        max(self.x, max(self.y, self.z))
    }

    /// Returns the index of the first minimum element of `self`.
    #[doc(alias = "argmin")]
    #[inline]
    #[must_use]
    pub fn min_position(self) -> usize {
        let mut min = self.x;
        let mut index = 0;
        if self.y < min {
            min = self.y;
            index = 1;
        }
        if self.z < min {
            index = 2;
        }
        index
    }

    /// Returns the index of the first maximum element of `self`.
    #[doc(alias = "argmax")]
    #[inline]
    #[must_use]
    pub fn max_position(self) -> usize {
        let mut max = self.x;
        let mut index = 0;
        if self.y > max {
            max = self.y;
            index = 1;
        }
        if self.z > max {
            index = 2;
        }
        index
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> u16 {
        self.x + self.y + self.z
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> u16 {
        self.x * self.y * self.z
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpeq(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.eq(&rhs.x), self.y.eq(&rhs.y), self.z.eq(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpne(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.ne(&rhs.x), self.y.ne(&rhs.y), self.z.ne(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpge(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.ge(&rhs.x), self.y.ge(&rhs.y), self.z.ge(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpgt(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.gt(&rhs.x), self.y.gt(&rhs.y), self.z.gt(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmple(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.le(&rhs.x), self.y.le(&rhs.y), self.z.le(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmplt(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.lt(&rhs.x), self.y.lt(&rhs.y), self.z.lt(&rhs.z))
    }

    /// Computes the squared length of `self`.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> u16 {
        self.dot(self)
    }

    /// Computes the [manhattan distance] between two points.
    ///
    /// # Overflow
    /// This method may overflow if the result is greater than [`u16::MAX`].
    ///
    /// See also [`checked_manhattan_distance`][U16Vec3::checked_manhattan_distance].
    ///
    /// [manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    #[inline]
    #[must_use]
    pub fn manhattan_distance(self, other: Self) -> u16 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    /// Computes the [manhattan distance] between two points.
    ///
    /// This will returns [`None`] if the result is greater than [`u16::MAX`].
    ///
    /// [manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    #[inline]
    #[must_use]
    pub fn checked_manhattan_distance(self, other: Self) -> Option<u16> {
        let d = self.x.abs_diff(other.x);
        let d = d.checked_add(self.y.abs_diff(other.y))?;
        d.checked_add(self.z.abs_diff(other.z))
    }

    /// Computes the [chebyshev distance] between two points.
    ///
    /// [chebyshev distance]: https://en.wikipedia.org/wiki/Chebyshev_distance
    #[inline]
    #[must_use]
    pub fn chebyshev_distance(self, other: Self) -> u16 {
        // Note: the compiler will eventually optimize out the loop
        [
            self.x.abs_diff(other.x),
            self.y.abs_diff(other.y),
            self.z.abs_diff(other.z),
        ]
        .into_iter()
        .max()
        .unwrap()
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[must_use]
    pub fn as_vec3(&self) -> crate::Vec3 {
        crate::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[must_use]
    pub fn as_vec3a(&self) -> crate::Vec3A {
        crate::Vec3A::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `f64`.
    #[inline]
    #[must_use]
    pub fn as_dvec3(&self) -> crate::DVec3 {
        crate::DVec3::new(self.x as f64, self.y as f64, self.z as f64)
    }

    /// Casts all elements of `self` to `i8`.
    #[inline]
    #[must_use]
    pub fn as_i8vec3(&self) -> crate::I8Vec3 {
        crate::I8Vec3::new(self.x as i8, self.y as i8, self.z as i8)
    }

    /// Casts all elements of `self` to `u8`.
    #[inline]
    #[must_use]
    pub fn as_u8vec3(&self) -> crate::U8Vec3 {
        crate::U8Vec3::new(self.x as u8, self.y as u8, self.z as u8)
    }

    /// Casts all elements of `self` to `i16`.
    #[inline]
    #[must_use]
    pub fn as_i16vec3(&self) -> crate::I16Vec3 {
        crate::I16Vec3::new(self.x as i16, self.y as i16, self.z as i16)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline]
    #[must_use]
    pub fn as_ivec3(&self) -> crate::IVec3 {
        crate::IVec3::new(self.x as i32, self.y as i32, self.z as i32)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline]
    #[must_use]
    pub fn as_uvec3(&self) -> crate::UVec3 {
        crate::UVec3::new(self.x as u32, self.y as u32, self.z as u32)
    }

    /// Casts all elements of `self` to `i64`.
    #[inline]
    #[must_use]
    pub fn as_i64vec3(&self) -> crate::I64Vec3 {
        crate::I64Vec3::new(self.x as i64, self.y as i64, self.z as i64)
    }

    /// Casts all elements of `self` to `u64`.
    #[inline]
    #[must_use]
    pub fn as_u64vec3(&self) -> crate::U64Vec3 {
        crate::U64Vec3::new(self.x as u64, self.y as u64, self.z as u64)
    }

    /// Casts all elements of `self` to `usize`.
    #[inline]
    #[must_use]
    pub fn as_usizevec3(&self) -> crate::USizeVec3 {
        crate::USizeVec3::new(self.x as usize, self.y as usize, self.z as usize)
    }

    /// Returns a vector containing the wrapping addition of `self` and `rhs`.
    ///
    /// In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let x = match self.x.checked_add(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_add(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_add(rhs.z) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z })
    }

    /// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
    ///
    /// In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        let x = match self.x.checked_sub(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_sub(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_sub(rhs.z) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z })
    }

    /// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
    ///
    /// In other words this computes `Some([self.x * rhs.x, self.y * rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        let x = match self.x.checked_mul(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_mul(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_mul(rhs.z) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z })
    }

    /// Returns a vector containing the wrapping division of `self` and `rhs`.
    ///
    /// In other words this computes `Some([self.x / rhs.x, self.y / rhs.y, ..])` but returns `None` on any division by zero.
    #[inline]
    #[must_use]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        let x = match self.x.checked_div(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_div(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_div(rhs.z) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z })
    }

    /// Returns a vector containing the wrapping addition of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            z: self.z.wrapping_add(rhs.z),
        }
    }

    /// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            z: self.z.wrapping_sub(rhs.z),
        }
    }

    /// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_mul(rhs.x),
            y: self.y.wrapping_mul(rhs.y),
            z: self.z.wrapping_mul(rhs.z),
        }
    }

    /// Returns a vector containing the wrapping division of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_div(rhs.x),
            y: self.y.wrapping_div(rhs.y),
            z: self.z.wrapping_div(rhs.z),
        }
    }

    /// Returns a vector containing the saturating addition of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
            z: self.z.saturating_add(rhs.z),
        }
    }

    /// Returns a vector containing the saturating subtraction of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
            z: self.z.saturating_sub(rhs.z),
        }
    }

    /// Returns a vector containing the saturating multiplication of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_mul(rhs.x),
            y: self.y.saturating_mul(rhs.y),
            z: self.z.saturating_mul(rhs.z),
        }
    }

    /// Returns a vector containing the saturating division of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_div(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_div(rhs.x),
            y: self.y.saturating_div(rhs.y),
            z: self.z.saturating_div(rhs.z),
        }
    }

    /// Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_add_signed(self, rhs: I16Vec3) -> Option<Self> {
        let x = match self.x.checked_add_signed(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_add_signed(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_add_signed(rhs.z) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z })
    }

    /// Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_add_signed(rhs.x), self.y.wrapping_add_signed(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_add_signed(self, rhs: I16Vec3) -> Self {
        Self {
            x: self.x.wrapping_add_signed(rhs.x),
            y: self.y.wrapping_add_signed(rhs.y),
            z: self.z.wrapping_add_signed(rhs.z),
        }
    }

    /// Returns a vector containing the saturating addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_add_signed(rhs.x), self.y.saturating_add_signed(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_add_signed(self, rhs: I16Vec3) -> Self {
        Self {
            x: self.x.saturating_add_signed(rhs.x),
            y: self.y.saturating_add_signed(rhs.y),
            z: self.z.saturating_add_signed(rhs.z),
        }
    }
}

impl Default for U16Vec3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div<U16Vec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
        }
    }
}

impl Div<&U16Vec3> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: &U16Vec3) -> U16Vec3 {
        self.div(*rhs)
    }
}

impl Div<&U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).div(*rhs)
    }
}

impl Div<U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).div(rhs)
    }
}

impl DivAssign<U16Vec3> for U16Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
    }
}

impl DivAssign<&U16Vec3> for U16Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &U16Vec3) {
        self.div_assign(*rhs)
    }
}

impl Div<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: u16) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}

impl Div<&u16> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: &u16) -> U16Vec3 {
        self.div(*rhs)
    }
}

impl Div<&u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: &u16) -> U16Vec3 {
        (*self).div(*rhs)
    }
}

impl Div<u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: u16) -> U16Vec3 {
        (*self).div(rhs)
    }
}

impl DivAssign<u16> for U16Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: u16) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
    }
}

impl DivAssign<&u16> for U16Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &u16) {
        self.div_assign(*rhs)
    }
}

impl Div<U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: U16Vec3) -> U16Vec3 {
        U16Vec3 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
        }
    }
}

impl Div<&U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: &U16Vec3) -> U16Vec3 {
        self.div(*rhs)
    }
}

impl Div<&U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).div(*rhs)
    }
}

impl Div<U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn div(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).div(rhs)
    }
}

impl Mul<U16Vec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}

impl Mul<&U16Vec3> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: &U16Vec3) -> U16Vec3 {
        self.mul(*rhs)
    }
}

impl Mul<&U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).mul(rhs)
    }
}

impl MulAssign<U16Vec3> for U16Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
    }
}

impl MulAssign<&U16Vec3> for U16Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &U16Vec3) {
        self.mul_assign(*rhs)
    }
}

impl Mul<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: u16) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}

impl Mul<&u16> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: &u16) -> U16Vec3 {
        self.mul(*rhs)
    }
}

impl Mul<&u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: &u16) -> U16Vec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: u16) -> U16Vec3 {
        (*self).mul(rhs)
    }
}

impl MulAssign<u16> for U16Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: u16) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
    }
}

impl MulAssign<&u16> for U16Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &u16) {
        self.mul_assign(*rhs)
    }
}

impl Mul<U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: U16Vec3) -> U16Vec3 {
        U16Vec3 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
        }
    }
}

impl Mul<&U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: &U16Vec3) -> U16Vec3 {
        self.mul(*rhs)
    }
}

impl Mul<&U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn mul(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).mul(rhs)
    }
}

impl Add<U16Vec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl Add<&U16Vec3> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: &U16Vec3) -> U16Vec3 {
        self.add(*rhs)
    }
}

impl Add<&U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).add(*rhs)
    }
}

impl Add<U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).add(rhs)
    }
}

impl AddAssign<U16Vec3> for U16Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}

impl AddAssign<&U16Vec3> for U16Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &U16Vec3) {
        self.add_assign(*rhs)
    }
}

impl Add<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u16) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}

impl Add<&u16> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: &u16) -> U16Vec3 {
        self.add(*rhs)
    }
}

impl Add<&u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: &u16) -> U16Vec3 {
        (*self).add(*rhs)
    }
}

impl Add<u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: u16) -> U16Vec3 {
        (*self).add(rhs)
    }
}

impl AddAssign<u16> for U16Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: u16) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
    }
}

impl AddAssign<&u16> for U16Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &u16) {
        self.add_assign(*rhs)
    }
}

impl Add<U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: U16Vec3) -> U16Vec3 {
        U16Vec3 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
        }
    }
}

impl Add<&U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: &U16Vec3) -> U16Vec3 {
        self.add(*rhs)
    }
}

impl Add<&U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).add(*rhs)
    }
}

impl Add<U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn add(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).add(rhs)
    }
}

impl Sub<U16Vec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl Sub<&U16Vec3> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: &U16Vec3) -> U16Vec3 {
        self.sub(*rhs)
    }
}

impl Sub<&U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).sub(rhs)
    }
}

impl SubAssign<U16Vec3> for U16Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: U16Vec3) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}

impl SubAssign<&U16Vec3> for U16Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &U16Vec3) {
        self.sub_assign(*rhs)
    }
}

impl Sub<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u16) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}

impl Sub<&u16> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: &u16) -> U16Vec3 {
        self.sub(*rhs)
    }
}

impl Sub<&u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: &u16) -> U16Vec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: u16) -> U16Vec3 {
        (*self).sub(rhs)
    }
}

impl SubAssign<u16> for U16Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: u16) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}

impl SubAssign<&u16> for U16Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &u16) {
        self.sub_assign(*rhs)
    }
}

impl Sub<U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: U16Vec3) -> U16Vec3 {
        U16Vec3 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
        }
    }
}

impl Sub<&U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: &U16Vec3) -> U16Vec3 {
        self.sub(*rhs)
    }
}

impl Sub<&U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn sub(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).sub(rhs)
    }
}

impl Rem<U16Vec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
            z: self.z.rem(rhs.z),
        }
    }
}

impl Rem<&U16Vec3> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: &U16Vec3) -> U16Vec3 {
        self.rem(*rhs)
    }
}

impl Rem<&U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).rem(*rhs)
    }
}

impl Rem<U16Vec3> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).rem(rhs)
    }
}

impl RemAssign<U16Vec3> for U16Vec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
        self.z.rem_assign(rhs.z);
    }
}

impl RemAssign<&U16Vec3> for U16Vec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: &U16Vec3) {
        self.rem_assign(*rhs)
    }
}

impl Rem<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: u16) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
            z: self.z.rem(rhs),
        }
    }
}

impl Rem<&u16> for U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: &u16) -> U16Vec3 {
        self.rem(*rhs)
    }
}

impl Rem<&u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: &u16) -> U16Vec3 {
        (*self).rem(*rhs)
    }
}

impl Rem<u16> for &U16Vec3 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: u16) -> U16Vec3 {
        (*self).rem(rhs)
    }
}

impl RemAssign<u16> for U16Vec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: u16) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
        self.z.rem_assign(rhs);
    }
}

impl RemAssign<&u16> for U16Vec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: &u16) {
        self.rem_assign(*rhs)
    }
}

impl Rem<U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: U16Vec3) -> U16Vec3 {
        U16Vec3 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
            z: self.rem(rhs.z),
        }
    }
}

impl Rem<&U16Vec3> for u16 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: &U16Vec3) -> U16Vec3 {
        self.rem(*rhs)
    }
}

impl Rem<&U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: &U16Vec3) -> U16Vec3 {
        (*self).rem(*rhs)
    }
}

impl Rem<U16Vec3> for &u16 {
    type Output = U16Vec3;
    #[inline]
    fn rem(self, rhs: U16Vec3) -> U16Vec3 {
        (*self).rem(rhs)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[u16; 3]> for U16Vec3 {
    #[inline]
    fn as_ref(&self) -> &[u16; 3] {
        unsafe { &*(self as *const U16Vec3 as *const [u16; 3]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[u16; 3]> for U16Vec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u16; 3] {
        unsafe { &mut *(self as *mut U16Vec3 as *mut [u16; 3]) }
    }
}

impl Sum for U16Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for U16Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for U16Vec3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for U16Vec3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Not for U16Vec3 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            x: self.x.not(),
            y: self.y.not(),
            z: self.z.not(),
        }
    }
}

impl BitAnd for U16Vec3 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
            z: self.z.bitand(rhs.z),
        }
    }
}

impl BitOr for U16Vec3 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
            z: self.z.bitor(rhs.z),
        }
    }
}

impl BitXor for U16Vec3 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
            z: self.z.bitxor(rhs.z),
        }
    }
}

impl BitAnd<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.bitand(rhs),
            y: self.y.bitand(rhs),
            z: self.z.bitand(rhs),
        }
    }
}

impl BitOr<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.bitor(rhs),
            y: self.y.bitor(rhs),
            z: self.z.bitor(rhs),
        }
    }
}

impl BitXor<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs),
            y: self.y.bitxor(rhs),
            z: self.z.bitxor(rhs),
        }
    }
}

impl Shl<i8> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
        }
    }
}

impl Shr<i8> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
        }
    }
}

impl Shl<i16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
        }
    }
}

impl Shr<i16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
        }
    }
}

impl Shl<i32> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
        }
    }
}

impl Shr<i32> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
        }
    }
}

impl Shl<i64> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
        }
    }
}

impl Shr<i64> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
        }
    }
}

impl Shl<u8> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
        }
    }
}

impl Shr<u8> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
        }
    }
}

impl Shl<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
        }
    }
}

impl Shr<u16> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
        }
    }
}

impl Shl<u32> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
        }
    }
}

impl Shr<u32> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
        }
    }
}

impl Shl<u64> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
        }
    }
}

impl Shr<u64> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
        }
    }
}

impl Shl<crate::IVec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: crate::IVec3) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
            z: self.z.shl(rhs.z),
        }
    }
}

impl Shr<crate::IVec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: crate::IVec3) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
            z: self.z.shr(rhs.z),
        }
    }
}

impl Shl<crate::UVec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: crate::UVec3) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
            z: self.z.shl(rhs.z),
        }
    }
}

impl Shr<crate::UVec3> for U16Vec3 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: crate::UVec3) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
            z: self.z.shr(rhs.z),
        }
    }
}

impl Index<usize> for U16Vec3 {
    type Output = u16;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for U16Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for U16Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl fmt::Debug for U16Vec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(U16Vec3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl From<[u16; 3]> for U16Vec3 {
    #[inline]
    fn from(a: [u16; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<U16Vec3> for [u16; 3] {
    #[inline]
    fn from(v: U16Vec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<(u16, u16, u16)> for U16Vec3 {
    #[inline]
    fn from(t: (u16, u16, u16)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<U16Vec3> for (u16, u16, u16) {
    #[inline]
    fn from(v: U16Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl From<(U16Vec2, u16)> for U16Vec3 {
    #[inline]
    fn from((v, z): (U16Vec2, u16)) -> Self {
        Self::new(v.x, v.y, z)
    }
}

impl From<U8Vec3> for U16Vec3 {
    #[inline]
    fn from(v: U8Vec3) -> Self {
        Self::new(u16::from(v.x), u16::from(v.y), u16::from(v.z))
    }
}

impl TryFrom<I8Vec3> for U16Vec3 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I8Vec3) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u16::try_from(v.x)?,
            u16::try_from(v.y)?,
            u16::try_from(v.z)?,
        ))
    }
}

impl TryFrom<I16Vec3> for U16Vec3 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I16Vec3) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u16::try_from(v.x)?,
            u16::try_from(v.y)?,
            u16::try_from(v.z)?,
        ))
    }
}

impl TryFrom<IVec3> for U16Vec3 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: IVec3) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u16::try_from(v.x)?,
            u16::try_from(v.y)?,
            u16::try_from(v.z)?,
        ))
    }
}

impl TryFrom<UVec3> for U16Vec3 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: UVec3) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u16::try_from(v.x)?,
            u16::try_from(v.y)?,
            u16::try_from(v.z)?,
        ))
    }
}

impl TryFrom<I64Vec3> for U16Vec3 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I64Vec3) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u16::try_from(v.x)?,
            u16::try_from(v.y)?,
            u16::try_from(v.z)?,
        ))
    }
}

impl TryFrom<U64Vec3> for U16Vec3 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: U64Vec3) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u16::try_from(v.x)?,
            u16::try_from(v.y)?,
            u16::try_from(v.z)?,
        ))
    }
}

impl TryFrom<USizeVec3> for U16Vec3 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: USizeVec3) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u16::try_from(v.x)?,
            u16::try_from(v.y)?,
            u16::try_from(v.z)?,
        ))
    }
}

impl From<BVec3> for U16Vec3 {
    #[inline]
    fn from(v: BVec3) -> Self {
        Self::new(u16::from(v.x), u16::from(v.y), u16::from(v.z))
    }
}

impl From<BVec3A> for U16Vec3 {
    #[inline]
    fn from(v: BVec3A) -> Self {
        let bool_array: [bool; 3] = v.into();
        Self::new(
            u16::from(bool_array[0]),
            u16::from(bool_array[1]),
            u16::from(bool_array[2]),
        )
    }
}
