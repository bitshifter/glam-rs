use super::Vec4;
use std::f32;

#[derive(Clone, Copy)]
#[repr(align(16), C)]
pub struct Quat(f32, f32, f32, f32);

impl Quat {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quat {
        Quat(x, y, z, w)
    }

    #[inline]
    pub fn identity() -> Quat {
        Quat(0.0, 0.0, 0.0, 1.0)
    }

    #[inline]
    pub(super) fn get_w(self) -> f32 {
        self.3
    }

    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Quat {
        Quat(slice[0], slice[1], slice[2], slice[3])
    }

    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        slice[0] = self.0;
        slice[1] = self.1;
        slice[2] = self.2;
        slice[3] = self.3;
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(v: Vec4) -> Self {
        v.as_ref().into()
    }
}

impl From<&Vec4> for Quat {
    #[inline]
    fn from(v: &Vec4) -> Self {
        v.as_ref().into()
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(q: Quat) -> Self {
        q.as_ref().into()
    }
}

impl From<&Quat> for Vec4 {
    #[inline]
    fn from(q: &Quat) -> Self {
        q.as_ref().into()
    }
}

impl From<Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(q: Quat) -> Self {
        (q.0, q.1, q.2, q.3)
    }
}

impl From<&Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(q: &Quat) -> Self {
        (q.0, q.1, q.2, q.3)
    }
}

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Quat(a[0], a[1], a[2], a[3])
    }
}

impl From<&[f32; 4]> for Quat {
    #[inline]
    fn from(a: &[f32; 4]) -> Self {
        Quat(a[0], a[1], a[2], a[3])
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(q: Quat) -> Self {
        [q.0, q.1, q.2, q.3]
    }
}

impl From<&Quat> for [f32; 4] {
    #[inline]
    fn from(q: &Quat) -> Self {
        [q.0, q.1, q.2, q.3]
    }
}
