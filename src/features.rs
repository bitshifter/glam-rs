#[cfg(feature = "approx-05")]
pub mod impl_approx_05;

#[cfg(feature = "float_eq")]
pub mod impl_float_eq;

#[cfg(feature = "bytemuck")]
pub mod impl_bytemuck;

#[cfg(feature = "mint-05")]
pub mod impl_mint_05;

#[cfg(feature = "rand-010")]
pub mod impl_rand_010;

#[cfg(feature = "serde")]
pub mod impl_serde;

#[cfg(feature = "speedy-08")]
pub mod impl_speedy_08;

#[cfg(feature = "rkyv-08")]
pub mod impl_rkyv_08;

#[cfg(all(feature = "rkyv", not(feature = "rkyv-08")))]
pub mod impl_rkyv;

#[cfg(feature = "encase-012")]
mod impl_encase_012 {
    use encase_012 as encase;
    include!("features/impl_encase.rs");
}

#[cfg(feature = "encase-013")]
mod impl_encase_013 {
    use encase_013 as encase;
    include!("features/impl_encase.rs");
}

#[cfg(feature = "zerocopy")]
pub mod impl_zerocopy;

#[cfg(feature = "arbitrary")]
pub mod impl_arbitrary;
