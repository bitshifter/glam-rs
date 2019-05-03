use super::{Mat4, Quat, Vec3};
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct TransformSRT {
    pub scale: Vec3,
    pub rotation: Quat,
    pub translation: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub struct TransformRT {
    pub rotation: Quat,
    pub translation: Vec3,
}

impl TransformSRT {
    #[inline]
    pub fn new(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        Self {
            scale,
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn from_transform_tr(tr: &TransformRT, scale: Vec3) -> Self {
        Self {
            scale,
            rotation: tr.rotation,
            translation: tr.translation,
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            scale: Vec3::one(),
            rotation: Quat::identity(),
            translation: Vec3::zero(),
        }
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.conjugate();
        let translation = -(self.translation * rotation);
        let scale = self.scale.reciprocal();
        Self {
            scale,
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let rotation = self.rotation.normalize();
        Self {
            scale: self.scale,
            rotation,
            translation: self.translation,
        }
    }

    #[inline]
    pub fn mul_transform(&self, rhs: &Self) -> Self {
        mul_srt_srt(self, rhs)
    }
}

#[inline]
fn mul_srt_srt(lhs: &TransformSRT, rhs: &TransformSRT) -> TransformSRT {
    // from rtm qvv_mul
    let min_scale = lhs.scale.min(rhs.scale);
    let scale = lhs.scale * rhs.scale;

    if min_scale.cmplt(Vec3::zero()).any() {
        // If negative scale, we go through a matrix
        let lhs_mtx = Mat4::from_transform_srt(lhs);
        let rhs_mtx = Mat4::from_transform_srt(rhs);
        let mut result_mtx = lhs_mtx * rhs_mtx;

        let sign = scale.sign();
        result_mtx.set_x_axis(
            (result_mtx.get_x_axis().truncate().normalize() * sign.dup_x()).extend(0.0),
        );
        result_mtx.set_y_axis(
            (result_mtx.get_y_axis().truncate().normalize() * sign.dup_y()).extend(0.0),
        );
        result_mtx.set_z_axis(
            (result_mtx.get_z_axis().truncate().normalize() * sign.dup_z()).extend(0.0),
        );

        let rotation = Quat::from_rotation_mat4(&result_mtx);
        let translation = result_mtx.get_w_axis().truncate();
        TransformSRT {
            scale,
            rotation,
            translation,
        }
    } else {
        let rotation = lhs.rotation * rhs.rotation;
        let translation = ((lhs.translation * rhs.scale) * rhs.rotation) + rhs.translation;
        TransformSRT {
            scale,
            rotation,
            translation,
        }
    }
}

#[inline]
fn mul_rt_rt(lhs: &TransformRT, rhs: &TransformRT) -> TransformRT {
    let rotation = lhs.rotation * rhs.rotation;
    let translation = (lhs.translation * rhs.rotation) + rhs.translation;
    TransformRT {
        rotation,
        translation,
    }
}

impl TransformRT {
    #[inline]
    pub fn new(rotation: Quat, translation: Vec3) -> Self {
        Self {
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            rotation: Quat::identity(),
            translation: Vec3::zero(),
        }
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.conjugate();
        let translation = -(self.translation * rotation);
        Self {
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let rotation = self.rotation.normalize();
        Self {
            rotation,
            translation: self.translation,
        }
    }

    #[inline]
    pub fn mul_transform(&self, rhs: &Self) -> Self {
        mul_rt_rt(self, rhs)
    }
}

impl Vec3 {
    #[inline]
    pub fn transform_tr(self, rhs: &TransformRT) -> Vec3 {
        (self * rhs.rotation) + rhs.translation
    }

    #[inline]
    pub fn transform_trs(self, rhs: &TransformSRT) -> Vec3 {
        ((self * rhs.scale) * rhs.rotation) + rhs.translation
    }
}

impl AsRef<TransformRT> for TransformSRT {
    #[inline]
    fn as_ref(&self) -> &TransformRT {
        unsafe { &*(self as *const Self as *const TransformRT) }
    }
}

impl AsMut<TransformRT> for TransformSRT {
    #[inline]
    fn as_mut(&mut self) -> &mut TransformRT {
        unsafe { &mut *(self as *mut Self as *mut TransformRT) }
    }
}

impl Mul<TransformRT> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: TransformRT) -> Vec3 {
        self.transform_tr(&rhs)
    }
}

impl Mul<&TransformRT> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &TransformRT) -> Vec3 {
        self.transform_tr(rhs)
    }
}

impl Mul<TransformSRT> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: TransformSRT) -> Vec3 {
        self.transform_trs(&rhs)
    }
}

impl Mul<&TransformSRT> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &TransformSRT) -> Vec3 {
        self.transform_trs(rhs)
    }
}

impl Mul<TransformRT> for TransformRT {
    type Output = TransformRT;
    #[inline]
    fn mul(self, rhs: TransformRT) -> TransformRT {
        mul_rt_rt(&self, &rhs)
    }
}

impl Mul<&TransformRT> for TransformRT {
    type Output = TransformRT;
    #[inline]
    fn mul(self, rhs: &TransformRT) -> TransformRT {
        mul_rt_rt(&self, rhs)
    }
}

impl Mul<TransformSRT> for TransformSRT {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        mul_srt_srt(&self, &rhs)
    }
}

impl Mul<&TransformSRT> for TransformSRT {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        mul_srt_srt(&self, rhs)
    }
}

impl Mul<TransformRT> for TransformSRT {
    type Output = TransformSRT;
    #[inline]
    fn mul(self, rhs: TransformRT) -> Self::Output {
        mul_srt_srt(&self, &rhs.into())
    }
}

impl Mul<&TransformRT> for TransformSRT {
    type Output = TransformSRT;
    #[inline]
    fn mul(self, rhs: &TransformRT) -> Self::Output {
        mul_srt_srt(&self, &rhs.into())
    }
}

impl Mul<TransformSRT> for TransformRT {
    type Output = TransformSRT;
    #[inline]
    fn mul(self, rhs: TransformSRT) -> Self::Output {
        mul_srt_srt(&self.into(), &rhs)
    }
}

impl Mul<&TransformSRT> for TransformRT {
    type Output = TransformSRT;
    #[inline]
    fn mul(self, rhs: &TransformSRT) -> Self::Output {
        mul_srt_srt(&self.into(), rhs)
    }
}

impl PartialEq for TransformRT {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.translation == rhs.translation && self.rotation == rhs.rotation
    }
}

impl PartialEq for TransformSRT {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.translation == rhs.translation
            && self.rotation == rhs.rotation
            && self.scale == rhs.scale
    }
}

impl From<TransformRT> for TransformSRT {
    #[inline]
    fn from(tr: TransformRT) -> Self {
        Self {
            translation: tr.translation,
            rotation: tr.rotation,
            scale: Vec3::one(),
        }
    }
}

impl From<&TransformRT> for TransformSRT {
    #[inline]
    fn from(tr: &TransformRT) -> Self {
        Self {
            translation: tr.translation,
            rotation: tr.rotation,
            scale: Vec3::one(),
        }
    }
}
