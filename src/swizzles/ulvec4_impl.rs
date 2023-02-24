// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{ULVec2, ULVec3, ULVec4, Vec4Swizzles};

impl Vec4Swizzles for ULVec4 {
    type Vec2 = ULVec2;

    type Vec3 = ULVec3;

    #[inline]
    fn xx(self) -> ULVec2 {
        ULVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> ULVec2 {
        ULVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> ULVec2 {
        ULVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn xw(self) -> ULVec2 {
        ULVec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn yx(self) -> ULVec2 {
        ULVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> ULVec2 {
        ULVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> ULVec2 {
        ULVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn yw(self) -> ULVec2 {
        ULVec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn zx(self) -> ULVec2 {
        ULVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> ULVec2 {
        ULVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> ULVec2 {
        ULVec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> ULVec2 {
        ULVec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn wx(self) -> ULVec2 {
        ULVec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn wy(self) -> ULVec2 {
        ULVec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn wz(self) -> ULVec2 {
        ULVec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn ww(self) -> ULVec2 {
        ULVec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xxz(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn xxw(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn xyx(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xyz(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn xyw(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn xzx(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn xzy(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn xzz(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn xzw(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn xwx(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn xwy(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn xwz(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn xww(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn yxx(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yxz(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn yxw(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn yyx(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yyz(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn yyw(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn yzx(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn yzy(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn yzz(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn yzw(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn ywx(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn ywy(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn ywz(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn yww(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn zxx(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn zxy(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn zxz(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn zxw(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn zyx(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn zyy(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn zyz(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn zyw(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn zzx(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn zzy(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn zzz(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn zzw(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn zwx(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn zwy(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn zwz(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn zww(self) -> ULVec3 {
        ULVec3 {
            x: self.z,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn wxx(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn wxy(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn wxz(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn wxw(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn wyx(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn wyy(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn wyz(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn wyw(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn wzx(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn wzy(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn wzz(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn wzw(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn wwx(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn wwy(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn wwz(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn www(self) -> ULVec3 {
        ULVec3 {
            x: self.w,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn xxxx(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xyzw(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn xywx(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> ULVec4 {
        ULVec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> ULVec4 {
        ULVec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> ULVec4 {
        ULVec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> ULVec4 {
        ULVec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> ULVec4 {
        ULVec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> ULVec4 {
        ULVec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> ULVec4 {
        ULVec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> ULVec4 {
        ULVec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> ULVec4 {
        ULVec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> ULVec4 {
        ULVec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> ULVec4 {
        ULVec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> ULVec4 {
        ULVec4::new(self.w, self.w, self.w, self.w)
    }
}
