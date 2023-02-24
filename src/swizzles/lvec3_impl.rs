// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{LVec2, LVec3, LVec4, Vec3Swizzles};

impl Vec3Swizzles for LVec3 {
    type Vec2 = LVec2;

    type Vec4 = LVec4;

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
}
