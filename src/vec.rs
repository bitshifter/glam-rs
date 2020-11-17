macro_rules! impl_vec_common_methods {
    ($t:ty, $vecn:ident, $mask:ident, $inner:ident, $trait:ident) => {
        impl $vecn {
            /// Creates a `$vec4` with all elements set to `0.0`.
            #[inline]
            pub const fn zero() -> Self {
                Self($inner::ZERO)
            }

            /// Creates a `$vecn` with all elements set to `1.0`.
            #[inline]
            pub const fn one() -> Self {
                Self($inner::ONE)
            }

            /// Creates a `$vecn` with all elements set to `v`.
            #[inline]
            pub fn splat(v: $t) -> Self {
                Self($inner::splat(v))
            }

            /// Creates a `$vecn` from the elements in `if_true` and `if_false`, selecting which to use for
            /// each element of `self`.
            ///
            /// A true element in the mask uses the corresponding element from `if_true`, and false uses
            /// the element from `if_false`.
            #[inline]
            pub fn select(mask: $mask, if_true: $vecn, if_false: $vecn) -> $vecn {
                Self($inner::select(mask.0, if_true.0, if_false.0))
            }

            /// Computes the 4D dot product of `self` and `other`.
            #[inline]
            pub fn dot(self, other: Self) -> $t {
                $trait::dot(self.0, other.0)
            }

            /// Returns the vertical minimum of `self` and `other`.
            ///
            /// In other words, this computes
            /// `[x: min(x1, x2), y: min(y1, y2), z: min(z1, z2), w: min(w1, w2)]`,
            /// taking the minimum of each element individually.
            #[inline]
            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }

            /// Returns the vertical maximum of `self` and `other`.
            ///
            /// In other words, this computes
            /// `[x: max(x1, x2), y: max(y1, y2), z: max(z1, z2), w: max(w1, w2)]`,
            /// taking the maximum of each element individually.
            #[inline]
            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            /// Returns the horizontal minimum of `self`'s elements.
            ///
            /// In other words, this computes `min(x, y, z, w)`.
            #[inline]
            pub fn min_element(self) -> $t {
                $trait::min_element(self.0)
            }

            /// Returns the horizontal maximum of `self`'s elements.
            ///
            /// In other words, this computes `max(x, y, z, w)`.
            #[inline]
            pub fn max_element(self) -> $t {
                $trait::max_element(self.0)
            }

            /// Performs a vertical `==` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
            #[inline]
            pub fn cmpeq(self, other: Self) -> $mask {
                $mask(self.0.cmpeq(other.0))
            }

            /// Performs a vertical `!=` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
            #[inline]
            pub fn cmpne(self, other: Self) -> $mask {
                $mask(self.0.cmpne(other.0))
            }

            /// Performs a vertical `>=` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
            #[inline]
            pub fn cmpge(self, other: Self) -> $mask {
                $mask(self.0.cmpge(other.0))
            }

            /// Performs a vertical `>` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
            #[inline]
            pub fn cmpgt(self, other: Self) -> $mask {
                $mask(self.0.cmpgt(other.0))
            }

            /// Performs a vertical `<=` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
            #[inline]
            pub fn cmple(self, other: Self) -> $mask {
                $mask(self.0.cmple(other.0))
            }

            /// Performs a vertical `<` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
            #[inline]
            pub fn cmplt(self, other: Self) -> $mask {
                $mask(self.0.cmplt(other.0))
            }

            /// Creates a `$vecn` from the first four values in `slice`.
            ///
            /// # Panics
            ///
            /// Panics if `slice` is less than four elements long.
            #[inline]
            pub fn from_slice_unaligned(slice: &[$t]) -> Self {
                Self($trait::from_slice_unaligned(slice))
            }

            /// Writes the elements of `self` to the first four elements in `slice`.
            ///
            /// # Panics
            ///
            /// Panics if `slice` is less than four elements long.
            #[inline]
            pub fn write_to_slice_unaligned(self, slice: &mut [$t]) {
                $trait::write_to_slice_unaligned(self.0, slice)
            }

            /// Per element multiplication/addition of the three inputs: b + (self * a)
            #[inline]
            #[allow(dead_code)]
            pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
                Self(self.0.mul_add(a.0, b.0))
            }
        }
    };
}

