use crate::core::{
    storage::{XY, XYZ, XYZW},
    traits::{scalar::*, vector::*},
};

impl<T> XY<T> {
    #[inline(always)]
    pub(crate) fn map<D, F>(self, f: F) -> XY<D>
    where
        F: Fn(T) -> D,
    {
        XY {
            x: f(self.x),
            y: f(self.y),
        }
    }

    #[inline(always)]
    pub(crate) fn map2<D, F>(self, other: Self, f: F) -> XY<D>
    where
        F: Fn(T, T) -> D,
    {
        XY {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
        }
    }

    #[inline(always)]
    pub(crate) fn map3<D, F>(self, a: Self, b: Self, f: F) -> XY<D>
    where
        F: Fn(T, T, T) -> D,
    {
        XY {
            x: f(self.x, a.x, b.x),
            y: f(self.y, a.y, b.y),
        }
    }
}

impl<T> XYZ<T> {
    #[inline(always)]
    pub(crate) fn map<D, F>(self, f: F) -> XYZ<D>
    where
        F: Fn(T) -> D,
    {
        XYZ {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    #[inline(always)]
    pub(crate) fn map2<D, F>(self, other: Self, f: F) -> XYZ<D>
    where
        F: Fn(T, T) -> D,
    {
        XYZ {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
        }
    }

    #[inline(always)]
    pub(crate) fn map3<D, F>(self, a: Self, b: Self, f: F) -> XYZ<D>
    where
        F: Fn(T, T, T) -> D,
    {
        XYZ {
            x: f(self.x, a.x, b.x),
            y: f(self.y, a.y, b.y),
            z: f(self.z, a.z, b.z),
        }
    }
}

impl<T> XYZW<T> {
    #[inline(always)]
    pub(crate) fn map<D, F>(self, f: F) -> XYZW<D>
    where
        F: Fn(T) -> D,
    {
        XYZW {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
            w: f(self.w),
        }
    }

    #[inline(always)]
    pub(crate) fn map2<D, F>(self, other: Self, f: F) -> XYZW<D>
    where
        F: Fn(T, T) -> D,
    {
        XYZW {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
            w: f(self.w, other.w),
        }
    }

    #[inline(always)]
    pub(crate) fn map3<D, F>(self, a: Self, b: Self, f: F) -> XYZW<D>
    where
        F: Fn(T, T, T) -> D,
    {
        XYZW {
            x: f(self.x, a.x, b.x),
            y: f(self.y, a.y, b.y),
            z: f(self.z, a.z, b.z),
            w: f(self.w, a.w, b.w),
        }
    }
}

impl<T: NumEx> VectorConst for XY<T> {
    const ZERO: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
    };
    const ONE: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> Vector2Const for XY<T> {
    const UNIT_X: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ZERO,
    };
    const UNIT_Y: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> VectorConst for XYZ<T> {
    const ZERO: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
    };
    const ONE: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ONE,
        z: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> Vector3Const for XYZ<T> {
    const UNIT_X: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
    };
    const UNIT_Y: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ONE,
        z: <T as NumConstEx>::ZERO,
    };
    const UNIT_Z: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> VectorConst for XYZW<T> {
    const ZERO: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
        w: <T as NumConstEx>::ZERO,
    };
    const ONE: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ONE,
        z: <T as NumConstEx>::ONE,
        w: <T as NumConstEx>::ONE,
    };
}
impl<T: NumEx> Vector4Const for XYZW<T> {
    const UNIT_X: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
        w: <T as NumConstEx>::ZERO,
    };
    const UNIT_Y: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ONE,
        z: <T as NumConstEx>::ZERO,
        w: <T as NumConstEx>::ZERO,
    };
    const UNIT_Z: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ONE,
        w: <T as NumConstEx>::ZERO,
    };
    const UNIT_W: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
        w: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> Vector<T> for XY<T> {
    type Mask = XY<bool>;

    #[inline]
    fn splat(s: T) -> Self {
        Self { x: s, y: s }
    }

    #[inline]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x { if_true.x } else { if_false.x },
            y: if mask.y { if_true.y } else { if_false.y },
        }
    }

    #[inline]
    fn cmpeq(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| a.eq(&b))
    }

    #[inline]
    fn cmpne(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| a.ne(&b))
    }

    #[inline]
    fn cmpge(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| a.ge(&b))
    }

    #[inline]
    fn cmpgt(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| a.gt(&b))
    }

    #[inline]
    fn cmple(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| a.le(&b))
    }

    #[inline]
    fn cmplt(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| a.lt(&b))
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        self.map2(other, |a, b| a + b)
    }

    #[inline]
    fn div(self, other: Self) -> Self {
        self.map2(other, |a, b| a / b)
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        self.map2(other, |a, b| a * b)
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        self.map3(b, c, |a, b, c| a * b + c)
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        self.map2(other, |a, b| a - b)
    }

    #[inline]
    fn mul_scalar(self, other: T) -> Self {
        self.map(|a| a * other)
    }

    #[inline]
    fn div_scalar(self, other: T) -> Self {
        self.map(|a| a / other)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        self.map2(other, |a, b| a.min(b))
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        self.map2(other, |a, b| a.max(b))
    }
}

