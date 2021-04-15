use crate::{
    DMat2, DMat3, DMat4, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3,
    UVec4, Vec2, Vec3, Vec4,
};

use bytemuck::Zeroable;

macro_rules! vectors {
            ( $( $glam_ty:ty, $std_name:ident, ( $($field:ident),* ), )* ) => {
                $(
                    impl crevice::std140::AsStd140 for $glam_ty {
                        type Std140Type = crevice::std140::$std_name;

                        fn as_std140(&self) -> Self::Std140Type {
                            crevice::std140::$std_name {
                                $(
                                    $field: self.$field,
                                )*
                            }
                        }
                    }

                    impl crevice::std430::AsStd430 for $glam_ty {
                        type Std430Type = crevice::std430::$std_name;

                        fn as_std430(&self) -> Self::Std430Type {
                            crevice::std430::$std_name {
                                $(
                                    $field: self.$field,
                                )*
                            }
                        }
                    }
                )*
            };
        }

vectors! {
    Vec2, Vec2, (x, y),
    Vec3, Vec3, (x, y, z),
    Vec4, Vec4, (x, y, z, w),
    IVec2, IVec2, (x, y),
    IVec3, IVec3, (x, y, z),
    IVec4, IVec4, (x, y, z, w),
    UVec2, UVec2, (x, y),
    UVec3, UVec3, (x, y, z),
    UVec4, UVec4, (x, y, z, w),
    DVec2, DVec2, (x, y),
    DVec3, DVec3, (x, y, z),
    DVec4, DVec4, (x, y, z, w),
}

macro_rules! matrices {
            ( $( $glam_ty:ty, $std_name:ident, ( $($field:ident = $index:expr),* ), )* ) => {
                $(
                    impl crevice::std140::AsStd140 for $glam_ty {
                        type Std140Type = crevice::std140::$std_name;

                        fn as_std140(&self) -> Self::Std140Type {
                            crevice::std140::$std_name {
                                $(
                                    $field: self.col($index).as_std140(),
                                )*
                                ..Zeroable::zeroed()
                            }
                        }
                    }

                    impl crevice::std430::AsStd430 for $glam_ty {
                        type Std430Type = crevice::std430::$std_name;

                        fn as_std430(&self) -> Self::Std430Type {
                            crevice::std430::$std_name {
                                $(
                                    $field: self.col($index).as_std430(),
                                )*
                                ..Zeroable::zeroed()
                            }
                        }
                    }
                )*
            };
        }

matrices! {
    Mat2, Mat2, (x = 0, y = 1),
    Mat3, Mat3, (x = 0, y = 1, z = 2),
    Mat4, Mat4, (x = 0, y = 1, z = 2, w = 3),
    DMat2, DMat2, (x = 0, y = 1),
    DMat3, DMat3, (x = 0, y = 1, z = 2),
    DMat4, DMat4, (x = 0, y = 1, z = 2, w = 3),
}
