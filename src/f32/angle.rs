use crate::f32::scalar_sin_cos;
use std::ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Angle(f32);

impl Angle {
    #[inline]
    pub fn from_radians(a: f32) -> Angle {
        Angle(a)
    }

    #[inline]
    pub fn from_degrees(a: f32) -> Angle {
        Angle(a.to_radians())
    }

    #[inline]
    pub fn as_radians(self) -> f32 {
        self.0
    }

    #[inline]
    pub fn as_degrees(self) -> f32 {
        self.0.to_degrees()
    }

    #[inline]
    pub fn sin_cos(self) -> (f32, f32) {
        scalar_sin_cos(self.0)
    }
}

impl PartialEq for Angle {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl Div<Angle> for Angle {
    type Output = Angle;
    #[inline]
    fn div(self, rhs: Angle) -> Angle {
        Angle(self.0 / rhs.0)
    }
}

impl DivAssign<Angle> for Angle {
    #[inline]
    fn div_assign(&mut self, rhs: Angle) {
        self.0 /= rhs.0;
    }
}

impl Div<f32> for Angle {
    type Output = Angle;
    #[inline]
    fn div(self, rhs: f32) -> Angle {
        Angle(self.0 / rhs)
    }
}

impl DivAssign<f32> for Angle {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl Mul<Angle> for Angle {
    type Output = Angle;
    #[inline]
    fn mul(self, rhs: Angle) -> Angle {
        Angle(self.0 * rhs.0)
    }
}

impl MulAssign<Angle> for Angle {
    #[inline]
    fn mul_assign(&mut self, rhs: Angle) {
        self.0 *= rhs.0;
    }
}

impl Mul<f32> for Angle {
    type Output = Angle;
    #[inline]
    fn mul(self, rhs: f32) -> Angle {
        Angle(self.0 * rhs)
    }
}

impl MulAssign<f32> for Angle {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl Mul<Angle> for f32 {
    type Output = Angle;
    #[inline]
    fn mul(self, rhs: Angle) -> Angle {
        Angle(self * rhs.0)
    }
}

impl Add for Angle {
    type Output = Angle;
    #[inline]
    fn add(self, rhs: Angle) -> Angle {
        Angle(self.0 + rhs.0)
    }
}

impl AddAssign for Angle {
    #[inline]
    fn add_assign(&mut self, rhs: Angle) {
        self.0 += rhs.0;
    }
}

impl Sub for Angle {
    type Output = Angle;
    #[inline]
    fn sub(self, rhs: Angle) -> Angle {
        Angle(self.0 - rhs.0)
    }
}

impl SubAssign for Angle {
    #[inline]
    fn sub_assign(&mut self, rhs: Angle) {
        self.0 -= rhs.0;
    }
}

impl Neg for Angle {
    type Output = Angle;
    #[inline]
    fn neg(self) -> Angle {
        Angle(-self.0)
    }
}

#[inline]
pub fn rad(a: f32) -> Angle {
    Angle::from_radians(a)
}

#[inline]
pub fn deg(a: f32) -> Angle {
    Angle::from_degrees(a)
}
