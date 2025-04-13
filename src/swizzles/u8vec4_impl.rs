// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U8Vec2, U8Vec3, U8Vec4, Vec4Swizzles};

impl Vec4Swizzles for U8Vec4 {
    type Vec2 = U8Vec2;

    type Vec3 = U8Vec3;

    #[inline]
    fn xx(self) -> U8Vec2 {
        U8Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> U8Vec2 {
        U8Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn with_xy(self, rhs: U8Vec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z, self.w)
    }

    #[inline]
    fn xz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: U8Vec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y, self.w)
    }

    #[inline]
    fn xw(self) -> U8Vec2 {
        U8Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn with_xw(self, rhs: U8Vec2) -> Self {
        Self::new(rhs.x, self.y, self.z, rhs.y)
    }

    #[inline]
    fn yx(self) -> U8Vec2 {
        U8Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: U8Vec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z, self.w)
    }

    #[inline]
    fn yy(self) -> U8Vec2 {
        U8Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn with_yz(self, rhs: U8Vec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yw(self) -> U8Vec2 {
        U8Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn with_yw(self, rhs: U8Vec2) -> Self {
        Self::new(self.x, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn zx(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: U8Vec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x, self.w)
    }

    #[inline]
    fn zy(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: U8Vec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn with_zw(self, rhs: U8Vec2) -> Self {
        Self::new(self.x, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn wx(self) -> U8Vec2 {
        U8Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn with_wx(self, rhs: U8Vec2) -> Self {
        Self::new(rhs.y, self.y, self.z, rhs.x)
    }

    #[inline]
    fn wy(self) -> U8Vec2 {
        U8Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn with_wy(self, rhs: U8Vec2) -> Self {
        Self::new(self.x, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn with_wz(self, rhs: U8Vec2) -> Self {
        Self::new(self.x, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn ww(self) -> U8Vec2 {
        U8Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xxz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    fn xxw(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.w)
    }

    #[inline]
    fn xyx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn xyz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    fn with_xyz(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.x, rhs.y, rhs.z, self.w)
    }

    #[inline]
    fn xyw(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.w)
    }

    #[inline]
    fn with_xyw(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.x, rhs.y, self.z, rhs.z)
    }

    #[inline]
    fn xzx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    fn xzy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    fn with_xzy(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.x, rhs.z, rhs.y, self.w)
    }

    #[inline]
    fn xzz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    fn xzw(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.w)
    }

    #[inline]
    fn with_xzw(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.x, self.y, rhs.y, rhs.z)
    }

    #[inline]
    fn xwx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.w, self.x)
    }

    #[inline]
    fn xwy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.w, self.y)
    }

    #[inline]
    fn with_xwy(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.x, rhs.z, self.z, rhs.y)
    }

    #[inline]
    fn xwz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.w, self.z)
    }

    #[inline]
    fn with_xwz(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.x, self.y, rhs.z, rhs.y)
    }

    #[inline]
    fn xww(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.w, self.w)
    }

    #[inline]
    fn yxx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yxz(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    fn with_yxz(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.y, rhs.x, rhs.z, self.w)
    }

    #[inline]
    fn yxw(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.w)
    }

    #[inline]
    fn with_yxw(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.y, rhs.x, self.z, rhs.z)
    }

    #[inline]
    fn yyx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn yyz(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    fn yyw(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.w)
    }

    #[inline]
    fn yzx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    fn with_yzx(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.z, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yzy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    fn yzz(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    fn yzw(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.w)
    }

    #[inline]
    fn with_yzw(self, rhs: U8Vec3) -> Self {
        Self::new(self.x, rhs.x, rhs.y, rhs.z)
    }

    #[inline]
    fn ywx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.w, self.x)
    }

    #[inline]
    fn with_ywx(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.z, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn ywy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.w, self.y)
    }

    #[inline]
    fn ywz(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.w, self.z)
    }

    #[inline]
    fn with_ywz(self, rhs: U8Vec3) -> Self {
        Self::new(self.x, rhs.x, rhs.z, rhs.y)
    }

    #[inline]
    fn yww(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.w, self.w)
    }

    #[inline]
    fn zxx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    fn zxy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    fn with_zxy(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.y, rhs.z, rhs.x, self.w)
    }

    #[inline]
    fn zxz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    fn zxw(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.w)
    }

    #[inline]
    fn with_zxw(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.y, self.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zyx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    fn with_zyx(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.z, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zyy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    fn zyz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    fn zyw(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.w)
    }

    #[inline]
    fn with_zyw(self, rhs: U8Vec3) -> Self {
        Self::new(self.x, rhs.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zzx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    fn zzy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    fn zzz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    fn zzw(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.z, self.w)
    }

    #[inline]
    fn zwx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.w, self.x)
    }

    #[inline]
    fn with_zwx(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.z, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn zwy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.w, self.y)
    }

    #[inline]
    fn with_zwy(self, rhs: U8Vec3) -> Self {
        Self::new(self.x, rhs.z, rhs.x, rhs.y)
    }

    #[inline]
    fn zwz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.w, self.z)
    }

    #[inline]
    fn zww(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.w, self.w)
    }

    #[inline]
    fn wxx(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.x, self.x)
    }

    #[inline]
    fn wxy(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.x, self.y)
    }

    #[inline]
    fn with_wxy(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.y, rhs.z, self.z, rhs.x)
    }

    #[inline]
    fn wxz(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.x, self.z)
    }

    #[inline]
    fn with_wxz(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.y, self.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wxw(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.x, self.w)
    }

    #[inline]
    fn wyx(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.y, self.x)
    }

    #[inline]
    fn with_wyx(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.z, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wyy(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.y, self.y)
    }

    #[inline]
    fn wyz(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.y, self.z)
    }

    #[inline]
    fn with_wyz(self, rhs: U8Vec3) -> Self {
        Self::new(self.x, rhs.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wyw(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.y, self.w)
    }

    #[inline]
    fn wzx(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.z, self.x)
    }

    #[inline]
    fn with_wzx(self, rhs: U8Vec3) -> Self {
        Self::new(rhs.z, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn wzy(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.z, self.y)
    }

    #[inline]
    fn with_wzy(self, rhs: U8Vec3) -> Self {
        Self::new(self.x, rhs.z, rhs.y, rhs.x)
    }

    #[inline]
    fn wzz(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.z, self.z)
    }

    #[inline]
    fn wzw(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.z, self.w)
    }

    #[inline]
    fn wwx(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.w, self.x)
    }

    #[inline]
    fn wwy(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.w, self.y)
    }

    #[inline]
    fn wwz(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.w, self.z)
    }

    #[inline]
    fn www(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.w, self.w)
    }

    #[inline]
    fn xxxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xywx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.w, self.w)
    }
}
