// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U8Vec2, U8Vec3, U8Vec4, Vec3Swizzles};

impl Vec3Swizzles for U8Vec3 {
    type Vec2 = U8Vec2;

    type Vec4 = U8Vec4;

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
        Self::new(rhs.x, rhs.y, self.z)
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
        Self::new(rhs.x, self.y, rhs.y)
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
        Self::new(rhs.y, rhs.x, self.z)
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
        Self::new(self.x, rhs.x, rhs.y)
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
        Self::new(rhs.y, self.y, rhs.x)
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
        Self::new(self.x, rhs.y, rhs.x)
    }

    #[inline]
    fn zz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.z,
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
    fn xyx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.y)
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
    fn xzz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.z)
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
    fn yzx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.x)
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
    fn zxx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    fn zxy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    fn zxz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    fn zyx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.x)
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
}
