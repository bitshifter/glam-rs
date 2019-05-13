use super::Vec2;

#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

impl Vec2 {
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

impl AsRef<[f32; 2]> for Vec2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Vec2 as *const [f32; 2]) }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Vec2 as *mut [f32; 2]) }
    }
}
