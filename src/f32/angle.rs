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

    #[inline]
    pub fn acos(value: f32) -> Angle {
        // from DirectXMath XMScalarAcos
        // Clamp input to [-1,1].
        let nonnegative = value >= 0.0;
        let x = value.abs();
        let mut omx = 1.0 - x;
        if omx < 0.0 {
            omx = 0.0;
        }
        let root = omx.sqrt();

        // 7-degree minimax approximation
        let mut result = ((((((-0.001_262_491_1 * x + 0.006_670_09) * x - 0.017_088_126) * x
            + 0.030_891_88)
            * x
            - 0.050_174_303)
            * x
            + 0.088_978_99)
            * x
            - 0.214_598_8)
            * x
            + 1.570_796_3;
        result *= root;

        // acos(x) = pi - acos(-x) when x < 0
        Angle::from_radians(if nonnegative {
            result
        } else {
            std::f32::consts::PI - result
        })
    }
}

impl PartialEq for Angle {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
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