impl<T: NumEx> Vector<T> for XYZ<T> {
    type Mask = XYZ<u32>;

    #[inline]
    fn splat(s: T) -> Self {
        Self { x: s, y: s, z: s }
    }

    #[inline]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x != 0 { if_true.x } else { if_false.x },
            y: if mask.y != 0 { if_true.y } else { if_false.y },
            z: if mask.z != 0 { if_true.z } else { if_false.z },
        }
    }

    #[inline]
    fn cmpeq(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.eq(&b) as usize])
    }

    #[inline]
    fn cmpne(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.ne(&b) as usize])
    }

    #[inline]
    fn cmpge(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.ge(&b) as usize])
    }

    #[inline]
    fn cmpgt(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.gt(&b) as usize])
    }

    #[inline]
    fn cmple(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.le(&b) as usize])
    }

    #[inline]
    fn cmplt(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.lt(&b) as usize])
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        self.map2(other, |a, b| a + b)
    }

    #[inline]
    fn div(self, other: Self) -> Self {
        self.map2(other, |a, b| a / b)
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        self.map2(other, |a, b| a * b)
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        self.map3(b, c, |a, b, c| a * b + c)
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        self.map2(other, |a, b| a - b)
    }

    #[inline]
    fn mul_scalar(self, other: T) -> Self {
        self.map(|a| a * other)
    }

    #[inline]
    fn div_scalar(self, other: T) -> Self {
        self.map(|a| a / other)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        self.map2(other, |a, b| a.min(b))
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        self.map2(other, |a, b| a.max(b))
    }
}

impl<T: NumEx> Vector<T> for XYZW<T> {
    type Mask = XYZW<u32>;

    #[inline]
    fn splat(s: T) -> Self {
        Self {
            x: s,
            y: s,
            z: s,
            w: s,
        }
    }

    #[inline]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x != 0 { if_true.x } else { if_false.x },
            y: if mask.y != 0 { if_true.y } else { if_false.y },
            z: if mask.z != 0 { if_true.z } else { if_false.z },
            w: if mask.w != 0 { if_true.w } else { if_false.w },
        }
    }

    #[inline]
    fn cmpeq(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.eq(&b) as usize])
    }

    #[inline]
    fn cmpne(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.ne(&b) as usize])
    }

    #[inline]
    fn cmpge(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.ge(&b) as usize])
    }

    #[inline]
    fn cmpgt(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.gt(&b) as usize])
    }

    #[inline]
    fn cmple(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.le(&b) as usize])
    }

    #[inline]
    fn cmplt(self, other: Self) -> Self::Mask {
        self.map2(other, |a, b| MaskConst::MASK[a.lt(&b) as usize])
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        self.map2(other, |a, b| a + b)
    }

    #[inline]
    fn div(self, other: Self) -> Self {
        self.map2(other, |a, b| a / b)
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        self.map2(other, |a, b| a * b)
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        self.map3(b, c, |a, b, c| a * b + c)
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        self.map2(other, |a, b| a - b)
    }

    #[inline]
    fn mul_scalar(self, other: T) -> Self {
        self.map(|a| a * other)
    }

    #[inline]
    fn div_scalar(self, other: T) -> Self {
        self.map(|a| a / other)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        self.map2(other, |a, b| a.min(b))
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        self.map2(other, |a, b| a.max(b))
    }
}

impl<T: NumEx> Vector2<T> for XY<T> {
    #[inline(always)]
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x)
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y)
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self::new(slice[0], slice[1])
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x;
        slice[1] = self.y;
    }

    #[inline(always)]
    fn as_ref_xy(&self) -> &XY<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xy(&mut self) -> &mut XY<T> {
        self
    }

    #[inline(always)]
    fn into_xyz(self, z: T) -> XYZ<T> {
        XYZ {
            x: self.x,
            y: self.y,
            z,
        }
    }

    #[inline(always)]
    fn into_xyzw(self, z: T, w: T) -> XYZW<T> {
        XYZW {
            x: self.x,
            y: self.y,
            z,
            w,
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 2]) -> Self {
        Self { x: a[0], y: a[1] }
    }

    #[inline(always)]
    fn into_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    #[inline]
    fn min_element(self) -> T {
        self.x.min(self.y)
    }

    #[inline]
    fn max_element(self) -> T {
        self.x.max(self.y)
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl<T: NumEx> Vector3<T> for XYZ<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x)
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y)
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        Self::splat(self.z)
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
    }

    #[inline(always)]
    fn as_ref_xyz(&self) -> &XYZ<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xyz(&mut self) -> &mut XYZ<T> {
        self
    }

    #[inline(always)]
    fn into_xy(self) -> XY<T> {
        XY {
            x: self.x,
            y: self.y,
        }
    }

    #[inline(always)]
    fn into_xyzw(self, w: T) -> XYZW<T> {
        XYZW {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 3]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }

    #[inline(always)]
    fn into_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    #[inline]
    fn min_element(self) -> T {
        self.x.min(self.y.min(self.z))
    }

    #[inline]
    fn max_element(self) -> T {
        self.x.max(self.y.max(self.z))
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[inline]
    fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }
}

