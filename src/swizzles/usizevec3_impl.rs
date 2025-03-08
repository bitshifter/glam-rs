// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{USizeVec2, USizeVec3, USizeVec4, Vec3Swizzles};

impl Vec3Swizzles for USizeVec3 {
    type Vec2 = USizeVec2;

    type Vec4 = USizeVec4;

    #[inline]
    #[must_use]
    fn xx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn with_xy(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn with_xz(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y)
    }

    #[inline]
    #[must_use]
    fn yx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn with_yx(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn with_yz(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y)
    }

    #[inline]
    #[must_use]
    fn zx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn with_zx(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x)
    }

    #[inline]
    #[must_use]
    fn zy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn with_zy(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x)
    }

    #[inline]
    #[must_use]
    fn zz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> USizeVec3 {
        USizeVec3::new(self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.z)
    }
}
