use crate::f32::{Vec2, Vec3, Vec4};
use approx::{AbsDiffEq, UlpsEq};

impl AbsDiffEq for Vec2 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let (x1, y1) = self.into();
        let (x2, y2) = other.into();
        x1.abs_diff_eq(&x2, epsilon) && y1.abs_diff_eq(&y2, epsilon)
    }
}

impl UlpsEq for Vec2 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let (x1, y1) = self.into();
        let (x2, y2) = other.into();
        x1.ulps_eq(&x2, epsilon, max_ulps) && y1.ulps_eq(&y2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Vec3 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let (x1, y1, z1) = self.into();
        let (x2, y2, z2) = other.into();
        x1.abs_diff_eq(&x2, epsilon) && y1.abs_diff_eq(&y2, epsilon) && z1.abs_diff_eq(&z2, epsilon)
    }
}

impl UlpsEq for Vec3 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let (x1, y1, z1) = self.into();
        let (x2, y2, z2) = other.into();
        x1.ulps_eq(&x2, epsilon, max_ulps)
            && y1.ulps_eq(&y2, epsilon, max_ulps)
            && z1.ulps_eq(&z2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Vec4 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let (x1, y1, z1, w1) = self.into();
        let (x2, y2, z2, w2) = other.into();
        x1.abs_diff_eq(&x2, epsilon)
            && y1.abs_diff_eq(&y2, epsilon)
            && z1.abs_diff_eq(&z2, epsilon)
            && w1.abs_diff_eq(&w2, epsilon)
    }
}

impl UlpsEq for Vec4 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let (x1, y1, z1, w1) = self.into();
        let (x2, y2, z2, w2) = other.into();
        x1.ulps_eq(&x2, epsilon, max_ulps)
            && y1.ulps_eq(&y2, epsilon, max_ulps)
            && z1.ulps_eq(&z2, epsilon, max_ulps)
            && w1.ulps_eq(&w2, epsilon, max_ulps)
    }
}
