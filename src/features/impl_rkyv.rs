//! `rkyv` support for glam types.
//!
//! Every glam type archives to a dedicated `Archived*` struct built out of rkyv's own
//! archived primitives ([`rkyv::Archived<f32>`] and friends) rather than aliasing the
//! native glam type. That keeps the archived representation independent of the SIMD
//! backend glam was built with, gives it the endianness rkyv asks for, and lets it
//! inherit rkyv's `unaligned` feature so archives can be read from buffers that are not
//! aligned to the native type.
//!
//! The archived types are storage only: they carry no `#[repr(align(..))]` and implement
//! no arithmetic. Use [`rkyv::Deserialize`] to recover the native glam type.

use rkyv::{
    rancor::Fallible,
    traits::{CopyOptimization, NoUndef},
    Archive, Archived, Deserialize, Place, Portable, Serialize,
};

use crate::{Affine2, Affine3, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
#[cfg(feature = "f64")]
use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
#[cfg(feature = "i16")]
use crate::{I16Vec2, I16Vec3, I16Vec4};
#[cfg(feature = "i64")]
use crate::{I64Vec2, I64Vec3, I64Vec4};
#[cfg(feature = "i8")]
use crate::{I8Vec2, I8Vec3, I8Vec4};
#[cfg(feature = "i32")]
use crate::{IVec2, IVec3, IVec4};
#[cfg(feature = "u16")]
use crate::{U16Vec2, U16Vec3, U16Vec4};
#[cfg(feature = "u64")]
use crate::{U64Vec2, U64Vec3, U64Vec4};
#[cfg(feature = "u8")]
use crate::{U8Vec2, U8Vec3, U8Vec4};
#[cfg(feature = "u32")]
use crate::{UVec2, UVec3, UVec4};

/// Conversion between a primitive and the archived form rkyv picked for it.
///
/// [`rkyv::Archived`] is one of `rend`'s endianness-explicit types for multi-byte
/// primitives, and the primitive itself for single-byte ones, which have no endianness.
trait ArchivablePrimitive: Archive + Copy {
    fn to_archived(self) -> Archived<Self>;
    fn from_archived(archived: Archived<Self>) -> Self;
}

macro_rules! impl_archivable_primitive {
    ($($prim:ty),* $(,)?) => {
        $(
            impl ArchivablePrimitive for $prim {
                #[inline]
                fn to_archived(self) -> Archived<Self> {
                    Archived::<Self>::from_native(self)
                }

                #[inline]
                fn from_archived(archived: Archived<Self>) -> Self {
                    archived.to_native()
                }
            }
        )*
    };
}

macro_rules! impl_archivable_byte {
    ($($prim:ty),* $(,)?) => {
        $(
            impl ArchivablePrimitive for $prim {
                #[inline]
                fn to_archived(self) -> Archived<Self> {
                    self
                }

                #[inline]
                fn from_archived(archived: Archived<Self>) -> Self {
                    archived
                }
            }
        )*
    };
}

impl_archivable_primitive!(f32, f64, i16, i32, i64, u16, u32, u64);
impl_archivable_byte!(i8, u8);

macro_rules! impl_rkyv {
    (array $type:ident, $archived:ident, $prim:ty, $n:expr) => {
        impl_rkyv!(
            @impl $type, $archived, $prim, $n,
            |value| value.to_array(),
            |array| <$type>::from_array(array)
        );
    };

    (cols $type:ident, $archived:ident, $prim:ty, $n:expr) => {
        impl_rkyv!(
            @impl $type, $archived, $prim, $n,
            |value| value.to_cols_array(),
            |array| <$type>::from_cols_array(&array)
        );
    };

    (
        @impl $type:ident, $archived:ident, $prim:ty, $n:expr,
        |$value:ident| $to_native:expr,
        |$array:ident| $from_native:expr
    ) => {
        #[doc = concat!("The archived version of [`", stringify!($type), "`].")]
        ///
        /// The element type follows rkyv's `unaligned` and `big_endian` features, so this
        /// type is one byte aligned whenever rkyv itself is built that way.
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $archived(pub [Archived<$prim>; $n]);

        // SAFETY: `#[repr(transparent)]` over an array of archived primitives, each of
        // which has a stable layout and explicit endianness on every target. Contains no
        // interior mutability.
        unsafe impl Portable for $archived {}

        // SAFETY: `#[repr(transparent)]` over an array of equally sized archived
        // primitives, so there is no padding and no uninitialised byte.
        unsafe impl NoUndef for $archived {}

        // SAFETY: An array of archived primitives, for which zero is a valid value.
        //
        // The derive is unusable here: it would require the element type to implement
        // `Zeroable`, which it only does when `rend` is built with its `bytemuck-1`
        // feature, and that is not something `glam` can enable through `rkyv`.
        #[cfg(feature = "bytemuck")]
        unsafe impl bytemuck::Zeroable for $archived {}

        // SAFETY: `#[repr(transparent)]` over an array of equally sized archived
        // primitives, so there is no padding, and every bit pattern is a valid value of
        // the primitive it archives. Note that this says nothing about endianness: reading
        // these bytes as native primitives is only meaningful when the archive endianness
        // matches the target, exactly as for the native `glam` types.
        #[cfg(feature = "bytemuck")]
        unsafe impl bytemuck::Pod for $archived {}

        impl From<$type> for $archived {
            #[inline]
            fn from(value: $type) -> Self {
                let $value = &value;
                let native: [$prim; $n] = $to_native;
                Self(native.map(ArchivablePrimitive::to_archived))
            }
        }

        impl From<$archived> for $type {
            #[inline]
            fn from(archived: $archived) -> Self {
                let $array: [$prim; $n] = archived.0.map(ArchivablePrimitive::from_archived);
                $from_native
            }
        }

        impl Archive for $type {
            // SAFETY: The optimization copies the bytes of the native value straight into
            // the archive, so the two layouts have to agree byte for byte.
            //
            // The element type reports whether an archived primitive is a copy of its
            // native form, which is false when the archive endianness and the target
            // endianness disagree. Given that, the size check is enough: the native type
            // holds exactly `$n` primitives, so if it is no larger than `$n` of them it has
            // no padding, and both forms store those primitives in the order used by
            // `to_array`/`to_cols_array` -- which glam itself implements as a transmute for
            // the types that are laid out that way.
            //
            // Types that do have padding, such as `Vec3A` and (on SIMD backends) `Affine2`,
            // are larger than their archived form and so disable the optimization here.
            const COPY_OPTIMIZATION: CopyOptimization<Self> = unsafe {
                CopyOptimization::enable_if(
                    <$prim as Archive>::COPY_OPTIMIZATION.is_enabled()
                        && core::mem::size_of::<Self>() == core::mem::size_of::<$archived>(),
                )
            };

            type Archived = $archived;
            type Resolver = ();

            #[inline]
            fn resolve(&self, _: Self::Resolver, out: Place<Self::Archived>) {
                out.write((*self).into());
            }
        }

        impl<S: Fallible + ?Sized> Serialize<S> for $type {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }

        impl<D: Fallible + ?Sized> Deserialize<$type, D> for $archived {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok((*self).into())
            }
        }

        impl PartialEq<$type> for $archived {
            #[inline]
            fn eq(&self, other: &$type) -> bool {
                let $value = other;
                let native: [$prim; $n] = $to_native;
                let archived: [$prim; $n] = self.0.map(ArchivablePrimitive::from_archived);
                archived == native
            }
        }

        #[cfg(feature = "bytecheck")]
        // SAFETY: `#[repr(transparent)]` over the array, so a pointer to `Self` is a
        // valid pointer to the array, and validating the array validates `Self`.
        unsafe impl<C> rkyv::bytecheck::CheckBytes<C> for $archived
        where
            C: Fallible + ?Sized,
            [Archived<$prim>; $n]: rkyv::bytecheck::CheckBytes<C>,
        {
            #[inline]
            unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), C::Error> {
                unsafe {
                    <[Archived<$prim>; $n] as rkyv::bytecheck::CheckBytes<C>>::check_bytes(
                        value.cast(),
                        context,
                    )
                }
            }
        }
    };
}

