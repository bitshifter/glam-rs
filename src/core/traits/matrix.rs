use crate::core::{
    storage::{Vector2x2, Vector3x3, Vector4x4, XY, XYZ, XYZW},
    traits::{
        quaternion::Quaternion,
        scalar::{FloatEx, NumEx},
        vector::*,
    },
};

pub trait MatrixConst {
    const ZERO: Self;
    const IDENTITY: Self;
}

pub trait Matrix<T: NumEx>: Sized + Copy + Clone {}

pub trait Matrix2x2<T: NumEx, V2: Vector2<T>>: Matrix<T> {
    #[inline(always)]
    fn new(m00: T, m01: T, m10: T, m11: T) -> Self {
        Self::from_cols(V2::new(m00, m01), V2::new(m10, m11))
    }

    fn from_cols(x_axis: V2, y_axis: V2) -> Self;

    #[inline(always)]
    fn x_axis(&self) -> &V2 {
        &self.as_ref_vector2x2().x_axis
    }

    #[inline(always)]
    fn y_axis(&self) -> &V2 {
        &self.as_ref_vector2x2().y_axis
    }

    fn as_ref_vector2x2(&self) -> &Vector2x2<V2>;
    fn as_mut_vector2x2(&mut self) -> &mut Vector2x2<V2>;

    #[inline(always)]
    fn from_cols_array(m: &[T; 4]) -> Self {
        Self::new(m[0], m[1], m[2], m[3])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn to_cols_array(&self) -> [T; 4] {
        let x_axis = self.x_axis().as_ref_xy();
        let y_axis = self.y_axis().as_ref_xy();
        [x_axis.x, x_axis.y,
         y_axis.x, y_axis.y]
    }

    #[inline(always)]
    fn from_cols_array_2d(m: &[[T; 2]; 2]) -> Self {
        Self::from_cols(V2::from_array(m[0]), V2::from_array(m[1]))
    }

    #[inline(always)]
    fn to_cols_array_2d(&self) -> [[T; 2]; 2] {
        [self.x_axis().into_array(), self.y_axis().into_array()]
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn from_scale(scale: V2) -> Self {
        let (scale_x, scale_y) = scale.into_tuple();
        Self::new(
            scale_x, T::ZERO,
            T::ZERO, scale_y)
    }

    fn determinant(&self) -> T;
    fn transpose(&self) -> Self;
    fn mul_vector(&self, other: V2) -> V2;
    fn mul_matrix(&self, other: &Self) -> Self;
    fn mul_scalar(&self, other: T) -> Self;
    fn add_matrix(&self, other: &Self) -> Self;
    fn sub_matrix(&self, other: &Self) -> Self;
}

pub trait FloatMatrix2x2<T: FloatEx, V2: FloatVector2<T>>: Matrix2x2<T, V2> {
    #[inline]
    fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool
    where
        <V2 as Vector<T>>::Mask: MaskVector2,
    {
        self.x_axis().abs_diff_eq(*other.x_axis(), max_abs_diff)
            && self.y_axis().abs_diff_eq(*other.y_axis(), max_abs_diff)
    }

    #[inline]
    fn from_scale_angle(scale: V2, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        let (scale_x, scale_y) = scale.into_tuple();
        Self::new(cos * scale_x, sin * scale_x, -sin * scale_y, cos * scale_y)
    }

    #[inline]
    fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(cos, sin, -sin, cos)
    }

    fn inverse(&self) -> Self;
    // fn is_finite(&self) -> bool;
}

pub trait Matrix3x3<T: NumEx, V3: Vector3<T>>: Matrix<T> {
    fn from_cols(x_axis: V3, y_axis: V3, z_axis: V3) -> Self;

    #[inline(always)]
    fn new(m00: T, m01: T, m02: T, m10: T, m11: T, m12: T, m20: T, m21: T, m22: T) -> Self {
        Self::from_cols(
            V3::new(m00, m01, m02),
            V3::new(m10, m11, m12),
            V3::new(m20, m21, m22),
        )
    }

    #[inline(always)]
    fn x_axis(&self) -> &V3 {
        &self.as_ref_vector3x3().x_axis
    }

    #[inline(always)]
    fn y_axis(&self) -> &V3 {
        &self.as_ref_vector3x3().y_axis
    }

    #[inline(always)]
    fn z_axis(&self) -> &V3 {
        &self.as_ref_vector3x3().z_axis
    }

