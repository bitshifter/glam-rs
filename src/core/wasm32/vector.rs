use core::arch::wasm32::*;
use crate::core::{
    storage::{XY, XYZ, XYZW},
    traits::{scalar::*, vector::*},
};
use core::mem::MaybeUninit;

impl MaskVectorConst for v128 {
    const FALSE: v128 = const_v128!([0.0; 4]);
}

impl MaskVector for v128 {
    #[inline(always)]
    fn bitand(self, other: Self) -> Self {
        v128_and(self, other)
    }

    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        v128_or(self, other)
    }

    #[inline]
    fn not(self) -> Self {
        v128_not(self)
    }
}

impl MaskVector3 for v128 {
    #[inline(always)]
    fn new(x: bool, y: bool, z: bool) -> Self {
        u32x4(
            MaskConst::MASK[x as usize],
            MaskConst::MASK[y as usize],
            MaskConst::MASK[z as usize],
            0,
        )
    }

    #[inline(always)]
    fn bitmask(self) -> u32 {
        (u32x4_bitmask(self) & 0x7) as u32
    }

    #[inline(always)]
    fn any(self) -> bool {
        (u32x4_bitmask(self) & 0x7) != 0
    }

    #[inline(always)]
    fn all(self) -> bool {
        (u32x4_bitmask(self) & 0x7) == 0x7
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 3] {
        let bitmask = MaskVector3::bitmask(self);
        [(bitmask & 1) != 0, (bitmask & 2) != 0, (bitmask & 4) != 0]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 3] {
        let bitmask = MaskVector3::bitmask(self);
        [
            MaskConst::MASK[(bitmask & 1) as usize],
            MaskConst::MASK[((bitmask >> 1) & 1) as usize],
            MaskConst::MASK[((bitmask >> 2) & 1) as usize],
        ]
    }
}

impl MaskVector4 for v128 {
    #[inline(always)]
    fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        u32x4(
            MaskConst::MASK[x as usize],
            MaskConst::MASK[y as usize],
            MaskConst::MASK[z as usize],
            MaskConst::MASK[w as usize],
        )
    }

    #[inline(always)]
    fn bitmask(self) -> u32 {
        u32x4_bitmask(self) as u32
    }

    #[inline(always)]
    fn any(self) -> bool {
        u32x4_bitmask(self) != 0
    }

    #[inline(always)]
    fn all(self) -> bool {
        u32x4_bitmask(self) == 0xf
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 4] {
        let bitmask = MaskVector4::bitmask(self);
        [
            (bitmask & 1) != 0,
            (bitmask & 2) != 0,
            (bitmask & 4) != 0,
            (bitmask & 8) != 0,
        ]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 4] {
        let bitmask = MaskVector4::bitmask(self);
        [
            MaskConst::MASK[(bitmask & 1) as usize],
            MaskConst::MASK[((bitmask >> 1) & 1) as usize],
            MaskConst::MASK[((bitmask >> 2) & 1) as usize],
            MaskConst::MASK[((bitmask >> 3) & 1) as usize],
        ]
    }
}

impl VectorConst for v128 {
    const ZERO: v128 = const_v128!([0.0; 4]);
    const ONE: v128 = const_v128!([1.0; 4]);
}

impl Vector3Const for v128 {
    const X: v128 = const_v128!([1.0, 0.0, 0.0, 0.0]);
    const Y: v128 = const_v128!([0.0, 1.0, 0.0, 0.0]);
    const Z: v128 = const_v128!([0.0, 0.0, 1.0, 0.0]);
}

impl Vector4Const for v128 {
    const X: v128 = const_v128!([1.0, 0.0, 0.0, 0.0]);
    const Y: v128 = const_v128!([0.0, 1.0, 0.0, 0.0]);
    const Z: v128 = const_v128!([0.0, 0.0, 1.0, 0.0]);
    const W: v128 = const_v128!([0.0, 0.0, 0.0, 1.0]);
}

impl Vector<f32> for v128 {
    type Mask = v128;

    #[inline(always)]
    fn splat(s: f32) -> Self {
        f32x4_splat(s)
    }

