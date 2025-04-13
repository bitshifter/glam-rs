// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{USizeVec2, USizeVec3, USizeVec4, Vec4Swizzles};

impl Vec4Swizzles for USizeVec4 {
    type Vec2 = USizeVec2;

    type Vec3 = USizeVec3;

    #[inline]
    fn xx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn with_xy(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z, self.w)
    }

    #[inline]
    fn xz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y, self.w)
    }

    #[inline]
    fn xw(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn with_xw(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.x, self.y, self.z, rhs.y)
    }

    #[inline]
    fn yx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z, self.w)
    }

    #[inline]
    fn yy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn with_yz(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yw(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn with_yw(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn zx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x, self.w)
    }

    #[inline]
    fn zy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn with_zw(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn wx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn with_wx(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.y, self.y, self.z, rhs.x)
    }

    #[inline]
    fn wy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn with_wy(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn with_wz(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn ww(self) -> USizeVec2 {
        USizeVec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xxz(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.z)
    }

    #[inline]
    fn xxw(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.w)
    }

    #[inline]
    fn xyx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn xyz(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.y, self.z)
    }

    #[inline]
    fn with_xyz(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.x, rhs.y, rhs.z, self.w)
    }

    #[inline]
    fn xyw(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.y, self.w)
    }

    #[inline]
    fn with_xyw(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.x, rhs.y, self.z, rhs.z)
    }

    #[inline]
    fn xzx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.z, self.x)
    }

    #[inline]
    fn xzy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.z, self.y)
    }

    #[inline]
    fn with_xzy(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.x, rhs.z, rhs.y, self.w)
    }

    #[inline]
    fn xzz(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.z, self.z)
    }

    #[inline]
    fn xzw(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.z, self.w)
    }

    #[inline]
    fn with_xzw(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.x, self.y, rhs.y, rhs.z)
    }

    #[inline]
    fn xwx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.w, self.x)
    }

    #[inline]
    fn xwy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.w, self.y)
    }

    #[inline]
    fn with_xwy(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.x, rhs.z, self.z, rhs.y)
    }

    #[inline]
    fn xwz(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.w, self.z)
    }

    #[inline]
    fn with_xwz(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.x, self.y, rhs.z, rhs.y)
    }

    #[inline]
    fn xww(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.w, self.w)
    }

    #[inline]
    fn yxx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yxz(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.z)
    }

    #[inline]
    fn with_yxz(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.y, rhs.x, rhs.z, self.w)
    }

    #[inline]
    fn yxw(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.w)
    }

    #[inline]
    fn with_yxw(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.y, rhs.x, self.z, rhs.z)
    }

    #[inline]
    fn yyx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn yyz(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.z)
    }

    #[inline]
    fn yyw(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.w)
    }

    #[inline]
    fn yzx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.z, self.x)
    }

    #[inline]
    fn with_yzx(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.z, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yzy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.z, self.y)
    }

    #[inline]
    fn yzz(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.z, self.z)
    }

    #[inline]
    fn yzw(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.z, self.w)
    }

    #[inline]
    fn with_yzw(self, rhs: USizeVec3) -> Self {
        Self::new(self.x, rhs.x, rhs.y, rhs.z)
    }

    #[inline]
    fn ywx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.w, self.x)
    }

    #[inline]
    fn with_ywx(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.z, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn ywy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.w, self.y)
    }

    #[inline]
    fn ywz(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.w, self.z)
    }

    #[inline]
    fn with_ywz(self, rhs: USizeVec3) -> Self {
        Self::new(self.x, rhs.x, rhs.z, rhs.y)
    }

    #[inline]
    fn yww(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.w, self.w)
    }

    #[inline]
    fn zxx(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.x, self.x)
    }

    #[inline]
    fn zxy(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.x, self.y)
    }

    #[inline]
    fn with_zxy(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.y, rhs.z, rhs.x, self.w)
    }

    #[inline]
    fn zxz(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.x, self.z)
    }

    #[inline]
    fn zxw(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.x, self.w)
    }

    #[inline]
    fn with_zxw(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.y, self.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zyx(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.y, self.x)
    }

    #[inline]
    fn with_zyx(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.z, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zyy(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.y, self.y)
    }

    #[inline]
    fn zyz(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.y, self.z)
    }

    #[inline]
    fn zyw(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.y, self.w)
    }

    #[inline]
    fn with_zyw(self, rhs: USizeVec3) -> Self {
        Self::new(self.x, rhs.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zzx(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.z, self.x)
    }

    #[inline]
    fn zzy(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.z, self.y)
    }

    #[inline]
    fn zzz(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.z, self.z)
    }

    #[inline]
    fn zzw(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.z, self.w)
    }

    #[inline]
    fn zwx(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.w, self.x)
    }

    #[inline]
    fn with_zwx(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.z, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn zwy(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.w, self.y)
    }

    #[inline]
    fn with_zwy(self, rhs: USizeVec3) -> Self {
        Self::new(self.x, rhs.z, rhs.x, rhs.y)
    }

    #[inline]
    fn zwz(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.w, self.z)
    }

    #[inline]
    fn zww(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.w, self.w)
    }

    #[inline]
    fn wxx(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.x, self.x)
    }

    #[inline]
    fn wxy(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.x, self.y)
    }

    #[inline]
    fn with_wxy(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.y, rhs.z, self.z, rhs.x)
    }

    #[inline]
    fn wxz(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.x, self.z)
    }

    #[inline]
    fn with_wxz(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.y, self.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wxw(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.x, self.w)
    }

    #[inline]
    fn wyx(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.y, self.x)
    }

    #[inline]
    fn with_wyx(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.z, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wyy(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.y, self.y)
    }

    #[inline]
    fn wyz(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.y, self.z)
    }

    #[inline]
    fn with_wyz(self, rhs: USizeVec3) -> Self {
        Self::new(self.x, rhs.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wyw(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.y, self.w)
    }

    #[inline]
    fn wzx(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.z, self.x)
    }

    #[inline]
    fn with_wzx(self, rhs: USizeVec3) -> Self {
        Self::new(rhs.z, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn wzy(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.z, self.y)
    }

    #[inline]
    fn with_wzy(self, rhs: USizeVec3) -> Self {
        Self::new(self.x, rhs.z, rhs.y, rhs.x)
    }

    #[inline]
    fn wzz(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.z, self.z)
    }

    #[inline]
    fn wzw(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.z, self.w)
    }

    #[inline]
    fn wwx(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.w, self.x)
    }

    #[inline]
    fn wwy(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.w, self.y)
    }

    #[inline]
    fn wwz(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.w, self.z)
    }

    #[inline]
    fn www(self) -> USizeVec3 {
        USizeVec3::new(self.w, self.w, self.w)
    }

    #[inline]
    fn xxxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xywx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> USizeVec4 {
        USizeVec4::new(self.w, self.w, self.w, self.w)
    }
}
