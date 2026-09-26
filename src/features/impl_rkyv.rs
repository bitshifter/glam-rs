#[cfg(feature = "bytecheck")]
macro_rules! impl_rkyv {
    (@bytecheck $type:ty) => {
        // SAFETY: All bit patterns are valid for these primitive types.
        // https://docs.rs/bytecheck/0.8.1/src/bytecheck/lib.rs.html#352
        unsafe impl<C: Fallible +?Sized> rkyv_08::bytecheck::CheckBytes<C> for $type {
            #[inline]
            unsafe fn check_bytes(
                _value: *const Self,
                _: &mut C,
            ) -> Result<(), C::Error> {
                Ok(())
            }
        }
    };

    ($type:ty) => {
        impl_rkyv_derive!(@serialize $type);
        impl_rkyv_derive!(@archive_deserialize $type);
        impl_rkyv!(@bytecheck $type);
    };
}

#[cfg(not(feature = "bytecheck"))]
macro_rules! impl_rkyv {
    ($type:ty) => {
        impl_rkyv_derive!(@serialize $type);
        impl_rkyv_derive!(@archive_deserialize $type);
    };
}

macro_rules! impl_rkyv_derive {
    (@serialize $type:ty) => {
        impl<S: Fallible + ?Sized> Serialize<S> for $type {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }
    };

    (@archive_deserialize $type:ty) => {
        // SAFETY: All glam types have a fully defined data layout.
        unsafe impl rkyv_08::traits::NoUndef for $type {}
        // SAFETY: All glam types have a stable, well-defined layout that is identical on all
        // targets.
        unsafe impl rkyv_08::Portable for $type {}
        impl Archive for $type {
            type Archived = $type;
            type Resolver = ();

            #[inline]
            fn resolve(&self, _: Self::Resolver, out: Place<Self::Archived>) {
                out.write(*self)
            }
        }

        impl<D: Fallible + ?Sized> Deserialize<$type, D> for $type {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(*self)
            }
        }
    };
}

mod f32 {
    use crate::{
        Affine2, Affine3, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4,
    };
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};
    impl_rkyv!(Affine2);
    impl_rkyv!(Affine3);
    impl_rkyv!(Affine3A);
    impl_rkyv!(Mat2);
    impl_rkyv!(Mat3);
    impl_rkyv!(Mat3A);
    impl_rkyv!(Mat4);
    impl_rkyv!(Quat);
    impl_rkyv!(Vec2);
    impl_rkyv!(Vec3);
    impl_rkyv!(Vec3A);
    impl_rkyv!(Vec4);
}

#[cfg(feature = "f64")]
mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(DAffine2);
    impl_rkyv!(DAffine3);
    impl_rkyv!(DMat2);
    impl_rkyv!(DMat3);
    impl_rkyv!(DMat4);
    impl_rkyv!(DQuat);
    impl_rkyv!(DVec2);
    impl_rkyv!(DVec3);
    impl_rkyv!(DVec4);
}

#[cfg(feature = "i8")]
mod i8 {
    use crate::{I8Vec2, I8Vec3, I8Vec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(I8Vec2);
    impl_rkyv!(I8Vec3);
    impl_rkyv!(I8Vec4);
}

#[cfg(feature = "i16")]
mod i16 {
    use crate::{I16Vec2, I16Vec3, I16Vec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(I16Vec2);
    impl_rkyv!(I16Vec3);
    impl_rkyv!(I16Vec4);
}

#[cfg(feature = "i32")]
mod i32 {
    use crate::{IVec2, IVec3, IVec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(IVec2);
    impl_rkyv!(IVec3);
    impl_rkyv!(IVec4);
}

#[cfg(feature = "i64")]
mod i64 {
    use crate::{I64Vec2, I64Vec3, I64Vec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(I64Vec2);
    impl_rkyv!(I64Vec3);
    impl_rkyv!(I64Vec4);
}

#[cfg(feature = "u8")]
mod u8 {
    use crate::{U8Vec2, U8Vec3, U8Vec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(U8Vec2);
    impl_rkyv!(U8Vec3);
    impl_rkyv!(U8Vec4);
}

#[cfg(feature = "u16")]
mod u16 {
    use crate::{U16Vec2, U16Vec3, U16Vec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(U16Vec2);
    impl_rkyv!(U16Vec3);
    impl_rkyv!(U16Vec4);
}

#[cfg(feature = "u32")]
mod u32 {
    use crate::{UVec2, UVec3, UVec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(UVec2);
    impl_rkyv!(UVec3);
    impl_rkyv!(UVec4);
}

#[cfg(feature = "u64")]
mod u64 {
    use crate::{U64Vec2, U64Vec3, U64Vec4};
    use rkyv_08::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(U64Vec2);
    impl_rkyv!(U64Vec3);
    impl_rkyv!(U64Vec4);
}

#[cfg(test)]
mod test {
    /// The serializer type expected by [`rkyv_08::to_bytes()`].
    pub type TestSerializer<'a> = rkyv_08::api::high::HighSerializer<
        rkyv_08::util::AlignedVec,
        rkyv_08::ser::allocator::ArenaHandle<'a>,
        rkyv_08::rancor::Panic,
    >;
    /// The deserializer type expected by [`rkyv_08::deserialize()`].
    pub type TestDeserializer = rkyv_08::api::high::HighDeserializer<rkyv_08::rancor::Panic>;
    pub fn test_archive<T>(value: &T)
    where
        T: core::fmt::Debug
            + PartialEq
            + rkyv_08::Portable
            + for<'a> rkyv_08::Serialize<TestSerializer<'a>>,
        T::Archived: core::fmt::Debug + PartialEq<T> + rkyv_08::Deserialize<T, TestDeserializer>,
    {
        let buffer = rkyv_08::to_bytes(value).unwrap();

        // SAFETY: all bit patterns are valid for the primitive types used by glam.  There is
        // no need to write special-cased conditional tests that rely on bytecheck for the safe
        // rkyv_08::access() wrapper.
        let archived_value = unsafe { rkyv_08::access_unchecked::<T::Archived>(&buffer) };
        assert_eq!(archived_value, value);
        assert_eq!(
            &rkyv_08::deserialize::<T, rkyv_08::rancor::Panic>(archived_value).unwrap(),
            value
        );
    }

