// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{HVec2, HVec3, HVec4, Vec2Swizzles};

impl Vec2Swizzles for HVec2 {
    type Vec3 = HVec3;

    type Vec4 = HVec4;

    #[inline]
    fn xx(self) -> Self {
        Self {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn yx(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> Self {
        Self {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn xxx(self) -> HVec3 {
        HVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> HVec3 {
        HVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xyx(self) -> HVec3 {
        HVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> HVec3 {
        HVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn yxx(self) -> HVec3 {
        HVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> HVec3 {
        HVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yyx(self) -> HVec3 {
        HVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> HVec3 {
        HVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn xxxx(self) -> HVec4 {
        HVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> HVec4 {
        HVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> HVec4 {
        HVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> HVec4 {
        HVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> HVec4 {
        HVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> HVec4 {
        HVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> HVec4 {
        HVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> HVec4 {
        HVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> HVec4 {
        HVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> HVec4 {
        HVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> HVec4 {
        HVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> HVec4 {
        HVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> HVec4 {
        HVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> HVec4 {
        HVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> HVec4 {
        HVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> HVec4 {
        HVec4::new(self.y, self.y, self.y, self.y)
    }
}