impl_rkyv!(cols Affine2, ArchivedAffine2, f32, 6);
impl_rkyv!(cols Affine3, ArchivedAffine3, f32, 12);
impl_rkyv!(cols Affine3A, ArchivedAffine3A, f32, 12);
impl_rkyv!(cols Mat2, ArchivedMat2, f32, 4);
impl_rkyv!(cols Mat3, ArchivedMat3, f32, 9);
impl_rkyv!(cols Mat3A, ArchivedMat3A, f32, 9);
impl_rkyv!(cols Mat4, ArchivedMat4, f32, 16);
impl_rkyv!(array Quat, ArchivedQuat, f32, 4);
impl_rkyv!(array Vec2, ArchivedVec2, f32, 2);
impl_rkyv!(array Vec3, ArchivedVec3, f32, 3);
impl_rkyv!(array Vec3A, ArchivedVec3A, f32, 3);
impl_rkyv!(array Vec4, ArchivedVec4, f32, 4);

#[cfg(feature = "f64")]
mod f64_impls {
    use super::*;

    impl_rkyv!(cols DAffine2, ArchivedDAffine2, f64, 6);
    impl_rkyv!(cols DAffine3, ArchivedDAffine3, f64, 12);
    impl_rkyv!(cols DMat2, ArchivedDMat2, f64, 4);
    impl_rkyv!(cols DMat3, ArchivedDMat3, f64, 9);
    impl_rkyv!(cols DMat4, ArchivedDMat4, f64, 16);
    impl_rkyv!(array DQuat, ArchivedDQuat, f64, 4);
    impl_rkyv!(array DVec2, ArchivedDVec2, f64, 2);
    impl_rkyv!(array DVec3, ArchivedDVec3, f64, 3);
    impl_rkyv!(array DVec4, ArchivedDVec4, f64, 4);
}

