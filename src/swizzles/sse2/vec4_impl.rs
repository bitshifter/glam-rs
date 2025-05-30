// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

use crate::{Vec2, Vec3, Vec4, Vec4Swizzles};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

impl Vec4Swizzles for Vec4 {
    type Vec2 = Vec2;

    type Vec3 = Vec3;

    #[inline]
    fn xx(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn with_xy(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z, self.w)
    }

    #[inline]
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y, self.w)
    }

    #[inline]
    fn xw(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn with_xw(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, self.y, self.z, rhs.y)
    }

    #[inline]
    fn yx(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z, self.w)
    }

    #[inline]
    fn yy(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn with_yz(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yw(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn with_yw(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn zx(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x, self.w)
    }

    #[inline]
    fn zy(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zz(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn with_zw(self, rhs: Vec2) -> Self {
        Self::new(self.x, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn wx(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn with_wx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, self.y, self.z, rhs.x)
    }

    #[inline]
    fn wy(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn with_wy(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wz(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn with_wz(self, rhs: Vec2) -> Self {
        Self::new(self.x, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn ww(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xxz(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    fn xxw(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.w)
    }

    #[inline]
    fn xyx(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    fn with_xyz(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, rhs.y, rhs.z, self.w)
    }

    #[inline]
    fn xyw(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.w)
    }

    #[inline]
    fn with_xyw(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, rhs.y, self.z, rhs.z)
    }

    #[inline]
    fn xzx(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    fn xzy(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    fn with_xzy(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, rhs.z, rhs.y, self.w)
    }

    #[inline]
    fn xzz(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    fn xzw(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.w)
    }

    #[inline]
    fn with_xzw(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, self.y, rhs.y, rhs.z)
    }

    #[inline]
    fn xwx(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.x)
    }

    #[inline]
    fn xwy(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.y)
    }

    #[inline]
    fn with_xwy(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, rhs.z, self.z, rhs.y)
    }

    #[inline]
    fn xwz(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.z)
    }

    #[inline]
    fn with_xwz(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, self.y, rhs.z, rhs.y)
    }

    #[inline]
    fn xww(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.w)
    }

    #[inline]
    fn yxx(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yxz(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    fn with_yxz(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, rhs.x, rhs.z, self.w)
    }

    #[inline]
    fn yxw(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.w)
    }

    #[inline]
    fn with_yxw(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, rhs.x, self.z, rhs.z)
    }

    #[inline]
    fn yyx(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn yyz(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    fn yyw(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.w)
    }

    #[inline]
    fn yzx(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    fn with_yzx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yzy(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    fn yzz(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    fn yzw(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.w)
    }

    #[inline]
    fn with_yzw(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.x, rhs.y, rhs.z)
    }

    #[inline]
    fn ywx(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.x)
    }

    #[inline]
    fn with_ywx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn ywy(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.y)
    }

    #[inline]
    fn ywz(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.z)
    }

    #[inline]
    fn with_ywz(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.x, rhs.z, rhs.y)
    }

    #[inline]
    fn yww(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.w)
    }

    #[inline]
    fn zxx(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    fn zxy(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    fn with_zxy(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, rhs.z, rhs.x, self.w)
    }

    #[inline]
    fn zxz(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    fn zxw(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.w)
    }

    #[inline]
    fn with_zxw(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, self.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zyx(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    fn with_zyx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zyy(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    fn zyz(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    fn zyw(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.w)
    }

    #[inline]
    fn with_zyw(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zzx(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    fn zzy(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    fn zzz(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    fn zzw(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.w)
    }

    #[inline]
    fn zwx(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.x)
    }

    #[inline]
    fn with_zwx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn zwy(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.y)
    }

    #[inline]
    fn with_zwy(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.z, rhs.x, rhs.y)
    }

    #[inline]
    fn zwz(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.z)
    }

    #[inline]
    fn zww(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.w)
    }

    #[inline]
    fn wxx(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.x)
    }

    #[inline]
    fn wxy(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.y)
    }

    #[inline]
    fn with_wxy(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, rhs.z, self.z, rhs.x)
    }

    #[inline]
    fn wxz(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.z)
    }

    #[inline]
    fn with_wxz(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, self.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wxw(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.w)
    }

    #[inline]
    fn wyx(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.x)
    }

    #[inline]
    fn with_wyx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wyy(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.y)
    }

    #[inline]
    fn wyz(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.z)
    }

    #[inline]
    fn with_wyz(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wyw(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.w)
    }

    #[inline]
    fn wzx(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.x)
    }

    #[inline]
    fn with_wzx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn wzy(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.y)
    }

    #[inline]
    fn with_wzy(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.z, rhs.y, rhs.x)
    }

    #[inline]
    fn wzz(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.z)
    }

    #[inline]
    fn wzw(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.w)
    }

    #[inline]
    fn wwx(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.x)
    }

    #[inline]
    fn wwy(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.y)
    }

    #[inline]
    fn wwz(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.z)
    }

    #[inline]
    fn www(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.w)
    }

    #[inline]
    fn xxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_00) })
    }

    #[inline]
    fn xxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_00) })
    }

    #[inline]
    fn xxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_00) })
    }

    #[inline]
    fn xxxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_00_00) })
    }

    #[inline]
    fn xxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_00) })
    }

    #[inline]
    fn xxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_00) })
    }

    #[inline]
    fn xxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_00) })
    }

    #[inline]
    fn xxyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_00_00) })
    }

    #[inline]
    fn xxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_00) })
    }

    #[inline]
    fn xxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_00) })
    }

    #[inline]
    fn xxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_00) })
    }

    #[inline]
    fn xxzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_00_00) })
    }

    #[inline]
    fn xxwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_00_00) })
    }

    #[inline]
    fn xxwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_00_00) })
    }

    #[inline]
    fn xxwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_00_00) })
    }

    #[inline]
    fn xxww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_00_00) })
    }

    #[inline]
    fn xyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_00) })
    }

    #[inline]
    fn xyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_00) })
    }

    #[inline]
    fn xyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_00) })
    }

    #[inline]
    fn xyxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_01_00) })
    }

    #[inline]
    fn xyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_00) })
    }

    #[inline]
    fn xyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_00) })
    }

    #[inline]
    fn xyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_00) })
    }

    #[inline]
    fn xyyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_01_00) })
    }

    #[inline]
    fn xyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_00) })
    }

    #[inline]
    fn xyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_00) })
    }

    #[inline]
    fn xyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_00) })
    }

    #[inline]
    fn xywx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_01_00) })
    }

    #[inline]
    fn xywy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_01_00) })
    }

    #[inline]
    fn xywz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_01_00) })
    }

    #[inline]
    fn xyww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_01_00) })
    }

    #[inline]
    fn xzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_00) })
    }

    #[inline]
    fn xzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_00) })
    }

    #[inline]
    fn xzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_00) })
    }

    #[inline]
    fn xzxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_10_00) })
    }

    #[inline]
    fn xzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_00) })
    }

    #[inline]
    fn xzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_00) })
    }

    #[inline]
    fn xzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_00) })
    }

    #[inline]
    fn xzyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_00) })
    }

    #[inline]
    fn xzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_00) })
    }

    #[inline]
    fn xzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_00) })
    }

    #[inline]
    fn xzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_00) })
    }

    #[inline]
    fn xzzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_10_00) })
    }

    #[inline]
    fn xzwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_10_00) })
    }

    #[inline]
    fn xzwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_10_00) })
    }

    #[inline]
    fn xzwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_10_00) })
    }

    #[inline]
    fn xzww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_10_00) })
    }

    #[inline]
    fn xwxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_11_00) })
    }

    #[inline]
    fn xwxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_11_00) })
    }

    #[inline]
    fn xwxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_11_00) })
    }

    #[inline]
    fn xwxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_11_00) })
    }

    #[inline]
    fn xwyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_11_00) })
    }

    #[inline]
    fn xwyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_11_00) })
    }

    #[inline]
    fn xwyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_11_00) })
    }

    #[inline]
    fn xwyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_11_00) })
    }

    #[inline]
    fn xwzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_11_00) })
    }

    #[inline]
    fn xwzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_11_00) })
    }

    #[inline]
    fn xwzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_11_00) })
    }

    #[inline]
    fn xwzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_11_00) })
    }

    #[inline]
    fn xwwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_11_00) })
    }

    #[inline]
    fn xwwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_11_00) })
    }

    #[inline]
    fn xwwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_11_00) })
    }

    #[inline]
    fn xwww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_11_00) })
    }

    #[inline]
    fn yxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_01) })
    }

    #[inline]
    fn yxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_01) })
    }

    #[inline]
    fn yxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_01) })
    }

    #[inline]
    fn yxxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_00_01) })
    }

    #[inline]
    fn yxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_01) })
    }

    #[inline]
    fn yxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_01) })
    }

    #[inline]
    fn yxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_01) })
    }

    #[inline]
    fn yxyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_00_01) })
    }

    #[inline]
    fn yxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_01) })
    }

    #[inline]
    fn yxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_01) })
    }

    #[inline]
    fn yxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_01) })
    }

    #[inline]
    fn yxzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_00_01) })
    }

    #[inline]
    fn yxwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_00_01) })
    }

    #[inline]
    fn yxwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_00_01) })
    }

    #[inline]
    fn yxwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_00_01) })
    }

    #[inline]
    fn yxww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_00_01) })
    }

    #[inline]
    fn yyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_01) })
    }

    #[inline]
    fn yyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_01) })
    }

    #[inline]
    fn yyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_01) })
    }

    #[inline]
    fn yyxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_01_01) })
    }

    #[inline]
    fn yyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_01) })
    }

    #[inline]
    fn yyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_01) })
    }

    #[inline]
    fn yyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_01) })
    }

    #[inline]
    fn yyyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_01_01) })
    }

    #[inline]
    fn yyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_01) })
    }

    #[inline]
    fn yyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_01) })
    }

    #[inline]
    fn yyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_01) })
    }

    #[inline]
    fn yyzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_01_01) })
    }

    #[inline]
    fn yywx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_01_01) })
    }

    #[inline]
    fn yywy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_01_01) })
    }

    #[inline]
    fn yywz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_01_01) })
    }

    #[inline]
    fn yyww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_01_01) })
    }

    #[inline]
    fn yzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_01) })
    }

    #[inline]
    fn yzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_01) })
    }

    #[inline]
    fn yzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_01) })
    }

    #[inline]
    fn yzxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_10_01) })
    }

    #[inline]
    fn yzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_01) })
    }

    #[inline]
    fn yzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_01) })
    }

    #[inline]
    fn yzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_01) })
    }

    #[inline]
    fn yzyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_01) })
    }

    #[inline]
    fn yzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_01) })
    }

    #[inline]
    fn yzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_01) })
    }

    #[inline]
    fn yzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_01) })
    }

    #[inline]
    fn yzzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_10_01) })
    }

    #[inline]
    fn yzwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_10_01) })
    }

    #[inline]
    fn yzwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_10_01) })
    }

    #[inline]
    fn yzwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_10_01) })
    }

    #[inline]
    fn yzww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_10_01) })
    }

    #[inline]
    fn ywxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_11_01) })
    }

    #[inline]
    fn ywxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_11_01) })
    }

    #[inline]
    fn ywxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_11_01) })
    }

    #[inline]
    fn ywxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_11_01) })
    }

    #[inline]
    fn ywyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_11_01) })
    }

    #[inline]
    fn ywyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_11_01) })
    }

    #[inline]
    fn ywyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_11_01) })
    }

    #[inline]
    fn ywyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_11_01) })
    }

    #[inline]
    fn ywzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_11_01) })
    }

    #[inline]
    fn ywzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_11_01) })
    }

    #[inline]
    fn ywzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_11_01) })
    }

    #[inline]
    fn ywzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_11_01) })
    }

    #[inline]
    fn ywwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_11_01) })
    }

    #[inline]
    fn ywwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_11_01) })
    }

    #[inline]
    fn ywwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_11_01) })
    }

    #[inline]
    fn ywww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_11_01) })
    }

    #[inline]
    fn zxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_10) })
    }

    #[inline]
    fn zxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_10) })
    }

    #[inline]
    fn zxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_10) })
    }

    #[inline]
    fn zxxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_00_10) })
    }

    #[inline]
    fn zxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_10) })
    }

    #[inline]
    fn zxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10) })
    }

    #[inline]
    fn zxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_10) })
    }

    #[inline]
    fn zxyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_00_10) })
    }

    #[inline]
    fn zxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_10) })
    }

    #[inline]
    fn zxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_10) })
    }

    #[inline]
    fn zxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_10) })
    }

    #[inline]
    fn zxzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_00_10) })
    }

    #[inline]
    fn zxwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_00_10) })
    }

    #[inline]
    fn zxwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_00_10) })
    }

    #[inline]
    fn zxwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_00_10) })
    }

    #[inline]
    fn zxww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_00_10) })
    }

    #[inline]
    fn zyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_10) })
    }

    #[inline]
    fn zyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_10) })
    }

    #[inline]
    fn zyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_10) })
    }

    #[inline]
    fn zyxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_01_10) })
    }

    #[inline]
    fn zyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_10) })
    }

    #[inline]
    fn zyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_10) })
    }

    #[inline]
    fn zyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_10) })
    }

    #[inline]
    fn zyyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_01_10) })
    }

    #[inline]
    fn zyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_10) })
    }

    #[inline]
    fn zyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_10) })
    }

    #[inline]
    fn zyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_10) })
    }

    #[inline]
    fn zyzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_01_10) })
    }

    #[inline]
    fn zywx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_01_10) })
    }

    #[inline]
    fn zywy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_01_10) })
    }

    #[inline]
    fn zywz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_01_10) })
    }

    #[inline]
    fn zyww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_01_10) })
    }

    #[inline]
    fn zzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_10) })
    }

    #[inline]
    fn zzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_10) })
    }

    #[inline]
    fn zzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_10) })
    }

    #[inline]
    fn zzxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_10_10) })
    }

    #[inline]
    fn zzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_10) })
    }

    #[inline]
    fn zzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_10) })
    }

    #[inline]
    fn zzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_10) })
    }

    #[inline]
    fn zzyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_10) })
    }

    #[inline]
    fn zzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_10) })
    }

    #[inline]
    fn zzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_10) })
    }

    #[inline]
    fn zzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_10) })
    }

    #[inline]
    fn zzzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_10_10) })
    }

    #[inline]
    fn zzwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_10_10) })
    }

    #[inline]
    fn zzwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_10_10) })
    }

    #[inline]
    fn zzwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_10_10) })
    }

    #[inline]
    fn zzww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_10_10) })
    }

    #[inline]
    fn zwxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_11_10) })
    }

    #[inline]
    fn zwxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_11_10) })
    }

    #[inline]
    fn zwxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_11_10) })
    }

    #[inline]
    fn zwxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_11_10) })
    }

    #[inline]
    fn zwyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_11_10) })
    }

    #[inline]
    fn zwyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_11_10) })
    }

    #[inline]
    fn zwyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_11_10) })
    }

    #[inline]
    fn zwyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_11_10) })
    }

    #[inline]
    fn zwzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_11_10) })
    }

    #[inline]
    fn zwzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_11_10) })
    }

    #[inline]
    fn zwzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_11_10) })
    }

    #[inline]
    fn zwzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_11_10) })
    }

    #[inline]
    fn zwwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_11_10) })
    }

    #[inline]
    fn zwwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_11_10) })
    }

    #[inline]
    fn zwwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_11_10) })
    }

    #[inline]
    fn zwww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_11_10) })
    }

    #[inline]
    fn wxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_11) })
    }

    #[inline]
    fn wxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_11) })
    }

    #[inline]
    fn wxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_11) })
    }

    #[inline]
    fn wxxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_00_11) })
    }

    #[inline]
    fn wxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_11) })
    }

    #[inline]
    fn wxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_11) })
    }

    #[inline]
    fn wxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_11) })
    }

    #[inline]
    fn wxyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_00_11) })
    }

    #[inline]
    fn wxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_11) })
    }

    #[inline]
    fn wxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_11) })
    }

    #[inline]
    fn wxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_11) })
    }

    #[inline]
    fn wxzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_00_11) })
    }

    #[inline]
    fn wxwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_00_11) })
    }

    #[inline]
    fn wxwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_00_11) })
    }

    #[inline]
    fn wxwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_00_11) })
    }

    #[inline]
    fn wxww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_00_11) })
    }

    #[inline]
    fn wyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_11) })
    }

    #[inline]
    fn wyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_11) })
    }

    #[inline]
    fn wyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_11) })
    }

    #[inline]
    fn wyxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_01_11) })
    }

    #[inline]
    fn wyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_11) })
    }

    #[inline]
    fn wyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_11) })
    }

    #[inline]
    fn wyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_11) })
    }

    #[inline]
    fn wyyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_01_11) })
    }

    #[inline]
    fn wyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_11) })
    }

    #[inline]
    fn wyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_11) })
    }

    #[inline]
    fn wyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_11) })
    }

    #[inline]
    fn wyzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_01_11) })
    }

    #[inline]
    fn wywx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_01_11) })
    }

    #[inline]
    fn wywy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_01_11) })
    }

    #[inline]
    fn wywz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_01_11) })
    }

    #[inline]
    fn wyww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_01_11) })
    }

    #[inline]
    fn wzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_11) })
    }

    #[inline]
    fn wzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_11) })
    }

    #[inline]
    fn wzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_11) })
    }

    #[inline]
    fn wzxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_10_11) })
    }

    #[inline]
    fn wzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_11) })
    }

    #[inline]
    fn wzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_11) })
    }

    #[inline]
    fn wzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_11) })
    }

    #[inline]
    fn wzyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_11) })
    }

    #[inline]
    fn wzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_11) })
    }

    #[inline]
    fn wzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_11) })
    }

    #[inline]
    fn wzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_11) })
    }

    #[inline]
    fn wzzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_10_11) })
    }

    #[inline]
    fn wzwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_10_11) })
    }

    #[inline]
    fn wzwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_10_11) })
    }

    #[inline]
    fn wzwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_10_11) })
    }

    #[inline]
    fn wzww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_10_11) })
    }

    #[inline]
    fn wwxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_11_11) })
    }

    #[inline]
    fn wwxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_11_11) })
    }

    #[inline]
    fn wwxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_11_11) })
    }

    #[inline]
    fn wwxw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_11_11) })
    }

    #[inline]
    fn wwyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_11_11) })
    }

    #[inline]
    fn wwyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_11_11) })
    }

    #[inline]
    fn wwyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_11_11) })
    }

    #[inline]
    fn wwyw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_11_11) })
    }

    #[inline]
    fn wwzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_11_11) })
    }

    #[inline]
    fn wwzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_11_11) })
    }

    #[inline]
    fn wwzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_11_11) })
    }

    #[inline]
    fn wwzw(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_11_11) })
    }

    #[inline]
    fn wwwx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_11_11) })
    }

    #[inline]
    fn wwwy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_11_11) })
    }

    #[inline]
    fn wwwz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_11_11) })
    }

    #[inline]
    fn wwww(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_11_11) })
    }
}
