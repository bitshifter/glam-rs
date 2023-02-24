// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{ULVec2, ULVec3, ULVec4, Vec3Swizzles};

impl Vec3Swizzles for ULVec3 {
    type Vec2 = ULVec2;

    type Vec4 = ULVec4;

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
}