macro_rules! impl_vec_signed_methods {
    ($vecn:ident, $trait:ident) => {
        impl $vecn {
            /// Returns a `$vec4` containing the absolute value of each element of `self`.
            #[inline]
            pub fn abs(self) -> Self {
                Self($trait::abs(self.0))
            }

            /// Returns a `$vec4` with elements representing the sign of `self`.
            ///
            /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
            /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
            /// - `NAN` if the number is `NAN`
            #[inline]
            pub fn signum(self) -> Self {
                Self($trait::signum(self.0))
            }
        }

        impl Neg for $vecn {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                Self(self.0.neg())
            }
        }
    };
}

macro_rules! impl_vec_float_methods {
    ($t:ty, $vecn:ident, $mask:ident, $inner:ident, $trait:ident) => {
        impl $vecn {
            /// Returns `true` if, and only if, all elements are finite.
            /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
            #[inline]
            pub fn is_finite(self) -> bool {
                $trait::is_finite(self.0)
            }

            /// Returns `true` if any elements are `NaN`.
            #[inline]
            pub fn is_nan(self) -> bool {
                $trait::is_nan(self.0)
            }

            /// Performs `is_nan` on each element of self, returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
            #[inline]
            pub fn is_nan_mask(self) -> $mask {
                $mask($trait::is_nan_mask(self.0))
            }

            /// Computes the 4D length of `self`.
            #[inline]
            pub fn length(self) -> $t {
                $trait::length(self.0)
            }

            /// Computes the squared length of `self`.
            ///
            /// This is generally faster than `$vec3::length()` as it avoids a square
            /// root operation.
            #[inline]
            pub fn length_squared(self) -> $t {
                $trait::length_squared(self.0)
            }

            /// Computes `1.0 / $vec4::length()`.
            ///
            /// For valid results, `self` must _not_ be of length zero.
            #[inline]
            pub fn length_recip(self) -> $t {
                $trait::length_recip(self.0)
            }

            /// Computes the Euclidean distance between two points in space.
            #[inline]
            pub fn distance(self, other: Self) -> $t {
                (self - other).length()
            }

            /// Compute the squared euclidean distance between two points in space.
            #[inline]
            pub fn distance_squared(self, other: Self) -> $t {
                (self - other).length_squared()
            }

            /// Returns `self` normalized to length 1.0.
            ///
            /// For valid results, `self` must _not_ be of length zero.
            #[inline]
            pub fn normalize(self) -> Self {
                Self($trait::normalize(self.0))
            }

            /// Returns whether `self` is length `1.0` or not.
            ///
            /// Uses a precision threshold of `1e-6`.
            #[inline]
            pub fn is_normalized(self) -> bool {
                $trait::is_normalized(self.0)
            }

            /// Returns a `$vec4` containing the nearest integer to a number for each element of `self`.
            /// Round half-way cases away from 0.0.
            #[inline]
            pub fn round(self) -> Self {
                Self($trait::round(self.0))
            }

            /// Returns a `$vec4` containing the largest integer less than or equal to a number for each
            /// element of `self`.
            #[inline]
            pub fn floor(self) -> Self {
                Self($trait::floor(self.0))
            }

            /// Returns a `$vec4` containing the smallest integer greater than or equal to a number for each
            /// element of `self`.
            #[inline]
            pub fn ceil(self) -> Self {
                Self($trait::ceil(self.0))
            }

            /// Returns a `$vec4` containing `e^self` (the exponential function) for each element of `self`.
            #[inline]
            pub fn exp(self) -> Self {
                Self($trait::exp(self.0))
            }

            /// Returns a `$vec4` containing each element of `self` raised to the power of `n`.
            #[inline]
            pub fn powf(self, n: $t) -> Self {
                Self($trait::powf(self.0, n))
            }

            /// Returns a `$vec4` containing the reciprocal `1.0/n` of each element of `self`.
            #[inline]
            pub fn recip(self) -> Self {
                Self($trait::recip(self.0))
            }

            /// Performs a linear interpolation between `self` and `other` based on
            /// the value `s`.
            ///
            /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
            /// is `1.0`, the result will be equal to `other`.
            #[inline]
            pub fn lerp(self, other: Self, s: $t) -> Self {
                self + ((other - self) * s)
            }

            /// Returns true if the absolute difference of all elements between `self`
            /// and `other` is less than or equal to `max_abs_diff`.
            ///
            /// This can be used to compare if two `$vec4`'s contain similar elements. It
            /// works best when comparing with a known value. The `max_abs_diff` that
            /// should be used used depends on the values being compared against.
            ///
            /// For more on floating point comparisons see
            /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
            #[inline]
            pub fn abs_diff_eq(self, other: Self, max_abs_diff: $t) -> bool {
                $trait::abs_diff_eq(self.0, other.0, max_abs_diff)
            }
        }
    };
}