    fn as_ref_vector3x3(&self) -> &Vector3x3<V3>;
    fn as_mut_vector3x3(&mut self) -> &mut Vector3x3<V3>;

    #[rustfmt::skip]
    #[inline(always)]
    fn from_cols_array(m: &[T; 9]) -> Self {
        Self::new(
            m[0], m[1], m[2],
            m[3], m[4], m[5],
            m[6], m[7], m[8])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn to_cols_array(&self) -> [T; 9] {
        let x_axis = self.x_axis().as_ref_xyz();
        let y_axis = self.y_axis().as_ref_xyz();
        let z_axis = self.z_axis().as_ref_xyz();
        [
            x_axis.x, x_axis.y, x_axis.z,
            y_axis.x, y_axis.y, y_axis.z,
            z_axis.x, z_axis.y, z_axis.z,
        ]
    }

    #[inline(always)]
    fn from_cols_array_2d(m: &[[T; 3]; 3]) -> Self {
        Self::from_cols(
            V3::from_array(m[0]),
            V3::from_array(m[1]),
            V3::from_array(m[2]),
        )
    }

    #[inline(always)]
    fn to_cols_array_2d(&self) -> [[T; 3]; 3] {
        [
            self.x_axis().into_array(),
            self.y_axis().into_array(),
            self.z_axis().into_array(),
        ]
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn from_scale(scale: V3) -> Self {
        // glam_assert!(MaskVector3::any(scale.cmpne(V3::ZERO)));
        let (scale_x, scale_y, scale_z) = scale.into_tuple();
        Self::new(
            scale_x, T::ZERO, T::ZERO,
            T::ZERO, scale_y, T::ZERO,
            T::ZERO, T::ZERO, scale_z,
        )
    }

    fn determinant(&self) -> T;
    fn transpose(&self) -> Self;
    fn mul_vector(&self, other: V3) -> V3;
    fn mul_matrix(&self, other: &Self) -> Self;
    fn mul_scalar(&self, other: T) -> Self;
    fn add_matrix(&self, other: &Self) -> Self;
    fn sub_matrix(&self, other: &Self) -> Self;
}

pub trait FloatMatrix3x3<T: FloatEx, V3: FloatVector3<T>>: Matrix3x3<T, V3> {
    #[inline]
    fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool
    where
        <V3 as Vector<T>>::Mask: MaskVector3,
    {
        self.x_axis().abs_diff_eq(*other.x_axis(), max_abs_diff)
            && self.y_axis().abs_diff_eq(*other.y_axis(), max_abs_diff)
            && self.z_axis().abs_diff_eq(*other.z_axis(), max_abs_diff)
    }

    #[rustfmt::skip]
    #[inline]
    fn from_scale_angle_translation(scale: XY<T>, angle: T, translation: XY<T>) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(
            cos * scale.x, sin * scale.x, T::ZERO,
            -sin * scale.y, cos * scale.y, T::ZERO,
            translation.x, translation.y, T::ONE)
    }

    #[rustfmt::skip]
    #[inline]
    fn from_axis_angle(axis: V3, angle: T) -> Self {
        glam_assert!(axis.is_normalized());
        let (sin, cos) = angle.sin_cos();
        let (xsin, ysin, zsin) = axis.mul_scalar(sin).into_tuple();
        let (x, y, z) = axis.into_tuple();
        let (x2, y2, z2) = axis.mul(axis).into_tuple();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;
        Self::new(
            x2 * omc + cos, xyomc + zsin, xzomc - ysin,
            xyomc - zsin, y2 * omc + cos, yzomc + xsin,
            xzomc + ysin, yzomc - xsin, z2 * omc + cos,
        )
    }

    #[rustfmt::skip]
    #[inline]
    fn from_quaternion(rotation: XYZW<T>) -> Self {
        glam_assert!(rotation.is_normalized());
        let x2 = rotation.x + rotation.x;
        let y2 = rotation.y + rotation.y;
        let z2 = rotation.z + rotation.z;
        let xx = rotation.x * x2;
        let xy = rotation.x * y2;
        let xz = rotation.x * z2;
        let yy = rotation.y * y2;
        let yz = rotation.y * z2;
        let zz = rotation.z * z2;
        let wx = rotation.w * x2;
        let wy = rotation.w * y2;
        let wz = rotation.w * z2;

        Self::new(
            T::ONE - (yy + zz), xy + wz, xz - wy,
            xy - wz, T::ONE - (xx + zz), yz + wx,
            xz + wy, yz - wx, T::ONE - (xx + yy))
    }

    #[rustfmt::skip]
    #[inline]
    fn from_rotation_x(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::new(
            T::ONE, T::ZERO, T::ZERO,
            T::ZERO, cosa, sina,
            T::ZERO, -sina, cosa)
    }

    #[rustfmt::skip]
    #[inline]
    fn from_rotation_y(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::new(
            cosa, T::ZERO, -sina,
            T::ZERO, T::ONE, T::ZERO,
            sina, T::ZERO, cosa)
    }

    #[rustfmt::skip]
    #[inline]
    fn from_rotation_z(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::new(
            cosa, sina, T::ZERO,
            -sina, cosa, T::ZERO,
            T::ZERO, T::ZERO, T::ONE)
    }

    // #[rustfmt::skip]
    // #[inline]
    // fn from_scale(scale: XYZ<T>) -> Self {
    //     // TODO: should have a affine 2D scale and a 3d scale?
    //     // Do not panic as long as any component is non-zero
    //     glam_assert!(scale.cmpne(XYZ::ZERO).any());
    //     Self::new(
    //         scale.x, T::ZERO, T::ZERO,
    //         T::ZERO, scale.y, T::ZERO,
    //         T::ZERO, T::ZERO, scale.z,
    //     )
    // }

    fn transform_point2(&self, other: XY<T>) -> XY<T>;
    fn transform_vector2(&self, other: XY<T>) -> XY<T>;

    fn inverse(&self) -> Self;
}

pub trait Matrix4x4<T: NumEx, V4: Vector4<T>>: Matrix<T> {
    #[rustfmt::skip]
    #[inline(always)]
    fn new(
        m00: T, m01: T, m02: T, m03: T,
        m10: T, m11: T, m12: T, m13: T,
        m20: T, m21: T, m22: T, m23: T,
        m30: T, m31: T, m32: T, m33: T,
        ) -> Self {
        Self::from_cols(
            V4::new(m00, m01, m02, m03),
            V4::new(m10, m11, m12, m13),
            V4::new(m20, m21, m22, m23),
            V4::new(m30, m31, m32, m33),
        )
    }

