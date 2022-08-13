macro_rules! impl_borsh {
    ($type:ident, $function:ident, $serialized:ty) => {
        
        impl borsh::BorshSerialize for $type {
            fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {

                let buf = self.to_$function();
                borsh::to_writer(writer, &buf)?;

                Ok(())
            }
        }

        impl borsh::BorshDeserialize for $type {
            fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {

                let d = Self::from_$function(<$serialized>::try_from_slice(buf)?);
                let instance = Self(d);

                Ok(instance)
            }
        }
    }
}


mod f32 {
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    
    impl_borsh!(Affine2, "cols_array", [f32; 6]);
    impl_borsh!(Affine3A, "cols_array", [f32; 12]);
    impl_borsh!(Mat2, "cols_array", [f32; 4]);
    impl_borsh!(Mat3, "cols_array", [f32; 9]);
    impl_borsh!(Mat3A, "cols_array", [f32; 9]);
    impl_borsh!(Mat4, "cols_array", [f32; 16]);
    impl_borsh!(Quat, "array", [f32; 4]);
    impl_borsh!(Vec2, "array", [f32; 2]);
    impl_borsh!(Vec3, "array", [f32; 3]);
    impl_borsh!(Vec3A, "array", [f32; 3]);
    impl_borsh!(Vec4, "array", [f32; 4]);
}

mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};

    impl_borsh!(DAffine2, "cols_array", [f64; 6]);
    impl_borsh!(DAffine3, "cols_array", [f64; 12]);
    impl_borsh!(DMat2, "cols_array", [f64; 4]);
    impl_borsh!(DMat3, "cols_array", [f64; 9]);
    impl_borsh!(DMat4, "cols_array", [f64; 16]);
    impl_borsh!(DQuat, "array", [f64; 4]);
    impl_borsh!(DVec2, "array", [f64; 2]);
    impl_borsh!(DVec3, "array", [f64; 3]);
    impl_borsh!(DVec4, "array", [f64; 4]);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};

    impl_borsh!(IVec2, "array", [i32; 2]);
    impl_borsh!(IVec3, "array", [i32; 3]);
    impl_borsh!(IVec4, "array", [i32; 4]);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};

    impl_borsh!(UVec2, "array", [u32; 2]);
    impl_borsh!(UVec3, "array", [u32; 3]);
    impl_borsh!(UVec4, "array", [u32; 4]);
}