impl<T: NumEx> Vector4<T> for XYZW<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x)
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y)
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        Self::splat(self.z)
    }

    #[inline(always)]
    fn splat_w(self) -> Self {
        Self::splat(self.w)
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
        slice[3] = self.w;
    }

    #[inline(always)]
    fn as_ref_xyzw(&self) -> &XYZW<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xyzw(&mut self) -> &mut XYZW<T> {
        self
    }

    #[inline(always)]
    fn into_xy(self) -> XY<T> {
        XY {
            x: self.x,
            y: self.y,
        }
    }

    #[inline(always)]
    fn into_xyz(self) -> XYZ<T> {
        XYZ {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 4]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
            w: a[3],
        }
    }

    #[inline(always)]
    fn into_array(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn min_element(self) -> T {
        self.x.min(self.y.min(self.z.min(self.w)))
    }

    #[inline]
    fn max_element(self) -> T {
        self.x.max(self.y.max(self.z.min(self.w)))
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }
}

impl<T: SignedEx> SignedVector<T> for XY<T> {
    #[inline]
    fn neg(self) -> Self {
        self.map(|a| a.neg())
    }
}

impl<T: SignedEx> SignedVector2<T> for XY<T> {
    #[inline]
    fn abs(self) -> Self {
        self.map(|a| a.abs())
    }

    #[inline]
    fn signum(self) -> Self {
        self.map(|a| a.signum())
    }

    #[inline]
    fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    #[inline]
    fn perp_dot(self, other: Self) -> T {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl<T: SignedEx> SignedVector<T> for XYZ<T> {
    #[inline]
    fn neg(self) -> Self {
        self.map(|a| a.neg())
    }
}

impl<T: SignedEx> SignedVector3<T> for XYZ<T> {
    #[inline]
    fn abs(self) -> Self {
        self.map(|a| a.abs())
    }

    #[inline]
    fn signum(self) -> Self {
        self.map(|a| a.signum())
    }
}

impl<T: SignedEx> SignedVector<T> for XYZW<T> {
    #[inline]
    fn neg(self) -> Self {
        self.map(|a| a.neg())
    }
}

impl<T: SignedEx> SignedVector4<T> for XYZW<T> {
    #[inline]
    fn abs(self) -> Self {
        self.map(|a| a.abs())
    }

    #[inline]
    fn signum(self) -> Self {
        self.map(|a| a.signum())
    }
}

impl<T: FloatEx> FloatVector2<T> for XY<T> {
    #[inline]
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    #[inline]
    fn is_nan_mask(self) -> Self::Mask {
        self.map(|a| a.is_nan())
    }

    #[inline]
    fn floor(self) -> Self {
        self.map(Float::floor)
    }

    #[inline]
    fn ceil(self) -> Self {
        self.map(Float::ceil)
    }

    #[inline]
    fn round(self) -> Self {
        self.map(Float::round)
    }

    #[inline]
    fn recip(self) -> Self {
        self.map(Float::recip)
    }

    #[inline]
    fn exp(self) -> Self {
        self.map(Float::exp)
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        self.map(|a| a.powf(n))
    }
}

impl<T: FloatEx> FloatVector3<T> for XYZ<T> {
    #[inline]
    fn floor(self) -> Self {
        self.map(Float::floor)
    }

    #[inline]
    fn ceil(self) -> Self {
        self.map(Float::ceil)
    }

    #[inline]
    fn round(self) -> Self {
        self.map(Float::round)
    }

    #[inline]
    fn recip(self) -> Self {
        self.map(Float::recip)
    }

    #[inline]
    fn exp(self) -> Self {
        self.map(Float::exp)
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        self.map(|a| a.powf(n))
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
    #[inline]
    fn is_nan_mask(self) -> Self::Mask {
        self.map(|a| MaskConst::MASK[a.is_nan() as usize])
    }
}

impl<T: FloatEx> FloatVector4<T> for XYZW<T> {
    #[inline]
    fn floor(self) -> Self {
        self.map(Float::floor)
    }

    #[inline]
    fn ceil(self) -> Self {
        self.map(Float::ceil)
    }

    #[inline]
    fn round(self) -> Self {
        self.map(Float::round)
    }

    #[inline]
    fn recip(self) -> Self {
        self.map(Float::recip)
    }

    #[inline]
    fn exp(self) -> Self {
        self.map(Float::exp)
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        self.map(|a| a.powf(n))
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }
    #[inline]
    fn is_nan_mask(self) -> Self::Mask {
        self.map(|a| MaskConst::MASK[a.is_nan() as usize])
    }
}

impl<T> From<XYZW<T>> for XYZ<T> {
    #[inline(always)]
    fn from(v: XYZW<T>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl<T> From<XYZW<T>> for XY<T> {
    #[inline(always)]
    fn from(v: XYZW<T>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl<T> From<XYZ<T>> for XY<T> {
    #[inline(always)]
    fn from(v: XYZ<T>) -> Self {
        Self { x: v.x, y: v.y }
    }
}