#[cfg(feature = "f64")]
pub use f64_impls::*;

#[cfg(feature = "i8")]
mod i8_impls {
    use super::*;

    impl_rkyv!(array I8Vec2, ArchivedI8Vec2, i8, 2);
    impl_rkyv!(array I8Vec3, ArchivedI8Vec3, i8, 3);
    impl_rkyv!(array I8Vec4, ArchivedI8Vec4, i8, 4);
}

#[cfg(feature = "i8")]
pub use i8_impls::*;

#[cfg(feature = "i16")]
mod i16_impls {
    use super::*;

    impl_rkyv!(array I16Vec2, ArchivedI16Vec2, i16, 2);
    impl_rkyv!(array I16Vec3, ArchivedI16Vec3, i16, 3);
    impl_rkyv!(array I16Vec4, ArchivedI16Vec4, i16, 4);
}

#[cfg(feature = "i16")]
pub use i16_impls::*;

#[cfg(feature = "i32")]
mod i32_impls {
    use super::*;

    impl_rkyv!(array IVec2, ArchivedIVec2, i32, 2);
    impl_rkyv!(array IVec3, ArchivedIVec3, i32, 3);
    impl_rkyv!(array IVec4, ArchivedIVec4, i32, 4);
}

#[cfg(feature = "i32")]
pub use i32_impls::*;

#[cfg(feature = "i64")]
mod i64_impls {
    use super::*;

    impl_rkyv!(array I64Vec2, ArchivedI64Vec2, i64, 2);
    impl_rkyv!(array I64Vec3, ArchivedI64Vec3, i64, 3);
    impl_rkyv!(array I64Vec4, ArchivedI64Vec4, i64, 4);
}

#[cfg(feature = "i64")]
pub use i64_impls::*;

#[cfg(feature = "u8")]
mod u8_impls {
    use super::*;

    impl_rkyv!(array U8Vec2, ArchivedU8Vec2, u8, 2);
    impl_rkyv!(array U8Vec3, ArchivedU8Vec3, u8, 3);
    impl_rkyv!(array U8Vec4, ArchivedU8Vec4, u8, 4);
}

#[cfg(feature = "u8")]
pub use u8_impls::*;

#[cfg(feature = "u16")]
mod u16_impls {
    use super::*;

    impl_rkyv!(array U16Vec2, ArchivedU16Vec2, u16, 2);
    impl_rkyv!(array U16Vec3, ArchivedU16Vec3, u16, 3);
    impl_rkyv!(array U16Vec4, ArchivedU16Vec4, u16, 4);
}

#[cfg(feature = "u16")]
pub use u16_impls::*;

#[cfg(feature = "u32")]
mod u32_impls {
    use super::*;

