use crate::f32::scalar_sin_cos;

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

#[inline]
pub fn rad(a: f32) -> Angle {
    Angle::from_radians(a)
}

#[inline]
pub fn deg(a: f32) -> Angle {
    Angle::from_degrees(a)
}

