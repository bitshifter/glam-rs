use core::arch::wasm32::*;

use crate::{
    const_v128,
    core::{
        storage::{Align16, Columns2, Columns3, Columns4, XY, XYZ},
        traits::{
            matrix::{
                FloatMatrix2x2, FloatMatrix3x3, FloatMatrix4x4, Matrix, Matrix2x2, Matrix3x3,
                Matrix4x4, MatrixConst,
            },
            projection::ProjectionMatrix,
            vector::{FloatVector4, Vector, Vector4, Vector4Const, VectorConst},
        },
    },
};

impl MatrixConst for Columns3<v128> {
    const ZERO: Columns3<v128> = Columns3 {
        x_axis: v128::ZERO,
        y_axis: v128::ZERO,
        z_axis: v128::ZERO,
    };
    const IDENTITY: Columns3<v128> = Columns3 {
        x_axis: v128::X,
        y_axis: v128::Y,
        z_axis: v128::Z,
    };
}

impl Matrix<f32> for Columns3<v128> {}

impl Matrix3x3<f32, v128> for Columns3<v128> {
    #[inline(always)]
    fn from_cols(x_axis: v128, y_axis: v128, z_axis: v128) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    fn x_axis(&self) -> &v128 {
        &self.x_axis
    }

    #[inline(always)]
    fn y_axis(&self) -> &v128 {
        &self.y_axis
    }

    #[inline(always)]
    fn z_axis(&self) -> &v128 {
        &self.z_axis
    }

    #[inline]
    fn transpose(&self) -> Self {
        let tmp0 = i32x4_shuffle::<0, 1, 4, 5>(self.x_axis, self.y_axis);
        let tmp1 = i32x4_shuffle::<2, 3, 6, 7>(self.x_axis, self.y_axis);

        Self {
            x_axis: i32x4_shuffle::<0, 2, 4, 4>(tmp0, self.z_axis),
            y_axis: i32x4_shuffle::<1, 3, 5, 5>(tmp0, self.z_axis),
            z_axis: i32x4_shuffle::<0, 2, 6, 6>(tmp1, self.z_axis),
        }
    }
}

impl FloatMatrix3x3<f32, v128> for Columns3<v128> {
    #[inline]
    fn transform_point2(&self, other: XY<f32>) -> XY<f32> {
        let mut res = self.x_axis.mul_scalar(other.x);
        res = self.y_axis.mul_scalar(other.y).add(res);
        res = self.z_axis.add(res);
        res.into()
    }

    #[inline]
    fn transform_vector2(&self, other: XY<f32>) -> XY<f32> {
        let mut res = self.x_axis.mul_scalar(other.x);
        res = self.y_axis.mul_scalar(other.y).add(res);
        res.into()
    }
}

impl From<Columns3<XYZ<f32>>> for Columns3<v128> {
    #[inline(always)]
    fn from(v: Columns3<XYZ<f32>>) -> Columns3<v128> {
        Self {
            x_axis: v.x_axis.into(),
            y_axis: v.y_axis.into(),
            z_axis: v.z_axis.into(),
        }
    }
}

impl From<Columns3<v128>> for Columns3<XYZ<f32>> {
    #[inline(always)]
    fn from(v: Columns3<v128>) -> Columns3<XYZ<f32>> {
        Self {
            x_axis: v.x_axis.into(),
            y_axis: v.y_axis.into(),
            z_axis: v.z_axis.into(),
        }
    }
}
