#![cfg(feature = "unreal-abi-compat")]

use crate::{Quat, UNREAL_EULER_ROT};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rotator {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[inline]
#[must_use]
pub const fn rotator(pitch: f32, yaw: f32, roll: f32) -> Rotator {
    Rotator { pitch, yaw, roll }
}

impl Rotator {
    pub const fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        rotator(pitch, yaw, roll)
    }

    pub fn from_euler(a: f32, b: f32, c: f32) -> Self {
        Self::new(f32::to_degrees(b), f32::to_degrees(c), f32::to_degrees(a))
    }

    pub fn from_quat(q: Quat) -> Self {
        let (a, b, c) = q.to_euler(UNREAL_EULER_ROT);
        Self::new(a, b, c)
    }

    pub fn to_euler(&self) -> (f32, f32, f32) {
        (
            f32::to_radians(self.roll),
            f32::to_radians(self.pitch),
            f32::to_radians(self.yaw),
        )
    }

    pub fn to_quat(&self) -> Quat {
        let (a, b, c) = self.to_euler();
        Quat::from_euler(UNREAL_EULER_ROT, a, b, c)
    }
}

impl From<Rotator> for Quat {
    fn from(r: Rotator) -> Self {
        r.to_quat()
    }
}

impl From<Quat> for Rotator {
    fn from(q: Quat) -> Self {
        Self::from_quat(q)
    }
}

impl Quat {
    pub fn from_rotator(rot: Rotator) -> Self {
        rot.to_quat()
    }

    pub fn to_rotator(&self) -> Rotator {
        Rotator::from_quat(*self)
    }
}
