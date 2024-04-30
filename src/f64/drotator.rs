#![cfg(feature = "unreal-abi-compat")]

use crate::{DQuat, UNREAL_EULER_ROT};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DRotator {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

#[inline]
#[must_use]
pub const fn drotator(pitch: f64, yaw: f64, roll: f64) -> DRotator {
    DRotator { pitch, yaw, roll }
}

impl DRotator {
    pub const fn new(pitch: f64, yaw: f64, roll: f64) -> Self {
        drotator(pitch, yaw, roll)
    }

    pub fn from_euler(a: f64, b: f64, c: f64) -> Self {
        Self::new(f64::to_degrees(b), f64::to_degrees(c), f64::to_degrees(a))
    }

    pub fn to_euler(&self) -> (f64, f64, f64) {
        (
            f64::to_radians(self.roll),
            f64::to_radians(self.pitch),
            f64::to_radians(self.yaw),
        )
    }
}

impl From<DRotator> for DQuat {
    fn from(r: DRotator) -> Self {
        let (a, b, c) = r.to_euler();
        DQuat::from_euler(UNREAL_EULER_ROT, a, b, c)
    }
}

impl From<DQuat> for DRotator {
    fn from(q: DQuat) -> Self {
        let euler = q.to_euler(UNREAL_EULER_ROT);
        DRotator::from_euler(euler.0, euler.1, euler.2)
    }
}
