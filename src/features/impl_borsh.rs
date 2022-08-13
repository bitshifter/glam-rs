macro_rules! derive_borsh {
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
    
    derive_borsh!(Affine2);
    derive_borsh!(Affine3A);
    derive_borsh!(Mat2);
    derive_borsh!(Mat3);
    derive_borsh!(Mat3A);
    derive_borsh!(Mat4);
    derive_borsh!(Quat);
    derive_borsh!(Vec2);
    derive_borsh!(Vec3);
    derive_borsh!(Vec3A);
    derive_borsh!(Vec4);
}

mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};

    derive_borsh!(DAffine2);
    derive_borsh!(DAffine3);
    derive_borsh!(DMat2);
    derive_borsh!(DMat3);
    derive_borsh!(DMat4);
    derive_borsh!(DQuat);
    derive_borsh!(DVec2);
    derive_borsh!(DVec3);
    derive_borsh!(DVec4);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};

    derive_borsh!(IVec2);
    derive_borsh!(IVec3);
    derive_borsh!(IVec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};

    derive_borsh!(UVec2);
    derive_borsh!(UVec3);
    derive_borsh!(UVec4);
}