    #[inline(always)]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        v128_bitselect(if_true, if_false, mask)
    }

    #[inline(always)]
    fn cmpeq(self, other: Self) -> Self::Mask {
        f32x4_eq(self, other)
    }

    #[inline(always)]
    fn cmpne(self, other: Self) -> Self::Mask {
        f32x4_ne(self, other)
    }

    #[inline(always)]
    fn cmpge(self, other: Self) -> Self::Mask {
        f32x4_ge(self, other)
    }

    #[inline(always)]
    fn cmpgt(self, other: Self) -> Self::Mask {
        f32x4_gt(self, other)
    }

    #[inline(always)]
    fn cmple(self, other: Self) -> Self::Mask {
        f32x4_le(self, other)
    }

    #[inline(always)]
    fn cmplt(self, other: Self) -> Self::Mask {
        f32x4_lt(self, other)
    }

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        f32x4_add(self, other)
    }

    #[inline(always)]
    fn div(self, other: Self) -> Self {
        f32x4_div(self, other)
    }

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        f32x4_mul(self, other)
    }

    #[inline(always)]
    fn mul_add(self, b: Self, c: Self) -> Self {
        f32x4_add(f32x4_mul(self, b), c)
    }

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        f32x4_sub(self, other)
    }

    #[inline(always)]
    fn add_scalar(self, other: f32) -> Self {
        f32x4_add(self, f32x4_splat(other))
    }

    #[inline(always)]
    fn sub_scalar(self, other: f32) -> Self {
        f32x4_sub(self, f32x4_splat(other))
    }

    #[inline(always)]
    fn mul_scalar(self, other: f32) -> Self {
        f32x4_mul(self, f32x4_splat(other))
    }

    #[inline(always)]
    fn div_scalar(self, other: f32) -> Self {
        f32x4_div(self, f32x4_splat(other))
    }

    #[inline(always)]
    fn rem(self, other: Self) -> Self {
        let n = f32x4_floor(f32x4_div(self, other));
        f32x4_sub(self, f32x4_mul(n, other))
    }

    #[inline(always)]
    fn rem_scalar(self, other: f32) -> Self {
        self.rem(f32x4_splat(other))
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f32x4_pmin(self, other)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f32x4_pmax(self, other)
    }
}

impl Vector3<f32> for v128 {
    #[inline(always)]
    fn new(x: f32, y: f32, z: f32) -> Self {
        f32x4(x, y, z, x)
    }

    #[inline(always)]
    fn x(self) -> f32 {
        f32x4_extract_lane::<0>(self)
    }

    #[inline(always)]
    fn y(self) -> f32 {
        f32x4_extract_lane::<1>(self)
    }

    #[inline(always)]
    fn z(self) -> f32 {
        f32x4_extract_lane::<2>(self)
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        i32x4_shuffle::<0, 0, 0, 0>(self, self)
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        i32x4_shuffle::<1, 1, 1, 1>(self, self)
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        i32x4_shuffle::<2, 2, 2, 2>(self, self)
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[f32]) -> Self {
        Vector3::new(slice[0], slice[1], slice[2])
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        let xyz = self.as_ref_xyz();
        slice[0] = xyz.x;
        slice[1] = xyz.y;
        slice[2] = xyz.z;
    }

    #[inline(always)]
    fn as_ref_xyz(&self) -> &XYZ<f32> {
        unsafe { &*(self as *const Self as *const XYZ<f32>) }
    }

    #[inline(always)]
    fn as_mut_xyz(&mut self) -> &mut XYZ<f32> {
        unsafe { &mut *(self as *mut Self as *mut XYZ<f32>) }
    }

    #[inline(always)]
    fn into_xy(self) -> XY<f32> {
        XY {
            x: f32x4_extract_lane::<0>(self),
            y: f32x4_extract_lane::<1>(self),
        }
    }

    #[inline]
    fn into_xyzw(self, w: f32) -> XYZW<f32> {
        let v = f32x4_replace_lane::<3>(self, w);
        unsafe { *(&v as *const v128 as *const XYZW<f32>) }
    }

    #[inline(always)]
    fn from_array(a: [f32; 3]) -> Self {
        Vector3::new(a[0], a[1], a[2])
    }

    #[inline(always)]
    fn into_array(self) -> [f32; 3] {
        let mut out: MaybeUninit<v128> = MaybeUninit::uninit();
        unsafe {
            v128_store(out.as_mut_ptr(), self);
            *(&out.assume_init() as *const v128 as *const [f32; 3])
        }
    }

