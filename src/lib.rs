#![doc(html_root_url = "https://docs.rs/glam/0.5.2")]

pub mod f32;
pub use self::f32::*;

#[repr(align(16))]
pub(crate) struct Align16<T>(T);
