// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{LVec2, LVec3, LVec4, Vec4Swizzles};

impl Vec4Swizzles for LVec4 {
    type Vec2 = LVec2;

    type Vec3 = LVec3;

    #[inline]
    fn xx(self) -> LVec2 {
        LVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> LVec2 {
        LVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> LVec2 {
        LVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn xw(self) -> LVec2 {
        LVec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn yx(self) -> LVec2 {
        LVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> LVec2 {
        LVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> LVec2 {
        LVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn yw(self) -> LVec2 {
        LVec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn zx(self) -> LVec2 {
        LVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> LVec2 {
        LVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> LVec2 {
        LVec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> LVec2 {
        LVec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn wx(self) -> LVec2 {
        LVec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn wy(self) -> LVec2 {
        LVec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn wz(self) -> LVec2 {
        LVec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn ww(self) -> LVec2 {
        LVec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xxz(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn xxw(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn xyx(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xyz(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn xyw(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn xzx(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn xzy(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn xzz(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn xzw(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn xwx(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn xwy(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn xwz(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn xww(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn yxx(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yxz(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn yxw(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn yyx(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yyz(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn yyw(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn yzx(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn yzy(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn yzz(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn yzw(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn ywx(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn ywy(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn ywz(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn yww(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn zxx(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn zxy(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn zxz(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn zxw(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn zyx(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn zyy(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn zyz(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn zyw(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn zzx(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn zzy(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn zzz(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn zzw(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn zwx(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn zwy(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn zwz(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn zww(self) -> LVec3 {
        LVec3 {
            x: self.z,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn wxx(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn wxy(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn wxz(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn wxw(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn wyx(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn wyy(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn wyz(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn wyw(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn wzx(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn wzy(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn wzz(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn wzw(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn wwx(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn wwy(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn wwz(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn www(self) -> LVec3 {
        LVec3 {
            x: self.w,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn xxxx(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xyzw(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn xywx(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> LVec4 {
        LVec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> LVec4 {
        LVec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> LVec4 {
        LVec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> LVec4 {
        LVec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> LVec4 {
        LVec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> LVec4 {
        LVec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> LVec4 {
        LVec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> LVec4 {
        LVec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> LVec4 {
        LVec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> LVec4 {
        LVec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> LVec4 {
        LVec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> LVec4 {
        LVec4::new(self.w, self.w, self.w, self.w)
    }
}