    fn from_cols(x_axis: V4, y_axis: V4, z_axis: V4, w_axis: V4) -> Self;

    fn x_axis(&self) -> &V4;
    fn y_axis(&self) -> &V4;
    fn z_axis(&self) -> &V4;
    fn w_axis(&self) -> &V4;

    fn as_ref_vector4x4(&self) -> &Vector4x4<V4>;
    fn as_mut_vector4x4(&mut self) -> &mut Vector4x4<V4>;

    #[rustfmt::skip]
    #[inline(always)]
    fn from_cols_array(m: &[T; 16]) -> Self {
        Self::new(
             m[0],  m[1],  m[2],  m[3],
             m[4],  m[5],  m[6],  m[7],
             m[8],  m[9], m[10], m[11],
            m[12], m[13], m[14], m[15])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn to_cols_array(&self) -> [T; 16] {
        let x_axis = self.x_axis().as_ref_xyzw();
        let y_axis = self.y_axis().as_ref_xyzw();
        let z_axis = self.z_axis().as_ref_xyzw();
        let w_axis = self.w_axis().as_ref_xyzw();
        [
            x_axis.x, x_axis.y, x_axis.z, x_axis.w,
            y_axis.x, y_axis.y, y_axis.z, y_axis.w,
            z_axis.x, z_axis.y, z_axis.z, z_axis.w,
            w_axis.x, w_axis.y, w_axis.z, w_axis.w,
        ]
    }

    #[inline(always)]
    fn from_cols_array_2d(m: &[[T; 4]; 4]) -> Self {
        Self::from_cols(
            Vector4::from_array(m[0]),
            Vector4::from_array(m[1]),
            Vector4::from_array(m[2]),
            Vector4::from_array(m[3]),
        )
    }

    #[inline(always)]
    fn to_cols_array_2d(&self) -> [[T; 4]; 4] {
        [
            self.x_axis().into_array(),
            self.y_axis().into_array(),
            self.z_axis().into_array(),
            self.w_axis().into_array(),
        ]
    }

    #[inline(always)]
    fn from_scale(scale: XYZ<T>) -> Self {
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne(XYZ::<T>::ZERO).any());
        Self::from_cols(
            V4::new(scale.x, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, scale.y, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, scale.z, T::ZERO),
            V4::W,
        )
    }

