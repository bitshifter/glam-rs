use super::{Mat4, Quat, Vec3};
use std::ops::Mul;

pub struct TransformTRS {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

pub struct TransformTR {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl TransformTRS {
    #[inline]
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    #[inline]
    pub fn from_transform_tr(tr: TransformTR, scale: Vec3) -> Self {
        Self {
            translation: tr.translation,
            rotation: tr.rotation,
            scale,
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            translation: Vec3::zero(),
            rotation: Quat::identity(),
            scale: Vec3::one(),
        }
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.conjugate();
        let translation = -(self.translation * rotation);
        let scale = self.scale.reciprocal();
        Self {
            translation,
            rotation,
            scale,
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let rotation = self.rotation.normalize();
        Self {
            translation: self.translation,
            rotation,
            scale: self.scale,
        }
    }

    #[inline]
    pub fn mul_transform(&self, rhs: &Self) -> Self {
        mul_trs_trs(self, rhs)
    }
}

#[inline]
fn mul_trs_trs(lhs: &TransformTRS, rhs: &TransformTRS) -> TransformTRS {
    // from rtm qvv_mul
    let min_scale = lhs.scale.min(rhs.scale);
    let scale = lhs.scale * rhs.scale;

    if min_scale.cmplt(Vec3::zero()).any() {
        // If negative scale, we go through a matrix
        let lhs_mtx = Mat4::from_transform_trs(lhs);
        let rhs_mtx = Mat4::from_transform_trs(rhs);
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
        TransformTRS {
            translation,
            rotation,
            scale,
        }
    } else {
        let rotation = lhs.rotation * rhs.rotation;
        let translation = ((lhs.translation * rhs.scale) * rhs.rotation) + rhs.translation;
        TransformTRS {
            translation,
            rotation,
            scale,
        }
    }
}

#[inline]
fn mul_tr_tr(lhs: &TransformTR, rhs: &TransformTR) -> TransformTR {
    let rotation = lhs.rotation * rhs.rotation;
    let translation = (lhs.translation * rhs.rotation) + rhs.translation;
    TransformTR {
        translation,
        rotation,
    }
}

impl TransformTR {
    #[inline]
    pub fn new(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            translation: Vec3::zero(),
            rotation: Quat::identity(),
        }
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.conjugate();
        let translation = -(self.translation * rotation);
        Self {
            translation,
            rotation,
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let rotation = self.rotation.normalize();
        Self {
            translation: self.translation,
            rotation,
        }
    }

    #[inline]
    pub fn mul_transform(&self, rhs: &Self) -> Self {
        mul_tr_tr(self, rhs)
    }
}

impl Vec3 {
    #[inline]
    pub fn transform_tr(self, rhs: &TransformTR) -> Vec3 {
        (self * rhs.rotation) + rhs.translation
    }

    #[inline]
    pub fn transform_trs(self, rhs: &TransformTRS) -> Vec3 {
        (self * rhs.rotation) + rhs.translation
    }
}

impl AsRef<TransformTR> for TransformTRS {
    #[inline]
    fn as_ref(&self) -> &TransformTR {
        unsafe { &*(self as *const Self as *const TransformTR) }
    }
}

impl AsMut<TransformTR> for TransformTRS {
    #[inline]
    fn as_mut(&mut self) -> &mut TransformTR {
        unsafe { &mut *(self as *mut Self as *mut TransformTR) }
    }
}

impl Mul<TransformTR> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: TransformTR) -> Vec3 {
        self.transform_tr(&rhs)
    }
}

impl Mul<&TransformTR> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &TransformTR) -> Vec3 {
        self.transform_tr(rhs)
    }
}

impl Mul<TransformTRS> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: TransformTRS) -> Vec3 {
        self.transform_trs(&rhs)
    }
}

impl Mul<&TransformTRS> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &TransformTRS) -> Vec3 {
        self.transform_trs(rhs)
    }
}

impl Mul<TransformTR> for TransformTR {
    type Output = TransformTR;
    #[inline]
    fn mul(self, rhs: TransformTR) -> TransformTR {
        mul_tr_tr(&self, &rhs)
    }
}

impl Mul<&TransformTR> for TransformTR {
    type Output = TransformTR;
    #[inline]
    fn mul(self, rhs: &TransformTR) -> TransformTR {
        mul_tr_tr(&self, rhs)
    }
}

impl Mul<TransformTRS> for TransformTRS {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        mul_trs_trs(&self, &rhs)
    }
}

impl Mul<&TransformTRS> for TransformTRS {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        mul_trs_trs(&self, rhs)
    }
}

impl Mul<TransformTR> for TransformTRS {
    type Output = TransformTRS;
    #[inline]
    fn mul(self, rhs: TransformTR) -> Self::Output {
        mul_trs_trs(&self, &rhs.into())
    }
}

impl Mul<&TransformTR> for TransformTRS {
    type Output = TransformTRS;
    #[inline]
    fn mul(self, rhs: &TransformTR) -> Self::Output {
        mul_trs_trs(&self, &rhs.into())
    }
}

impl Mul<TransformTRS> for TransformTR {
    type Output = TransformTRS;
    #[inline]
    fn mul(self, rhs: TransformTRS) -> Self::Output {
        mul_trs_trs(&self.into(), &rhs)
    }
}

impl Mul<&TransformTRS> for TransformTR {
    type Output = TransformTRS;
    #[inline]
    fn mul(self, rhs: &TransformTRS) -> Self::Output {
        mul_trs_trs(&self.into(), rhs)
    }
}

impl PartialEq for TransformTR {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.translation == rhs.translation && self.rotation == rhs.rotation
    }
}

impl PartialEq for TransformTRS {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.translation == rhs.translation
            && self.rotation == rhs.rotation
            && self.scale == rhs.scale
    }
}

impl From<TransformTR> for TransformTRS {
    #[inline]
    fn from(tr: TransformTR) -> Self {
        Self {
            translation: tr.translation,
            rotation: tr.rotation,
            scale: Vec3::one(),
        }
    }
}

impl From<&TransformTR> for TransformTRS {
    #[inline]
    fn from(tr: &TransformTR) -> Self {
        Self {
            translation: tr.translation,
            rotation: tr.rotation,
            scale: Vec3::one(),
        }
    }
}