    impl_rkyv!(array UVec2, ArchivedUVec2, u32, 2);
    impl_rkyv!(array UVec3, ArchivedUVec3, u32, 3);
    impl_rkyv!(array UVec4, ArchivedUVec4, u32, 4);
}

#[cfg(feature = "u32")]
pub use u32_impls::*;

#[cfg(feature = "u64")]
mod u64_impls {
    use super::*;

    impl_rkyv!(array U64Vec2, ArchivedU64Vec2, u64, 2);
    impl_rkyv!(array U64Vec3, ArchivedU64Vec3, u64, 3);
    impl_rkyv!(array U64Vec4, ArchivedU64Vec4, u64, 4);
}

#[cfg(feature = "u64")]
pub use u64_impls::*;

#[cfg(test)]
mod test {
    use rkyv::Archive;

    /// The serializer type expected by [`rkyv::to_bytes()`].
    pub type TestSerializer<'a> = rkyv::api::high::HighSerializer<
        rkyv::util::AlignedVec,
        rkyv::ser::allocator::ArenaHandle<'a>,
        rkyv::rancor::Panic,
    >;
    /// The deserializer type expected by [`rkyv::deserialize()`].
    pub type TestDeserializer = rkyv::api::high::HighDeserializer<rkyv::rancor::Panic>;

    #[cfg(feature = "bytecheck")]
    pub fn test_archive<T>(value: &T)
    where
        T: core::fmt::Debug + PartialEq + for<'a> rkyv::Serialize<TestSerializer<'a>>,
        T::Archived: core::fmt::Debug
            + PartialEq<T>
            + rkyv::Deserialize<T, TestDeserializer>
            + rkyv::Portable
            + for<'a> rkyv::bytecheck::CheckBytes<
                rkyv::api::high::HighValidator<'a, rkyv::rancor::Panic>,
            >,
    {
        let buffer = rkyv::to_bytes(value).unwrap();

        // The archive is validated rather than accessed unchecked, which also covers the
        // alignment of the buffer it is read out of.
        let archived_value = rkyv::access::<T::Archived, rkyv::rancor::Panic>(&buffer).unwrap();
        assert_archived_eq::<T>(archived_value, value);
    }

    #[cfg(not(feature = "bytecheck"))]
    pub fn test_archive<T>(value: &T)
    where
        T: core::fmt::Debug + PartialEq + for<'a> rkyv::Serialize<TestSerializer<'a>>,
        T::Archived: core::fmt::Debug
            + PartialEq<T>
            + rkyv::Deserialize<T, TestDeserializer>
            + rkyv::Portable,
    {
        let buffer = rkyv::to_bytes(value).unwrap();

        // SAFETY: `to_bytes` returns an `AlignedVec`, which satisfies the alignment of any
        // archived type, and every archived glam type is an array of archived primitives
        // for which all bit patterns are valid.
        let archived_value = unsafe { rkyv::access_unchecked::<T::Archived>(&buffer) };
        assert_archived_eq::<T>(archived_value, value);
    }

    fn assert_archived_eq<T>(archived_value: &T::Archived, value: &T)
    where
        T: core::fmt::Debug + PartialEq + Archive,
        T::Archived: core::fmt::Debug + PartialEq<T> + rkyv::Deserialize<T, TestDeserializer>,
    {
        assert_eq!(archived_value, value);
        assert_eq!(
            &rkyv::deserialize::<T, rkyv::rancor::Panic>(archived_value).unwrap(),
            value
        );
        assert_copy_optimization_is_sound(value, archived_value);
    }

    /// [`rkyv::Archive::COPY_OPTIMIZATION`] promises that the archived form is a copy of
    /// the native one, and rkyv acts on it by memcpying whole slices of native values into
    /// the archive. The size half of that promise is checked when the constant is declared;
    /// this pins the field order, which is not otherwise verifiable at compile time.
    fn assert_copy_optimization_is_sound<T: Archive>(value: &T, archived_value: &T::Archived) {
        if !T::COPY_OPTIMIZATION.is_enabled() {
            return;
        }

        assert_eq!(
            core::mem::size_of::<T>(),
            core::mem::size_of::<T::Archived>(),
            "the copy optimization may only be enabled for equally sized types"
        );

        // SAFETY: The optimization is only enabled for types without padding, so every byte
        // of both values is initialised. This is the same read rkyv performs when it acts on
        // the hint.
        let (native, archived) = unsafe {
            (
                core::slice::from_raw_parts(
                    core::ptr::from_ref(value).cast::<u8>(),
                    core::mem::size_of::<T>(),
                ),
                core::slice::from_raw_parts(
                    core::ptr::from_ref(archived_value).cast::<u8>(),
                    core::mem::size_of::<T::Archived>(),
                ),
            )
        };
        assert_eq!(
            native, archived,
            "the copy optimization is enabled for a type whose archived form is not a copy"
        );
    }