    #[test]
    fn test_rkyv() {
        use crate::{
            Affine2, Affine3, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4,
        };
        test_archive(&Affine2::from_cols_array(&[1.0, 0.0, 2.0, 0.0, 3.0, 4.0]));
        test_archive(&Affine3::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
        ]));
        test_archive(&Affine3A::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
        ]));
        test_archive(&Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]));
        test_archive(&Mat3::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
        ]));
        test_archive(&Mat3A::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
        ]));
        test_archive(&Mat4::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]));
        test_archive(&Quat::from_xyzw(1.0, 2.0, 3.0, 4.0));
        test_archive(&Vec2::new(1.0, 2.0));
        test_archive(&Vec3::new(1.0, 2.0, 3.0));
        test_archive(&Vec3A::new(1.0, 2.0, 3.0));
        test_archive(&Vec4::new(1.0, 2.0, 3.0, 4.0));

        #[cfg(feature = "f64")]
        {
            use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
            test_archive(&DAffine2::from_cols_array(&[1.0, 0.0, 2.0, 0.0, 3.0, 4.0]));
            test_archive(&DAffine3::from_cols_array(&[
                1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
            ]));
            test_archive(&DMat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]));
            test_archive(&DMat3::from_cols_array(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
            ]));
            test_archive(&DMat4::from_cols_array(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ]));
            test_archive(&DQuat::from_xyzw(1.0, 2.0, 3.0, 4.0));
            test_archive(&DVec2::new(1.0, 2.0));
            test_archive(&DVec3::new(1.0, 2.0, 3.0));
            test_archive(&DVec4::new(1.0, 2.0, 3.0, 4.0));
        }

        #[cfg(feature = "i8")]
        {
            use crate::{I8Vec2, I8Vec3, I8Vec4};
            test_archive(&I8Vec2::new(-1, 2));
            test_archive(&I8Vec3::new(-1, 2, 3));
            test_archive(&I8Vec4::new(-1, 2, 3, 4));
        }

        #[cfg(feature = "i16")]
        {
            use crate::{I16Vec2, I16Vec3, I16Vec4};
            test_archive(&I16Vec2::new(-1, 2));
            test_archive(&I16Vec3::new(-1, 2, 3));
            test_archive(&I16Vec4::new(-1, 2, 3, 4));
        }

        #[cfg(feature = "i32")]
        {
            use crate::{IVec2, IVec3, IVec4};
            test_archive(&IVec2::new(-1, 2));
            test_archive(&IVec3::new(-1, 2, 3));
            test_archive(&IVec4::new(-1, 2, 3, 4));
        }

        #[cfg(feature = "i64")]
        {
            use crate::{I64Vec2, I64Vec3, I64Vec4};
            test_archive(&I64Vec2::new(-1, 2));
            test_archive(&I64Vec3::new(-1, 2, 3));
            test_archive(&I64Vec4::new(-1, 2, 3, 4));
        }

        #[cfg(feature = "u8")]
        {
            use crate::{U8Vec2, U8Vec3, U8Vec4};
            test_archive(&U8Vec2::new(1, 2));
            test_archive(&U8Vec3::new(1, 2, 3));
            test_archive(&U8Vec4::new(1, 2, 3, 4));
        }

        #[cfg(feature = "u16")]
        {
            use crate::{U16Vec2, U16Vec3, U16Vec4};
            test_archive(&U16Vec2::new(1, 2));
            test_archive(&U16Vec3::new(1, 2, 3));
            test_archive(&U16Vec4::new(1, 2, 3, 4));
        }

        #[cfg(feature = "u32")]
        {
            use crate::{UVec2, UVec3, UVec4};
            test_archive(&UVec2::new(1, 2));
            test_archive(&UVec3::new(1, 2, 3));
            test_archive(&UVec4::new(1, 2, 3, 4));
        }

        #[cfg(feature = "u64")]
        {
            use crate::{U64Vec2, U64Vec3, U64Vec4};
            test_archive(&U64Vec2::new(1, 2));
            test_archive(&U64Vec3::new(1, 2, 3));
            test_archive(&U64Vec4::new(1, 2, 3, 4));
        }
    }
}
