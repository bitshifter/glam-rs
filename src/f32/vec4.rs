use super::{Vec4, Vec4Mask};

#[deprecated(since = "0.7.1", note = "please use `Vec3Mask` instead")]
pub type Vec4b = Vec4Mask;

#[inline]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

impl Vec4 {
    /// Returns a new `Vec4` with `1.0` for elements that are greater than or
    /// equal to zero in the original and `-1.0` for elements that are negative.
    #[inline]
    pub fn sign(self) -> Self {
        let mask = self.cmpge(Self::zero());
        mask.select(Self::splat(1.0), Self::splat(-1.0))
    }

    /// Computes the reciprocal `1.0/n` of each element, returning the
    /// results in a new `Vec4`.
    #[inline]
    pub fn reciprocal(self) -> Self {
        // TODO: Optimize
        Self::one() / self
    }

    /// Performs a linear interpolation between the `Vec4` and `rhs` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to the `Vec4`.  When `s`
    /// is `1.0`, the result will be equal to `rhs`.
    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        glam_assert!(s >= 0.0 && s <= 1.0);
        self + ((rhs - self) * s)
    }

    /// Returns whether the `Vec4` is normalized to length `1.0` or not.
    ///
    /// Uses a precision threshold of `0.00001`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 4]) }
    }
}
