#[cfg(feature = "bytecheck")]
macro_rules! impl_rkyv {
    (@bytecheck $type:ty) => {
        impl<C: ?Sized> bytecheck::CheckBytes<C> for $type {
            type Error = core::convert::Infallible;

            #[inline]
            unsafe fn check_bytes<'a>(
                value: *const Self,
                _: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                Ok(&*value)
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
        const _: () = {
            #[cfg(not(any(feature = "archive_le", feature = "archive_be")))]
            type Archived = $type;
            #[cfg(feature = "archive_le")]
            type Archived = rend::LittleEndian<$type>;
            #[cfg(feature = "archive_be")]
            type Archived = rend::BigEndian<$type>;

            impl Archive for $type {
                type Archived = Archived;
                type Resolver = ();

                #[inline]
                unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
                    out.write(to_archived!(*self as Self));
                }
            }

            impl<D: Fallible + ?Sized> Deserialize<$type, D> for Archived {
                #[inline]
                fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                    Ok(from_archived!(*self))
                }
            }
        };
    };
}

mod f32 {
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use rkyv::{from_archived, to_archived, Archive, Deserialize, Fallible, Serialize};
    impl_rkyv!(Affine2);
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

mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use rkyv::{from_archived, to_archived, Archive, Deserialize, Fallible, Serialize};

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

mod i32 {
    use crate::{IVec2, IVec3, IVec4};
    use rkyv::{from_archived, to_archived, Archive, Deserialize, Fallible, Serialize};

    impl_rkyv!(IVec2);
    impl_rkyv!(IVec3);
    impl_rkyv!(IVec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};
    use rkyv::{from_archived, to_archived, Archive, Deserialize, Fallible, Serialize};

    impl_rkyv!(UVec2);
    impl_rkyv!(UVec3);
    impl_rkyv!(UVec4);
}
