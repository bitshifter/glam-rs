macro_rules! impl_borsh {
    ($type:ident) => {
        
        impl borsh::BorshSerialize for $type {
            fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {

                let buf = self.to_array();
                borsh::to_writer(writer, &buf)?;

                Ok(())
            }
        }

        impl borsh::BorshDeserialize for $type {
            fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {

                let d = <glam::$type>::from_array(<$serialized>::try_from_slice(buf)?);
                let instance = Self(d);

                Ok(instance)
            }
        }
    }
}


mod f32 {
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    
    impl_borsh!(Affine2);
    impl_borsh!(Affine3A);
    impl_borsh!(Mat2);
    impl_borsh!(Mat3);
    impl_borsh!(Mat3A);
    impl_borsh!(Mat4);
    impl_borsh!(Quat);
    impl_borsh!(Vec2);
    impl_borsh!(Vec3);
    impl_borsh!(Vec3A);
    impl_borsh!(Vec4);
}

mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};

    impl_borsh!(DAffine2);
    impl_borsh!(DAffine3);
    impl_borsh!(DMat2);
    impl_borsh!(DMat3);
    impl_borsh!(DMat4);
    impl_borsh!(DQuat);
    impl_borsh!(DVec2);
    impl_borsh!(DVec3);
    impl_borsh!(DVec4);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};

    impl_borsh!(IVec2);
    impl_borsh!(IVec3);
    impl_borsh!(IVec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};

    impl_borsh!(UVec2);
    impl_borsh!(UVec3);
    impl_borsh!(UVec4);
}
