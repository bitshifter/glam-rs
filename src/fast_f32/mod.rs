// taken from https://gitlab.com/kornelski/ffast-math/blob/master/src/lib.rs
use std::ops::*;
use std::fmt;
use std::fmt::Write;
use std::cmp::Ordering;

use num_traits::Float;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
/// Fast, finite, floating-point
pub struct ff32(pub f32);

/// It's necessary for literals: 0.ff32()
pub trait ToFFF32 {
    fn ff32(&self) -> ff32;
}

macro_rules! impl_cast {
    ($ty:ty) => {
        impl ToFFF32 for $ty {
            #[inline(always)]
            fn ff32(&self) -> ff32 {
                ff32(*self as f32)
            }
        }
    }
}

impl_cast! {i8}
impl_cast! {u8}
impl_cast! {i16}
impl_cast! {u16}
impl_cast! {i32}
impl_cast! {u32}
impl_cast! {f32}
impl_cast! {f64}
impl_cast! {usize}
impl_cast! {isize}

/// Not all float functions are wrapped/implemented in a wrapped/fast version,
/// but all should work by falling back to a regular f32 via Deref.
impl Float for ff32 {
    #[inline(always)]
    fn min<V: Into<f32>>(self, min: V) -> Self {
        let min = min.into();
        if !(self.0 > min) { self } else { ff32(min) }
    }

    #[inline(always)]
    fn max<V: Into<f32>>(self, max: V) -> Self {
        let max = max.into();
        if !(self.0 < max) { self } else { ff32(max) }
    }

    #[inline(always)]
    /// Always returns true
    fn is_finite(&self) -> bool {
        debug_assert!(self.0.is_finite());
        true
    }

    #[inline(always)]
    fn powf<V: Into<f32>>(self, v: V) -> Self {
        ff32(self.0.powf(v.into()))
    }

    #[inline(always)]
    fn powi(self, v: i32) -> Self {
        ff32(self.0.powi(v))
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        ff32(unsafe{std::intrinsics::sqrtf32(self.0)})
    }

    #[inline(always)]
    /// Very slow. Use trunc()
    fn round(self) -> Self {
        self.0.round().into()
    }

    #[inline(always)]
    /// Very slow. Use trunc()
    fn floor(self) -> Self {
        self.0.floor().into()
    }

    #[inline(always)]
    /// Very slow. Use trunc()
    fn ceil(self) -> Self {
        self.0.ceil().into()
    }

    #[inline(always)]
    /// Inaccurate for values that don't fit in i32
    fn trunc(self) -> Self {
        ff32(self.0 as i32 as f32)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        self.0.abs().into()
    }

    #[inline(always)]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        self.0.from_str_radix(str, radix)
    }
}

impl ff32 {
    #[inline(always)]
    fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    fn as_f32(&self) -> f32 {
        self.0
    }

    #[inline(always)]
    fn as_u16(&self) -> u16 {
        self.0 as u16
    }

    #[inline(always)]
    fn as_u8(&self) -> u8 {
        self.0 as u8
    }

}

macro_rules! impl_fast {
    ($tr:ident, $fn:ident, $func:ident) => {
        impl $tr for ff32 {
            type Output = ff32;

            #[inline(always)]
            fn $fn(self, other: ff32) -> Self::Output {
                unsafe {
                    ff32(std::intrinsics::$func(self.0, other.0))
                }
            }
        }

        impl $tr<f32> for ff32 {
            type Output = ff32;

            #[inline(always)]
            fn $fn(self, other: f32) -> Self::Output {
                unsafe {
                    std::intrinsics::$func(self.0, other).into()
                }
            }
        }

        impl $tr<ff32> for f32 {
            type Output = ff32;

            #[inline(always)]
            fn $fn(self, other: ff32) -> Self::Output {
                unsafe {
                    std::intrinsics::$func(self, other.0).into()
                }
            }
        }
    }
}

macro_rules! impl_assign {
    ($tr:ident, $func:ident, $fn:ident) => {
        impl $tr for ff32 {
            #[inline(always)]
            fn $fn(&mut self, other: ff32) {
                *self = self.$func(other)
            }
        }
    }
}

impl_fast! {Add, add, fadd_fast}
impl_assign! {AddAssign, add, add_assign}
impl_fast! {Sub, sub, fsub_fast}
impl_assign! {SubAssign, sub, sub_assign}
impl_fast! {Mul, mul, fmul_fast}
impl_fast! {Rem, rem, frem_fast}
impl_fast! {Div, div, fdiv_fast}

