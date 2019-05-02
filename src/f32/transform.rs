use super::{Quat, Vec3};
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
}

// #[inline]
// fn mul_trs_trs(lhs: &TransformTRS, rhs: &TransformTRS) -> TransformTRS {
//     let min_scale = lhs.scale.min(rhs.scale);
//     let scale = lhs.scale * rhs.scale;

//     if min_scale.cmplt(Vec3::zero()).any() {
//         // If we have negative scale, we go through a matrix
//         let lhs_mtx = Mat4::from_transform_trs(lhs);
//         let rhs_mtx = Mat4::from_transform_trs(rhs);
//         let result_mtx = lhs_mtx * rhs_mtx;
//         result_mtx = matrix_remove_scale(result_mtx);

//         const vector4f sign = vector_sign(scale);
//         result_mtx.x_axis = vector_mul(result_mtx.x_axis, vector_dup_x(sign));
//         result_mtx.y_axis = vector_mul(result_mtx.y_axis, vector_dup_y(sign));
//         result_mtx.z_axis = vector_mul(result_mtx.z_axis, vector_dup_z(sign));

//         const quatf rotation = quat_from_matrix(result_mtx);
//         const vector4f translation = result_mtx.w_axis;
//         return qvv_set(rotation, translation, scale);
//     }
//     else
//     {
//         let rotation = lhs.rotation * rhs.rotation;
//         let translation = ((lhs.translation * rhs.scale) * rhs.rotation) + rhs.translation;
//         TransformTRS {
//             translation,
//             rotation,
//             scale
//         }
//     }
// }

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
