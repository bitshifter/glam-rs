#[path = "support/macros.rs"]
#[macro_use]
mod macros;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::Vec2;

euler!(vec2_euler, "vec2 euler", ty => Vec2, storage => Vec2, zero => Vec2::zero());
euler!(vec2_tuple_euler, "vec2 tuple euler", ty => Vec2, storage => (f32, f32), zero => (0.0, 0.0));

criterion_group!(
    benches,
    vec2_euler,
    vec2_tuple_euler,
);

criterion_main!(benches);
