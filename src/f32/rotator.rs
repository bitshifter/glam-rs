#![cfg(feature = "unreal-abi-compat")]

use crate::{Quat, UNREAL_EULER_ROT};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
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

    pub const fn from_euler(a: f32, b: f32, c: f32) -> Self {
        Self::new(b, c, a)
    }

    pub const fn to_euler(&self) -> (f32, f32, f32) {
        (self.roll, self.pitch, self.yaw)
    }
}

impl From<Rotator> for Quat {
    fn from(r: Rotator) -> Self {
        let (a, b, c) = r.to_euler();
        Quat::from_euler(UNREAL_EULER_ROT, a, b, c)
    }
}

impl From<Quat> for Rotator {
    fn from(q: Quat) -> Self {
        let euler = q.to_euler(UNREAL_EULER_ROT);
        Rotator::from_euler(euler.0, euler.1, euler.2)
    }
}