macro_rules! impl_vec_common_traits {
    ($t:ty, $size:literal, $vecn:ident, $inner:ident, $trait:ident) => {
        impl Default for $vecn {
            #[inline]
            fn default() -> Self {
                Self($inner::ZERO)
            }
        }

        impl PartialEq for $vecn {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.cmpeq(*other).all()
            }
        }

        impl PartialOrd for $vecn {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.as_ref().partial_cmp(other.as_ref())
            }
        }

        impl From<$vecn> for $inner {
            #[inline(always)]
            fn from(t: $vecn) -> Self {
                t.0
            }
        }

        impl From<$inner> for $vecn {
            #[inline(always)]
            fn from(t: $inner) -> Self {
                Self(t)
            }
        }

        impl Div<$vecn> for $vecn {
            type Output = Self;
            #[inline]
            fn div(self, other: $vecn) -> Self {
                Self(self.0.div(other.0))
            }
        }

        impl DivAssign<$vecn> for $vecn {
            #[inline]
            fn div_assign(&mut self, other: $vecn) {
                self.0 = self.0.div(other.0)
            }
        }

        impl Div<$t> for $vecn {
            type Output = Self;
            #[inline]
            fn div(self, other: $t) -> Self {
                Self(self.0.div_scalar(other))
            }
        }

        impl DivAssign<$t> for $vecn {
            #[inline]
            fn div_assign(&mut self, other: $t) {
                self.0 = self.0.div_scalar(other)
            }
        }

        impl Div<$vecn> for $t {
            type Output = $vecn;
            #[inline]
            fn div(self, other: $vecn) -> $vecn {
                $vecn($inner::splat(self).div(other.0))
            }
        }

        impl Mul<$vecn> for $vecn {
            type Output = Self;
            #[inline]
            fn mul(self, other: $vecn) -> Self {
                Self(self.0.mul(other.0))
            }
        }

        impl MulAssign<$vecn> for $vecn {
            #[inline]
            fn mul_assign(&mut self, other: $vecn) {
                self.0 = self.0.mul(other.0)
            }
        }

        impl Mul<$t> for $vecn {
            type Output = Self;
            #[inline]
            fn mul(self, other: $t) -> Self {
                Self(self.0.mul_scalar(other))
            }
        }

        impl MulAssign<$t> for $vecn {
            #[inline]
            fn mul_assign(&mut self, other: $t) {
                self.0 = self.0.mul_scalar(other)
            }
        }

        impl Mul<$vecn> for $t {
            type Output = $vecn;
            #[inline]
            fn mul(self, other: $vecn) -> $vecn {
                $vecn($inner::splat(self).mul(other.0))
            }
        }

        impl Add for $vecn {
            type Output = Self;
            #[inline]
            fn add(self, other: Self) -> Self {
                Self(self.0.add(other.0))
            }
        }

        impl AddAssign for $vecn {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0.add(other.0)
            }
        }

        impl Sub for $vecn {
            type Output = Self;
            #[inline]
            fn sub(self, other: $vecn) -> Self {
                Self(self.0.sub(other.0))
            }
        }

        impl SubAssign for $vecn {
            #[inline]
            fn sub_assign(&mut self, other: $vecn) {
                self.0 = self.0.sub(other.0)
            }
        }

        impl AsRef<[$t; $size]> for $vecn {
            #[inline(always)]
            fn as_ref(&self) -> &[$t; $size] {
                unsafe { &*(self as *const $vecn as *const [$t; $size]) }
            }
        }

        impl From<[$t; $size]> for $vecn {
            #[inline(always)]
            fn from(a: [$t; $size]) -> Self {
                Self($trait::from_array(a))
            }
        }

        impl AsMut<[$t; $size]> for $vecn {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut [$t; $size] {
                unsafe { &mut *(self as *mut $vecn as *mut [$t; $size]) }
            }
        }

        impl From<$vecn> for [$t; $size] {
            #[inline(always)]
            fn from(v: $vecn) -> Self {
                v.into_array()
            }
        }

        impl Index<usize> for $vecn {
            type Output = $t;
            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                &self.as_ref()[index]
            }
        }

        impl IndexMut<usize> for $vecn {
            #[inline(always)]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_mut()[index]
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Sum<&'a Self> for $vecn {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::zero(), |a, &b| Self::add(a, b))
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Product<&'a Self> for $vecn {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::one(), |a, &b| Self::mul(a, b))
            }
        }
    };
}
