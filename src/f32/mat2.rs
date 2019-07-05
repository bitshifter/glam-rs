use super::{super::Angle, Vec2, Vec4};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::ops::{Add, Mul, Sub};

#[inline]
pub fn mat2(x_axis: Vec2, y_axis: Vec2) -> Mat2 {
    Mat2::new(x_axis, y_axis)
}

/// A 2x2 column major matrix.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Mat2(pub(crate) Vec4);

impl Default for Mat2 {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Mat2 {
    #[inline]
    pub fn zero() -> Self {
        Mat2(Vec4::zero())
    }

    #[inline]
    pub fn identity() -> Self {
        Self(Vec4::new(1.0, 0.0, 0.0, 1.0))
    }

    #[inline]
    pub fn new(x_axis: Vec2, y_axis: Vec2) -> Self {
        Self(Vec4::new(x_axis.x(), x_axis.y(), y_axis.x(), y_axis.y()))
    }

    #[inline]
    pub fn from_scale_angle(scale: Vec2, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        let (scale_x, scale_y) = scale.into();
        Self(Vec4::new(
            cos * scale_x,
            sin * scale_x,
            -sin * scale_y,
            cos * scale_y,
        ))
    }

    #[inline]
    pub fn from_angle(angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self(Vec4::new(cos, sin, -sin, cos))
    }

    #[inline]
    pub fn from_scale(scale: Vec2) -> Self {
        let (x, y) = scale.into();
        Self(Vec4::new(x, 0.0, 0.0, y))
    }

    #[inline]
    pub fn set_x_axis(&mut self, x: Vec2) {
        let m = self.0.as_mut();
        m[0] = x.x();
        m[1] = x.y();
    }

    #[inline]
    pub fn set_y_axis(&mut self, y: Vec2) {
        let m = self.0.as_mut();
        m[2] = y.x();
        m[3] = y.y();
    }

    #[inline]
    pub fn x_axis(&self) -> Vec2 {
        let (x, y, _, _) = self.0.into();
        Vec2::new(x, y)
    }

    #[inline]
    pub fn y_axis(&self) -> Vec2 {
        let (_, _, x, y) = self.0.into();
        Vec2::new(x, y)
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        let (m00, m01, m10, m11) = self.0.into();
        Self(Vec4::new(m00, m10, m01, m11))
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        // TODO: SSE2
        let (a, b, c, d) = self.0.into();
        a * d - b * c
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        // TODO: SSE2
        let (a, b, c, d) = self.0.into();
        let det = a * d - b * c;
        glam_assert!(det != 0.0);
        let tmp = Vec4::new(1.0, -1.0, -1.0, 1.0) / det;
        Self(Vec4::new(d, b, c, a) * tmp)
    }

    #[inline]
    pub fn mul_vec2(&self, rhs: Vec2) -> Vec2 {
        // TODO: SSE2
        let rhs = Vec4::new(rhs.x(), rhs.x(), rhs.y(), rhs.y());
        let tmp = self.0 * rhs;
        let (x0, y0, x1, y1) = tmp.into();
        Vec2::new(x0 + x1, y0 + y1)
    }

    #[inline]
    pub fn mul_mat2(&self, rhs: &Self) -> Self {
        // TODO: SSE2
        let (x0, y0, x1, y1) = rhs.0.into();
        Mat2::new(
            self.mul_vec2(Vec2::new(x0, y0)),
            self.mul_vec2(Vec2::new(x1, y1)),
        )
    }

    #[inline]
    pub fn add_mat2(&self, rhs: &Self) -> Self {
        Mat2(self.0 + rhs.0)
    }

    #[inline]
    pub fn sub_mat2(&self, rhs: &Self) -> Self {
        Mat2(self.0 - rhs.0)
    }

    #[inline]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        let s = Vec4::splat(rhs);
        Mat2(self.0 * s)
    }
}

#[cfg(feature = "rand")]
impl Distribution<Mat2> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat2 {
        rng.gen::<[[f32; 2]; 2]>().into()
    }
}

impl AsRef<[f32; 4]> for Mat2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Mat2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 4]) }
    }
}

impl Add<Mat2> for Mat2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.add_mat2(&rhs)
    }
}

impl Sub<Mat2> for Mat2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.sub_mat2(&rhs)
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_mat2(&rhs)
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        self.mul_vec2(rhs)
    }
}

impl Mul<Mat2> for f32 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: Mat2) -> Mat2 {
        rhs.mul_scalar(self)
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        self.mul_scalar(rhs)
    }
}

impl From<[[f32; 2]; 2]> for Mat2 {
    #[inline]
    fn from(m: [[f32; 2]; 2]) -> Self {
        Mat2(Vec4::new(m[0][0], m[0][1], m[1][0], m[1][1]))
    }
}

impl From<Mat2> for [[f32; 2]; 2] {
    #[inline]
    fn from(m: Mat2) -> Self {
        let (x0, y0, x1, y1) = m.0.into();
        [[x0, y0], [x1, y1]]
    }
}

impl From<[f32; 4]> for Mat2 {
    #[inline]
    /// Load from array in column major order.
    fn from(m: [f32; 4]) -> Self {
        Mat2(m.into())
    }
}

impl From<Mat2> for [f32; 4] {
    #[inline]
    /// Store to array in column major order.
    fn from(m: Mat2) -> Self {
        m.0.into()
    }
}