impl Neg for ff32 {
    type Output = ff32;
    fn neg(self) -> Self::Output {
        ff32(self.0.neg())
    }
}

impl Eq for ff32 {
}

impl PartialEq<f32> for ff32 {
    #[inline(always)]
    fn eq(&self, other: &f32) -> bool {
        self.0.eq(other)
    }
    #[inline(always)]
    fn ne(&self, other: &f32) -> bool {
        self.0.ne(other)
    }
}

impl PartialEq<ff32> for f32 {
    #[inline(always)]
    fn eq(&self, other: &ff32) -> bool {
        self.eq(&other.0)
    }
    #[inline(always)]
    fn ne(&self, other: &ff32) -> bool {
        self.ne(&other.0)
    }
}

impl Ord for ff32 {
    #[inline(always)]
    fn cmp(&self, other: &ff32) -> Ordering {
        self.0.partial_cmp(&other.0).expect("ff32")
    }
}

impl PartialOrd<f32> for ff32 {
    #[inline(always)]
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<ff32> for f32 {
    #[inline(always)]
    fn partial_cmp(&self, other: &ff32) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl From<f32> for ff32 {
    #[inline(always)]
    fn from(v: f32) -> Self {
        debug_assert!(v.is_finite());
        ff32(v)
    }
}

impl From<ff32> for f32 {
    #[inline(always)]
    fn from(v: ff32) -> Self {
        v.0
    }
}

impl Deref for ff32 {
    type Target = f32;

    #[inline(always)]
    fn deref(&self) -> &f32 {
        &self.0
    }
}

impl fmt::Display for ff32 {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

impl fmt::Debug for ff32 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)?;
        fmt.write_char('f')
    }
}

#[test]
fn add_mul() {
    let a = ff32(2.);
    let b = ff32(2.5);
    let c: ff32 = 4.5f32.into();
    let d: ff32 = 5f32.into();
    assert_eq!(c, a+b);
    assert_eq!(d, a*b);
    assert_eq!(d, b+b);
}

#[test]
fn div_sub() {
    let a = ff32(-9.);
    let b = ff32(999.);
    let c: ff32 = (-111f32).into();
    let d: ff32 = 1008f32.into();
    assert_eq!(c, b/a);
    assert_eq!(d, b-a);
    assert_eq!(9.ff32(), -a);
}

#[test]
fn deref() {
    let a = ff32(1.);
    assert_eq!(false, a.is_sign_negative());
}

#[test]
fn into_eq() {
    let a: ff32 = 3f32.into();
    let b: f32 = a.into();
    let c = b.ff32();
    assert_eq!(3., a);
    assert_eq!(c, b);
    assert_eq!(b, c);
}

#[test]
fn ord_max() {
    let a = 1f32.ff32();
    assert!(a > 0f32);
    assert!(0f32 < a);
    assert!(a < 2f32);
    assert!(a > 0.9f32);
    assert!(a > 0.9f32.ff32());

    assert_eq!(2., a.max(2.));
    assert_eq!(1., a.min(1.));
}

#[test]
fn round() {
    let a = 1.1f32.ff32();
    assert_eq!(a.round(), 1.ff32());
    assert_eq!(1.5.ff32().round(), 2.ff32());
    assert_eq!(1.5.ff32().floor(), 1.ff32());
    assert_eq!(1.4.ff32().round(), 1.ff32());
    assert_eq!(1.4.ff32().ceil(), 2.ff32());
    assert_eq!(-1.5.ff32().round(), -2.ff32());
    assert_eq!(-1.4.ff32().round(), -1.ff32());
    assert_eq!(-1.6.ff32().round(), -2.ff32());
}

#[test]
fn trunc() {
    let a = 1.1f32.ff32();
    assert_eq!(a.trunc(), 1.ff32());
    assert_eq!(1.5.ff32().trunc(), 1.ff32());
    assert_eq!(1.4.ff32().trunc(), 1.ff32());
    assert_eq!(-1.5.ff32().trunc(), -1.ff32());
    assert_eq!(-1.4.ff32().trunc(), -1.ff32());
    assert_eq!(-1.6.ff32().trunc(), -1.ff32());
}

#[test]
fn display_debug() {
    assert_eq!("1.5 -3.5f", format!("{} {:?}", ff32(1.5), ff32(-3.5)));
}

