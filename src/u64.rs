mod ulvec2;
mod ulvec3;
mod ulvec4;

pub use ulvec2::{ulvec2, ULVec2};
pub use ulvec3::{ulvec3, ULVec3};
pub use ulvec4::{ulvec4, ULVec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;
    mod const_test_ulvec2 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u64>(),
            core::mem::align_of::<super::ULVec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::ULVec2>());
        const_assert_eq!(16, core::mem::size_of::<super::ULVec2>());
    }

    mod const_test_ulvec3 {
        const_assert_eq!(
            core::mem::align_of::<u64>(),
            core::mem::align_of::<super::ULVec3>()
        );
        const_assert_eq!(24, core::mem::size_of::<super::ULVec3>());
    }

    mod const_test_ulvec4 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<u64>(),
            core::mem::align_of::<super::ULVec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::ULVec4>());
        const_assert_eq!(32, core::mem::size_of::<super::ULVec4>());
    }
}