    #[inline(always)]
    fn from_tuple(t: (f32, f32, f32)) -> Self {
        Vector3::new(t.0, t.1, t.2)
    }

    #[inline(always)]
    fn into_tuple(self) -> (f32, f32, f32) {
        let mut out: MaybeUninit<v128> = MaybeUninit::uninit();
        unsafe {
            v128_store(out.as_mut_ptr(), self);
            *(&out.assume_init() as *const v128 as *const (f32, f32, f32))
        }
    }

    #[inline]
    fn min_element(self) -> f32 {
        let v = self;
        let v = f32x4_pmin(v, i32x4_shuffle::<2, 2, 1, 1>(v, v));
        let v = f32x4_pmin(v, i32x4_shuffle::<1, 0, 0, 0>(v, v));
        f32x4_extract_lane::<0>(v)
    }

    #[inline]
    fn max_element(self) -> f32 {
        let v = self;
        let v = f32x4_pmax(v, i32x4_shuffle::<2, 2, 0, 0>(v, v));
        let v = f32x4_pmax(v, i32x4_shuffle::<1, 0, 0, 0>(v, v));
        f32x4_extract_lane::<0>(v)
    }

    #[inline]
    fn dot(self, other: Self) -> f32 {
        let v = f32x4_mul(self, other);
        f32x4_extract_lane::<0>(v) + f32x4_extract_lane::<1>(v) + f32x4_extract_lane::<2>(v)
    }

    #[inline]
    fn cross(self, other: Self) -> Self {
        // x  <-  a.y*b.z - a.z*b.y
        // y  <-  a.z*b.x - a.x*b.z
        // z  <-  a.x*b.y - a.y*b.x
        // We can save a shuffle by grouping it in this wacky order:
        // (self.zxy() * other - self * other.zxy()).zxy()
        let lhszxy = i32x4_shuffle::<2, 0, 1, 1>(self, self);
        let rhszxy = i32x4_shuffle::<2, 0, 1, 1>(other, other);
        let lhszxy_rhs = f32x4_mul(lhszxy, other);
        let rhszxy_lhs = f32x4_mul(rhszxy, self);
        let sub = f32x4_sub(lhszxy_rhs, rhszxy_lhs);
        i32x4_shuffle::<2, 0, 1, 1>(sub, sub)
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(
            MaskVector3::all(min.cmple(max)),
            "clamp: expected min <= max"
        );
        self.max(min).min(max)
    }
}

impl From<XYZW<f32>> for v128 {
    #[inline(always)]
    fn from(v: XYZW<f32>) -> v128 {
        f32x4(v.x, v.y, v.z, v.w)
    }
}

impl From<XYZ<f32>> for v128 {
    #[inline(always)]
    fn from(v: XYZ<f32>) -> v128 {
        f32x4(v.x, v.y, v.z, v.z)
    }
}

impl From<XY<f32>> for v128 {
    #[inline(always)]
    fn from(v: XY<f32>) -> v128 {
        f32x4(v.x, v.y, v.y, v.y)
    }
}

impl From<v128> for XYZW<f32> {
    #[inline(always)]
    fn from(v: v128) -> XYZW<f32> {
        let mut out: MaybeUninit<v128> = MaybeUninit::uninit();
        unsafe {
            v128_store(out.as_mut_ptr(), v);
            *(&out.assume_init() as *const v128 as *const XYZW<f32>)
        }
    }
}

impl From<v128> for XYZ<f32> {
    #[inline(always)]
    fn from(v: v128) -> XYZ<f32> {
        let mut out: MaybeUninit<v128> = MaybeUninit::uninit();
        unsafe {
            v128_store(out.as_mut_ptr(), v);
            *(&out.assume_init() as *const v128 as *const XYZ<f32>)
        }
    }
}

impl From<v128> for XY<f32> {
    #[inline(always)]
    fn from(v: v128) -> XY<f32> {
        let mut out: MaybeUninit<v128> = MaybeUninit::uninit();
        unsafe {
            v128_store(out.as_mut_ptr(), v);
            *(&out.assume_init() as *const v128 as *const XY<f32>)
        }
    }
}
