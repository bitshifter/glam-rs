use super::{Vec3, Vec4};
use std::f32;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quat(f32, f32, f32, f32);

impl Quat {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(x, y, z, w)
    }

    #[inline]
    pub fn identity() -> Self {
        Self(0.0, 0.0, 0.0, 1.0)
    }

    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self(slice[0], slice[1], slice[2], slice[3])
    }

    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        slice[0] = self.0;
        slice[1] = self.1;
        slice[2] = self.2;
        slice[3] = self.3;
    }

    #[inline]
    /// Multiplies two quaternions.
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    /// Multiplication order is as follows:
    /// `local_to_world = local_to_object * object_to_world`
    pub fn mul_quat(self, rhs: Self) -> Self {
        let (x0, y0, z0, w0) = self.into();
        let (x1, y1, z1, w1) = rhs.into();

        let x = (w1 * x0) + (x1 * w0) + (y1 * z0) - (z1 * y0);
        let y = (w1 * y0) - (x1 * z0) + (y1 * w0) + (z1 * x0);
        let z = (w1 * z0) + (x1 * y0) - (y1 * x0) + (z1 * w0);
        let w = (w1 * w0) - (x1 * x0) - (y1 * y0) - (z1 * z0);
        Self::new(x, y, z, w)
    }
}

impl Vec3 {
    #[inline]
    /// Multiplies a quaternion and a 3D vector, rotating it.
    /// Multiplication order is as follows:
    /// `world_position = local_position * local_to_world`
    pub fn rotate_quat(self, rhs: Quat) -> Self {
        let vec_quat: Quat = self.extend(0.0).into();
        let inv_self = rhs.conjugate();
        let res_vec4: Vec4 = inv_self.mul_quat(vec_quat).mul_quat(rhs).into();
        res_vec4.truncate()
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
