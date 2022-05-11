
    
        
    
    



    
    
    
    
        
            
            
        
        
        
        
        
    



/// Creates a 2-dimensional vector.
#[inline(always)]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}


/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "cuda", repr(C, align(8)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct Vec2(pub(crate) crate::XY::<f32>);

impl Vec2 {
    /// All zeroes.
    pub const ZERO: Self = Self(crate::XY::<f32>::ZERO);

    /// All ones.
    pub const ONE: Self = Self(crate::XY::<f32>::ONE);


    /// `[1, 0]`: a unit-length vector pointing along the positive X axis.
    pub const X: Self = Self(crate::XY::<f32>::X);

    /// `[0, 1]`: a unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self(crate::XY::<f32>::Y);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Creates a new vector.
    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector2::new(x, y))
    }

    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline(always)]
    pub fn extend(self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    /// `[x, y]`
    #[inline(always)]
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }


    /// Creates a vector with all elements set to `v`.
    #[inline(always)]
    pub fn splat(v: f32) -> Self {
        Self(crate::XY::<f32>::splat(v))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline(always)]
    pub fn select(mask: BVec2, if_true: Self, if_false: Self) -> Self {
        Self(crate::XY::<f32>::select(mask.0, if_true.0, if_false.0))
    }

    /// Computes the dot product of `self` and `other`.
    #[inline(always)]
    pub fn dot(self, other: Self) -> f32 {
        FloatVector2::dot(self.0, other.0)
    }

    /// Returns a vector containing the minimum values for each element of `self` and `other`.
    ///
    /// In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// Returns a vector containing the maximum values for each element of `self` and `other`.
    ///
    /// In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self(FloatVector2::clamp(self.0, min.0, max.0))
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline(always)]
    pub fn min_element(self) -> f32 {
        FloatVector2::min_element(self.0)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline(always)]
    pub fn max_element(self) -> f32 {
        FloatVector2::max_element(self.0)
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpeq(self, other: Self) -> BVec2 {
        BVec2(self.0.cmpeq(other.0))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpne(self, other: Self) -> BVec2 {
        BVec2(self.0.cmpne(other.0))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpge(self, other: Self) -> BVec2 {
        BVec2(self.0.cmpge(other.0))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpgt(self, other: Self) -> BVec2 {
        BVec2(self.0.cmpgt(other.0))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmple(self, other: Self) -> BVec2 {
        BVec2(self.0.cmple(other.0))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmplt(self, other: Self) -> BVec2 {
        BVec2(self.0.cmplt(other.0))
    }

    /// Creates a vector from the first N values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn from_slice(slice: &[f32]) -> Self {
        Self(FloatVector2::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        FloatVector2::write_to_slice_unaligned(self.0, slice)
    }


    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(FloatVector2::abs(self.0))
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline(always)]
    pub fn signum(self) -> Self {
        Self(FloatVector2::signum(self.0))
    }



    /// All NAN.
    pub const NAN: Self = Self(<crate::XY::<f32> as crate::core::traits::scalar::NanConstEx>::NAN);

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline(always)]
    pub fn is_finite(self) -> bool {
        FloatVector2::is_finite(self.0)
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline(always)]
    pub fn is_nan(self) -> bool {
        FloatVector2::is_nan(self.0)
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline(always)]
    pub fn is_nan_mask(self) -> BVec2 {
        BVec2(FloatVector2::is_nan_mask(self.0))
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline(always)]
    pub fn length(self) -> f32 {
        FloatVector2::length(self.0)
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline(always)]
    pub fn length_squared(self) -> f32 {
        FloatVector2::length_squared(self.0)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline(always)]
    pub fn length_recip(self) -> f32 {
        FloatVector2::length_recip(self.0)
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline(always)]
    pub fn normalize(self) -> Self {
        Self(FloatVector2::normalize(self.0))
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero`].
    #[must_use]
    #[inline]
    pub fn try_normalize(self) -> Option<Self> {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            Some(self * rcp)
        } else {
            None
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be zero.
    ///
    /// See also [`Self::try_normalize`].
    #[must_use]
    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            self * rcp
        } else {
            Self::ZERO
        }
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline(always)]
    pub fn is_normalized(self) -> bool {
        FloatVector2::is_normalized(self.0)
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto(self, other: Self) -> Self {
        let other_len_sq_rcp = other.dot(other).recip();
        glam_assert!(other_len_sq_rcp.is_finite());
        other * self.dot(other) * other_len_sq_rcp
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `other`, in other words the result of `self - self.project_onto(other)`.
    ///
    /// `other` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `other` has a length of zero when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from(self, other: Self) -> Self {
        self - self.project_onto(other)
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto_normalized(self, other: Self) -> Self {
        glam_assert!(other.is_normalized());
        other * self.dot(other)
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `other`, in other words the result of `self - self.project_onto(other)`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from_normalized(self, other: Self) -> Self {
        self - self.project_onto_normalized(other)
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline(always)]
    pub fn round(self) -> Self {
        Self(FloatVector2::round(self.0))
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline(always)]
    pub fn floor(self) -> Self {
        Self(FloatVector2::floor(self.0))
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline(always)]
    pub fn ceil(self) -> Self {
        Self(FloatVector2::ceil(self.0))
    }

    /// Returns a vector containing the fractional part of the vector, e.g. `self -
    /// self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline(always)]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline(always)]
    pub fn exp(self) -> Self {
        Self(FloatVector2::exp(self.0))
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline(always)]
    pub fn powf(self, n: f32) -> Self {
        Self(FloatVector2::powf(self.0, n))
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline(always)]
    pub fn recip(self) -> Self {
        Self(FloatVector2::recip(self.0))
    }

    /// Performs a linear interpolation between `self` and `other` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `other`. When `s` is outside of range [0,1], the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    pub fn lerp(self, other: Self, s: f32) -> Self {
        self + ((other - self) * s)
    }

    /// Returns true if the absolute difference of all elements between `self` and `other` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline(always)]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        FloatVector2::abs_diff_eq(self.0, other.0, max_abs_diff)
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        glam_assert!(min <= max);
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no more than `max`
    pub fn clamp_length_max(self, max: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`
    pub fn clamp_length_min(self, min: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else {
            self
        }
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated fma CPU instruction. However, this is not always true,
    /// and will be heavily dependant on designing algorithms with specific target hardware in
    /// mind.
    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self(FloatVector2::mul_add(self.0, a.0, b.0))
    }


    /// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    /// conjunction with the `rotate` method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will
    /// create the vector [-1, 0] and rotate `Vec2::Y` around it returning `-Vec2::Y`.
    #[inline(always)]
    pub fn from_angle(angle: f32) -> Self {
        Self(FloatVector2::from_angle(angle))
    }

    /// Returns the angle (in radians) between `self` and `other`.
    ///
    /// The input vectors do not need to be unit length however they must be non-zero.
    #[inline(always)]
    pub fn angle_between(self, other: Self) -> f32 {
        self.0.angle_between(other.0)
    }




    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline(always)]
    pub fn perp(self) -> Self {
        Self(self.0.perp())
    }

    /// The perpendicular dot product of `self` and `other`.
    /// Also known as the wedge product, 2D cross product, and determinant.
    #[doc(alias = "wedge")]
    #[doc(alias = "cross")]
    #[doc(alias = "determinant")]
    #[inline(always)]
    pub fn perp_dot(self, other: Self) -> f32 {
        self.0.perp_dot(other.0)
    }

    /// Returns `other` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    #[must_use]
    #[inline(always)]
    pub fn rotate(self, other: Self) -> Self {
        Self(self.0.rotate(other.0))
    }




    
    /// Casts all elements of `self` to `f64`.
    #[inline(always)]
    pub fn as_dvec2(&self) -> DVec2 {
        DVec2::new(self.x as f64, self.y as f64)
    }
    


    
    /// Casts all elements of `self` to `i32`.
    #[inline(always)]
    pub fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x as i32, self.y as i32)
    }
    


    
    /// Casts all elements of `self` to `u32`.
    #[inline(always)]
    pub fn as_uvec2(&self) -> UVec2 {
        UVec2::new(self.x as u32, self.y as u32)
    }
    

}

impl Default for Vec2 {
    #[inline(always)]
    fn default() -> Self {
        Self(crate::XY::<f32>::ZERO)
    }
}

impl PartialEq for Vec2 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl From<Vec2> for crate::XY::<f32> {
    #[inline(always)]
    fn from(t: Vec2) -> Self {
        t.0
    }
}

impl From<crate::XY::<f32>> for Vec2 {
    #[inline(always)]
    fn from(t: crate::XY::<f32>) -> Self {
        Self(t)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: Vec2) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<Vec2> for Vec2 {
    #[inline(always)]
    fn div_assign(&mut self, other: Vec2) {
        self.0 = self.0.div(other.0)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: f32) -> Self {
        Self(self.0.div_scalar(other))
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline(always)]
    fn div_assign(&mut self, other: f32) {
        self.0 = self.0.div_scalar(other)
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn div(self, other: Vec2) -> Vec2 {
        Vec2(crate::XY::<f32>::splat(self).div(other.0))
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Vec2) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<Vec2> for Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Vec2) {
        self.0 = self.0.mul(other.0)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: f32) -> Self {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, other: f32) {
        self.0 = self.0.mul_scalar(other)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2(crate::XY::<f32>::splat(self).mul(other.0))
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Vec2) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign<Vec2> for Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, other: Vec2) {
        self.0 = self.0.add(other.0)
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: f32) -> Self {
        Self(self.0.add_scalar(other))
    }
}

impl AddAssign<f32> for Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, other: f32) {
        self.0 = self.0.add_scalar(other)
    }
}

impl Add<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn add(self, other: Vec2) -> Vec2 {
        Vec2(crate::XY::<f32>::splat(self).add(other.0))
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Vec2) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign<Vec2> for Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Vec2) {
        self.0 = self.0.sub(other.0)
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: f32) -> Self {
        Self(self.0.sub_scalar(other))
    }
}

impl SubAssign<f32> for Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, other: f32) {
        self.0 = self.0.sub_scalar(other)
    }
}

impl Sub<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2(crate::XY::<f32>::splat(self).sub(other.0))
    }
}

impl Rem<Vec2> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: Vec2) -> Self {
        Self(self.0.rem(other.0))
    }
}

impl RemAssign<Vec2> for Vec2 {
    #[inline(always)]
    fn rem_assign(&mut self, other: Vec2) {
        self.0 = self.0.rem(other.0)
    }
}

impl Rem<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: f32) -> Self {
        Self(self.0.rem_scalar(other))
    }
}

impl RemAssign<f32> for Vec2 {
    #[inline(always)]
    fn rem_assign(&mut self, other: f32) {
        self.0 = self.0.rem_scalar(other)
    }
}

impl Rem<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn rem(self, other: Vec2) -> Vec2 {
        Vec2(crate::XY::<f32>::splat(self).rem(other.0))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f32; 2]> for Vec2 {
    #[inline(always)]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Vec2 as *const [f32; 2]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f32; 2]> for Vec2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Vec2 as *mut [f32; 2]) }
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline(always)]
    fn from(a: [f32; 2]) -> Self {
        Self(FloatVector2::from_array(a))
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline(always)]
    fn from(v: Vec2) -> Self {
        v.into_array()
    }
}

impl<'a> Sum<&'a Self> for Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}


impl Index<usize> for Vec2 {
    type Output = f32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}
#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for Vec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Vec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline(always)]
    fn from(t: (f32, f32)) -> Self {
        Self($inner::from_tuple(t))
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline(always)]
    fn from(v: Vec2) -> Self {
        v.0.into_tuple()
    }
}

impl Deref for Vec2 {
    type Target = crate::XY<f32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref_xy()
    }
}

impl DerefMut for Vec2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_xy()
    }
}

