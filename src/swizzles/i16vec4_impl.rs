// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{I16Vec2, I16Vec3, I16Vec4, Vec4Swizzles};

impl Vec4Swizzles for I16Vec4 {
    type Vec2 = I16Vec2;

    type Vec3 = I16Vec3;

    #[inline]
    fn xx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn with_xy(self, rhs: I16Vec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z, self.w)
    }

    #[inline]
    fn xz(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: I16Vec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y, self.w)
    }

    #[inline]
    fn xw(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn with_xw(self, rhs: I16Vec2) -> Self {
        Self::new(rhs.x, self.y, self.z, rhs.y)
    }

    #[inline]
    fn yx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: I16Vec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z, self.w)
    }

    #[inline]
    fn yy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn with_yz(self, rhs: I16Vec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yw(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn with_yw(self, rhs: I16Vec2) -> Self {
        Self::new(self.x, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn zx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: I16Vec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x, self.w)
    }

    #[inline]
    fn zy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: I16Vec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zz(self) -> I16Vec2 {
        I16Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> I16Vec2 {
        I16Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn with_zw(self, rhs: I16Vec2) -> Self {
        Self::new(self.x, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn wx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn with_wx(self, rhs: I16Vec2) -> Self {
        Self::new(rhs.y, self.y, self.z, rhs.x)
    }

    #[inline]
    fn wy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn with_wy(self, rhs: I16Vec2) -> Self {
        Self::new(self.x, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wz(self) -> I16Vec2 {
        I16Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn with_wz(self, rhs: I16Vec2) -> Self {
        Self::new(self.x, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn ww(self) -> I16Vec2 {
        I16Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xxz(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    fn xxw(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.w)
    }

    #[inline]
    fn xyx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn xyz(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    fn with_xyz(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.x, rhs.y, rhs.z, self.w)
    }

    #[inline]
    fn xyw(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.w)
    }

    #[inline]
    fn with_xyw(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.x, rhs.y, self.z, rhs.z)
    }

    #[inline]
    fn xzx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    fn xzy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    fn with_xzy(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.x, rhs.z, rhs.y, self.w)
    }

    #[inline]
    fn xzz(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    fn xzw(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.z, self.w)
    }

    #[inline]
    fn with_xzw(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.x, self.y, rhs.y, rhs.z)
    }

    #[inline]
    fn xwx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.w, self.x)
    }

    #[inline]
    fn xwy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.w, self.y)
    }

    #[inline]
    fn with_xwy(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.x, rhs.z, self.z, rhs.y)
    }

    #[inline]
    fn xwz(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.w, self.z)
    }

    #[inline]
    fn with_xwz(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.x, self.y, rhs.z, rhs.y)
    }

    #[inline]
    fn xww(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.w, self.w)
    }

    #[inline]
    fn yxx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yxz(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    fn with_yxz(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.y, rhs.x, rhs.z, self.w)
    }

    #[inline]
    fn yxw(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.w)
    }

    #[inline]
    fn with_yxw(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.y, rhs.x, self.z, rhs.z)
    }

    #[inline]
    fn yyx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn yyz(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    fn yyw(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.w)
    }

    #[inline]
    fn yzx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    fn with_yzx(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.z, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yzy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    fn yzz(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    fn yzw(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.z, self.w)
    }

    #[inline]
    fn with_yzw(self, rhs: I16Vec3) -> Self {
        Self::new(self.x, rhs.x, rhs.y, rhs.z)
    }

    #[inline]
    fn ywx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.w, self.x)
    }

    #[inline]
    fn with_ywx(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.z, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn ywy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.w, self.y)
    }

    #[inline]
    fn ywz(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.w, self.z)
    }

    #[inline]
    fn with_ywz(self, rhs: I16Vec3) -> Self {
        Self::new(self.x, rhs.x, rhs.z, rhs.y)
    }

    #[inline]
    fn yww(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.w, self.w)
    }

    #[inline]
    fn zxx(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    fn zxy(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    fn with_zxy(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.y, rhs.z, rhs.x, self.w)
    }

    #[inline]
    fn zxz(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    fn zxw(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.x, self.w)
    }

    #[inline]
    fn with_zxw(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.y, self.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zyx(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    fn with_zyx(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.z, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zyy(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    fn zyz(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    fn zyw(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.y, self.w)
    }

    #[inline]
    fn with_zyw(self, rhs: I16Vec3) -> Self {
        Self::new(self.x, rhs.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zzx(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    fn zzy(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    fn zzz(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    fn zzw(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.z, self.w)
    }

    #[inline]
    fn zwx(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.w, self.x)
    }

    #[inline]
    fn with_zwx(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.z, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn zwy(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.w, self.y)
    }

    #[inline]
    fn with_zwy(self, rhs: I16Vec3) -> Self {
        Self::new(self.x, rhs.z, rhs.x, rhs.y)
    }

    #[inline]
    fn zwz(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.w, self.z)
    }

    #[inline]
    fn zww(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.w, self.w)
    }

    #[inline]
    fn wxx(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.x, self.x)
    }

    #[inline]
    fn wxy(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.x, self.y)
    }

    #[inline]
    fn with_wxy(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.y, rhs.z, self.z, rhs.x)
    }

    #[inline]
    fn wxz(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.x, self.z)
    }

    #[inline]
    fn with_wxz(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.y, self.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wxw(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.x, self.w)
    }

    #[inline]
    fn wyx(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.y, self.x)
    }

    #[inline]
    fn with_wyx(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.z, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wyy(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.y, self.y)
    }

    #[inline]
    fn wyz(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.y, self.z)
    }

    #[inline]
    fn with_wyz(self, rhs: I16Vec3) -> Self {
        Self::new(self.x, rhs.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wyw(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.y, self.w)
    }

    #[inline]
    fn wzx(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.z, self.x)
    }

    #[inline]
    fn with_wzx(self, rhs: I16Vec3) -> Self {
        Self::new(rhs.z, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn wzy(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.z, self.y)
    }

    #[inline]
    fn with_wzy(self, rhs: I16Vec3) -> Self {
        Self::new(self.x, rhs.z, rhs.y, rhs.x)
    }

    #[inline]
    fn wzz(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.z, self.z)
    }

    #[inline]
    fn wzw(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.z, self.w)
    }

    #[inline]
    fn wwx(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.w, self.x)
    }

    #[inline]
    fn wwy(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.w, self.y)
    }

    #[inline]
    fn wwz(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.w, self.z)
    }

    #[inline]
    fn www(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.w, self.w)
    }

    #[inline]
    fn xxxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xywx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.w, self.w)
    }
}
