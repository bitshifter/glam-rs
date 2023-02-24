mod lvec2;
mod lvec3;
mod lvec4;

pub use lvec2::{lvec2, LVec2};
pub use lvec3::{lvec3, LVec3};
pub use lvec4::{lvec4, LVec4};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    mod const_test_lvec2 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i64>(),
            core::mem::align_of::<super::LVec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::LVec2>());
        const_assert_eq!(16, core::mem::size_of::<super::LVec2>());
    }

    mod const_test_lvec3 {
        const_assert_eq!(
            core::mem::align_of::<i64>(),
            core::mem::align_of::<super::LVec3>()
        );
        const_assert_eq!(24, core::mem::size_of::<super::LVec3>());
    }

    mod const_test_lvec4 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<i64>(),
            core::mem::align_of::<super::LVec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::LVec4>());
        const_assert_eq!(32, core::mem::size_of::<super::LVec4>());
    }
}
