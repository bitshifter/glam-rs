use super::{super::Angle, Vec2};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::ops::{Add, Mul, Sub};

pub fn mat2(x_axis: Vec2, y_axis: Vec2) -> Mat2 {
    Mat2 { x_axis, y_axis }
}

// TODO: Could use Vec4 for storage
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Mat2 {
    pub(crate) x_axis: Vec2,
    pub(crate) y_axis: Vec2,
}

impl Default for Mat2 {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Mat2 {
    #[inline]
    pub fn zero() -> Self {
        Self {
            x_axis: Vec2::zero(),
            y_axis: Vec2::zero(),
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            x_axis: Vec2::unit_x(),
            y_axis: Vec2::unit_y(),
        }
    }

    #[inline]
    pub fn new(x_axis: Vec2, y_axis: Vec2) -> Self {
        Self { x_axis, y_axis }
    }

    #[inline]
    pub fn from_scale_angle(scale: Vec2, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        let (scale_x, scale_y) = scale.into();
        Self {
            x_axis: Vec2::new(cos * scale_x, sin * scale_x),
            y_axis: Vec2::new(-sin * scale_y, cos * scale_y),
        }
    }

    #[inline]
    pub fn from_angle(angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x_axis: Vec2::new(cos, sin),
            y_axis: Vec2::new(-sin, cos),
        }
    }

    #[inline]
    pub fn from_scale(scale: Vec2) -> Self {
        let (x, y) = scale.into();
        Self {
            x_axis: Vec2::new(x, 0.0),
            y_axis: Vec2::new(0.0, y),
        }
    }

    #[inline]
    pub fn set_x_axis(&mut self, x: Vec2) {
        self.x_axis = x;
    }

    #[inline]
    pub fn set_y_axis(&mut self, y: Vec2) {
        self.y_axis = y;
    }

    #[inline]
    pub fn x_axis(&self) -> Vec2 {
        self.x_axis
    }

    #[inline]
    pub fn y_axis(&self) -> Vec2 {
        self.y_axis
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        let (m00, m01) = self.x_axis.into();
        let (m10, m11) = self.y_axis.into();

        Self {
            x_axis: Vec2::new(m00, m10),
            y_axis: Vec2::new(m01, m11),
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        let (a, b) = self.x_axis.into();
        let (c, d) = self.y_axis.into();
        a * d - b * c
    }

    pub fn inverse(&self) -> Self {
        let (a, b) = self.x_axis.into();
        let (c, d) = self.y_axis.into();
        let det = a * d - b * c;
        debug_assert!(det != 0.0);
        let inv_det = 1.0 / det;
        Self {
            x_axis: Vec2::new(inv_det * d, -inv_det * b),
            y_axis: Vec2::new(-inv_det * c, inv_det * a),
        }
    }

    #[inline]
    /// Multiplies two 2x2 matrices.
    /// Multiplication order is as follows:
    /// `local_to_world = local_to_object * local_to_world`
    pub fn mul_mat2(&self, rhs: &Self) -> Self {
        Self {
            x_axis: rhs.x_axis * *self,
            y_axis: rhs.y_axis * *self,
        }
    }

    #[inline]
    pub fn add_mat2(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
        }
    }

    #[inline]
    pub fn sub_mat2(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
        }
    }

    #[inline]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        let s = Vec2::splat(rhs);
        Self {
            x_axis: self.x_axis * s,
            y_axis: self.y_axis * s,
        }
    }
}

// implemented here so they don't need to be duplicated between f32x4 and f32 versions
impl Vec2 {
    #[inline]
    /// Multiplies a 2x2 matrix and a 2D vector.
    /// Multiplication order is as follows:
    /// `world_direction = local_direction.transform_mat3(local_to_world)`
    pub fn transform_mat2(self, rhs: &Mat2) -> Self {
        let mut tmp = self.dup_x().mul(rhs.x_axis());
        tmp = self.dup_y().mul_add(rhs.y_axis(), tmp);
        tmp
    }
}

#[cfg(feature = "rand")]
impl Distribution<Mat2> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat2 {
        rng.gen::<[[f32; 2]; 2]>().into()
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

impl Mul<Mat2> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Mat2) -> Vec2 {
        self.transform_mat2(&rhs)
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
        Mat2 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
        }
    }
}

impl From<&[[f32; 2]; 2]> for Mat2 {
    #[inline]
    fn from(m: &[[f32; 2]; 2]) -> Self {
        Mat2 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
        }
    }
}

impl From<&Mat2> for [[f32; 2]; 2] {
    #[inline]
    fn from(m: &Mat2) -> Self {
        [m.x_axis.into(), m.y_axis.into()]
    }
}

impl From<Mat2> for [[f32; 2]; 2] {
    #[inline]
    fn from(m: Mat2) -> Self {
        [m.x_axis.into(), m.y_axis.into()]
    }
}