    #[inline(always)]
    fn from_translation(translation: XYZ<T>) -> Self {
        Self::from_cols(
            V4::X,
            V4::Y,
            V4::Z,
            V4::new(translation.x, translation.y, translation.z, T::ONE),
        )
    }

    fn determinant(&self) -> T;
    fn transpose(&self) -> Self;

    #[inline]
    fn mul_vector(&self, other: &V4) -> V4 {
        let mut res = self.x_axis().mul(other.splat_x());
        res = self.y_axis().mul_add(other.splat_y(), res);
        res = self.z_axis().mul_add(other.splat_z(), res);
        res = self.w_axis().mul_add(other.splat_w(), res);
        res
    }

    #[inline]
    fn mul_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.mul_vector(other.x_axis()),
            self.mul_vector(other.y_axis()),
            self.mul_vector(other.z_axis()),
            self.mul_vector(other.w_axis()),
        )
    }

    #[inline]
    fn mul_scalar(&self, other: T) -> Self {
        Self::from_cols(
            self.x_axis().mul_scalar(other),
            self.y_axis().mul_scalar(other),
            self.z_axis().mul_scalar(other),
            self.w_axis().mul_scalar(other),
        )
    }

    #[inline]
    fn add_matrix(&self, other: &Self) -> Self {
        // TODO: Make Vector4::add take a ref?
        Self::from_cols(
            self.x_axis().add(*other.x_axis()),
            self.y_axis().add(*other.y_axis()),
            self.z_axis().add(*other.z_axis()),
            self.w_axis().add(*other.w_axis()),
        )
    }

    #[inline]
    fn sub_matrix(&self, other: &Self) -> Self {
        // TODO: Make Vector4::sub take a ref?
        Self::from_cols(
            self.x_axis().sub(*other.x_axis()),
            self.y_axis().sub(*other.y_axis()),
            self.z_axis().sub(*other.z_axis()),
            self.w_axis().sub(*other.w_axis()),
        )
    }
}