    /// The archived form must not impose an alignment on the buffer it is read from when
    /// `rkyv` is built with its `unaligned` feature.
    #[test]
    fn archived_alignment_follows_rkyv() {
        use crate::{ArchivedVec3, ArchivedVec3A, ArchivedVec4};

        let expected = core::mem::align_of::<rkyv::Archived<f32>>();
        assert_eq!(core::mem::align_of::<ArchivedVec3>(), expected);
        assert_eq!(core::mem::align_of::<ArchivedVec3A>(), expected);
        assert_eq!(core::mem::align_of::<ArchivedVec4>(), expected);
    }

    /// The archived form is a packed array of elements, with no padding or trailing
    /// alignment, regardless of the SIMD backend glam was built with.
    #[test]
    fn archived_size_is_exact() {
        use crate::{ArchivedVec3, ArchivedVec3A, ArchivedVec4};

        let element = core::mem::size_of::<rkyv::Archived<f32>>();
        assert_eq!(core::mem::size_of::<ArchivedVec3>(), element * 3);
        assert_eq!(core::mem::size_of::<ArchivedVec3A>(), element * 3);
        assert_eq!(core::mem::size_of::<ArchivedVec4>(), element * 4);
    }

    /// Whether a type can be copied into the archive wholesale is a property of its native
    /// layout, so it must not depend on the SIMD backend glam was built with.
    #[test]
    fn copy_optimization_is_backend_independent() {
        use crate::{Affine3, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};

        fn is_enabled<T: Archive>() -> bool {
            T::COPY_OPTIMIZATION.is_enabled()
        }

        // False when rkyv archives to the endianness the target does not use, which rules
        // the optimization out for every type built on `f32`.
        if is_enabled::<f32>() {
            assert!(is_enabled::<Vec2>());
            assert!(is_enabled::<Vec3>());
            assert!(is_enabled::<Vec4>());
            assert!(is_enabled::<Quat>());
            assert!(is_enabled::<Mat2>());
            assert!(is_enabled::<Mat3>());
            assert!(is_enabled::<Mat4>());
            assert!(is_enabled::<Affine3>());
        }

        // The archived form of the 16 byte aligned types drops the padding the native form
        // carries, so it is smaller and cannot be a copy. `Affine2` is deliberately absent:
        // it has tail padding on the SIMD backends but not under `scalar-math`.
        assert!(!is_enabled::<Vec3A>());
        assert!(!is_enabled::<Mat3A>());
        assert!(!is_enabled::<Affine3A>());
    }

    /// The archived types are the form data is uploaded to a GPU or written to a file in,
    /// so they have to be castable to bytes without going through a serializer.
    #[cfg(feature = "bytemuck")]
    #[test]
    fn archived_types_are_pod() {
        use crate::{ArchivedVec3, Vec3};

        let values = [
            ArchivedVec3::from(Vec3::new(1.0, 2.0, 3.0)),
            ArchivedVec3::from(Vec3::new(4.0, 5.0, 6.0)),
        ];

        let bytes = bytemuck::cast_slice::<ArchivedVec3, u8>(&values);
        assert_eq!(bytes.len(), 2 * core::mem::size_of::<ArchivedVec3>());
        assert_eq!(bytemuck::cast_slice::<u8, ArchivedVec3>(bytes), &values);
    }

    /// Converting an archived value back without a deserializer, which is all the leaf
    /// types need and is what makes the reference cast that used to work unnecessary.
    #[test]
    fn archived_conversions_round_trip() {
        use crate::{ArchivedMat2, ArchivedVec3, Mat2, Vec3};

        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::from(ArchivedVec3::from(vec)), vec);

        let mat = Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Mat2::from(ArchivedMat2::from(mat)), mat);
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
