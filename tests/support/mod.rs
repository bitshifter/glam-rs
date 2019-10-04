/// Helper function for migrating away from `glam::angle::deg`.
#[allow(dead_code)]
#[inline]
pub fn deg(angle: f32) -> f32 {
    angle.to_radians()
}

/// Helper function for migrating away from `glam::angle::rad`.
#[allow(dead_code)]
#[inline]
pub fn rad(angle: f32) -> f32 {
    angle
}