pub trait FloatMatrix4x4<T: FloatEx, V4: FloatVector4<T> + Quaternion<T>>:
    Matrix4x4<T, V4>
{
    // Vector3 represented by a SIMD type if available
    type SIMDVector3;

    #[inline]
    fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool
    where
        <V4 as Vector<T>>::Mask: MaskVector4,
    {
        self.x_axis().abs_diff_eq(*other.x_axis(), max_abs_diff)
            && self.y_axis().abs_diff_eq(*other.y_axis(), max_abs_diff)
            && self.z_axis().abs_diff_eq(*other.z_axis(), max_abs_diff)
            && self.w_axis().abs_diff_eq(*other.w_axis(), max_abs_diff)
    }

    #[inline]
    fn quaternion_to_axes(rotation: V4) -> (V4, V4, V4) {
        glam_assert!(rotation.is_normalized());
        let (x, y, z, w) = rotation.into_tuple();
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let x_axis = V4::new(T::ONE - (yy + zz), xy + wz, xz - wy, T::ZERO);
        let y_axis = V4::new(xy - wz, T::ONE - (xx + zz), yz + wx, T::ZERO);
        let z_axis = V4::new(xz + wy, yz - wx, T::ONE - (xx + yy), T::ZERO);
        (x_axis, y_axis, z_axis)
    }

    #[inline]
    fn from_quaternion(rotation: V4) -> Self {
        glam_assert!(rotation.is_normalized());
        let (x_axis, y_axis, z_axis) = Self::quaternion_to_axes(rotation);
        Self::from_cols(x_axis, y_axis, z_axis, V4::W)
    }

    fn to_scale_quaternion_translation(&self) -> (XYZ<T>, V4, XYZ<T>) {
        let det = self.determinant();
        glam_assert!(det != T::ZERO);

        let scale: XYZ<T> = Vector3::new(
            self.x_axis().length() * det.signum(),
            self.y_axis().length(),
            self.z_axis().length(),
        );

        glam_assert!(scale.cmpne(XYZ::<T>::ZERO).all());

        let inv_scale = scale.recip();

        let rotation = Quaternion::from_rotation_axes(
            self.x_axis().mul_scalar(inv_scale.x).into_xyz(),
            self.y_axis().mul_scalar(inv_scale.y).into_xyz(),
            self.z_axis().mul_scalar(inv_scale.z).into_xyz(),
        );

        let translation = self.w_axis().into_xyz();

        (scale, rotation, translation)
    }

    #[inline]
    fn from_scale_quaternion_translation(scale: XYZ<T>, rotation: V4, translation: XYZ<T>) -> Self {
        glam_assert!(rotation.is_normalized());
        let (x_axis, y_axis, z_axis) = Self::quaternion_to_axes(rotation);
        Self::from_cols(
            x_axis.mul_scalar(scale.x),
            y_axis.mul_scalar(scale.y),
            z_axis.mul_scalar(scale.z),
            V4::from_xyz(translation, T::ONE),
        )
    }

    #[inline]
    fn from_quaternion_translation(rotation: V4, translation: XYZ<T>) -> Self {
        glam_assert!(rotation.is_normalized());
        let (x_axis, y_axis, z_axis) = Self::quaternion_to_axes(rotation);
        Self::from_cols(x_axis, y_axis, z_axis, V4::from_xyz(translation, T::ONE))
    }

    #[inline]
    fn from_axis_angle(axis: XYZ<T>, angle: T) -> Self {
        glam_assert!(axis.is_normalized());
        let (sin, cos) = angle.sin_cos();
        let axis_sin = axis.mul_scalar(sin);
        let axis_sq = axis.mul(axis);
        let omc = T::ONE - cos;
        let xyomc = axis.x * axis.y * omc;
        let xzomc = axis.x * axis.z * omc;
        let yzomc = axis.y * axis.z * omc;
        Self::from_cols(
            V4::new(
                axis_sq.x * omc + cos,
                xyomc + axis_sin.z,
                xzomc - axis_sin.y,
                T::ZERO,
            ),
            V4::new(
                xyomc - axis_sin.z,
                axis_sq.y * omc + cos,
                yzomc + axis_sin.x,
                T::ZERO,
            ),
            V4::new(
                xzomc + axis_sin.y,
                yzomc - axis_sin.x,
                axis_sq.z * omc + cos,
                T::ZERO,
            ),
            V4::W,
        )
    }

    #[inline]
    fn from_rotation_x(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V4::X,
            V4::new(T::ZERO, cosa, sina, T::ZERO),
            V4::new(T::ZERO, -sina, cosa, T::ZERO),
            V4::W,
        )
    }

    #[inline]
    fn from_rotation_y(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V4::new(cosa, T::ZERO, -sina, T::ZERO),
            V4::Y,
            V4::new(sina, T::ZERO, cosa, T::ZERO),
            V4::W,
        )
    }

    #[inline]
    fn from_rotation_z(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V4::new(cosa, sina, T::ZERO, T::ZERO),
            V4::new(-sina, cosa, T::ZERO, T::ZERO),
            V4::Z,
            V4::W,
        )
    }

    #[inline]
    fn look_to_lh(eye: XYZ<T>, dir: XYZ<T>, up: XYZ<T>) -> Self {
        let f = dir.normalize();
        let s = up.cross(f).normalize();
        let u = f.cross(s);
        Self::from_cols(
            V4::new(s.x, u.x, f.x, T::ZERO),
            V4::new(s.y, u.y, f.y, T::ZERO),
            V4::new(s.z, u.z, f.z, T::ZERO),
            V4::new(-s.dot(eye), -u.dot(eye), -f.dot(eye), T::ONE),
        )
    }

    #[inline]
    fn look_at_lh(eye: XYZ<T>, center: XYZ<T>, up: XYZ<T>) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, center.sub(eye), up)
    }

    #[inline]
    fn look_at_rh(eye: XYZ<T>, center: XYZ<T>, up: XYZ<T>) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, eye.sub(center), up)
    }

    #[inline]
    fn transform_point3(&self, other: XYZ<T>) -> XYZ<T> {
        let mut res = self.x_axis().mul_scalar(other.x);
        res = self.y_axis().mul_scalar(other.y).add(res);
        res = self.z_axis().mul_scalar(other.z).add(res);
        res = self.w_axis().add(res);
        res = res.mul(res.splat_w().recip());
        res.into_xyz()
    }

    #[inline]
    fn transform_vector3(&self, other: XYZ<T>) -> XYZ<T> {
        let mut res = self.x_axis().mul_scalar(other.x);
        res = self.y_axis().mul_scalar(other.y).add(res);
        res = self.z_axis().mul_scalar(other.z).add(res);
        res.into_xyz()
    }

    fn transform_float4_as_point3(&self, other: Self::SIMDVector3) -> Self::SIMDVector3;
    fn transform_float4_as_vector3(&self, other: Self::SIMDVector3) -> Self::SIMDVector3;

    fn inverse(&self) -> Self;
}
