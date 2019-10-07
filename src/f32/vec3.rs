use super::{Vec3, Vec3Mask};

#[deprecated(since = "0.7.1", note = "please use `Vec3Mask` instead")]
pub type Vec3b = Vec3Mask;

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    /// Returns a new `Vec4` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    #[inline]
    pub fn sign(self) -> Self {
        let mask = self.cmpge(Self::zero());
        mask.select(Self::splat(1.0), Self::splat(-1.0))
    }

    /// Computes the reciprocal `1.0/n` of each element, returning the
    /// results in a new `Vec3`.
    #[inline]
    pub fn reciprocal(self) -> Self {
        // TODO: Optimize
        Self::one() / self
    }

    /// Performs a linear interpolation between the `Vec3` and `rhs` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to the `Vec3`.  When `s`
    /// is `1.0`, the result will be equal to `rhs`.
    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        glam_assert!(s >= 0.0 && s <= 1.0);
        self + ((rhs - self) * s)
    }

    /// Returns whether the `Vec3` is normalized to length `1.0` or not.
    ///
    /// Uses a precision threshold of `0.00001`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
    }
}

impl AsRef<[f32; 3]> for Vec3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3 as *const [f32; 3]) }
    }
}

impl AsMut<[f32; 3]> for Vec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3 as *mut [f32; 3]) }
    }
}
