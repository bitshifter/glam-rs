use super::{Vec3, Vec3Mask};

#[deprecated(since = "0.7.1", note = "please use `Vec3Mask` instead")]
pub type Vec3b = Vec3Mask;

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    #[inline]
    pub fn sign(self) -> Self {
        let mask = self.cmpge(Self::zero());
        mask.select(Self::splat(1.0), Self::splat(-1.0))
    }

    #[inline]
    pub fn reciprocal(self) -> Self {
        // TODO: Optimize
        Self::one() / self
    }

    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
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
