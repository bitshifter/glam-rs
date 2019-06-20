#![allow(dead_code)]
use glam::f32::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
use rand::Rng;

// pub fn random_type<T, R>(rng: &mut R) -> T where R: Rng, distributions::Standard: distributions::Distribution<T> {
//     rng.gen()
// }

pub fn random_vec2<R>(rng: &mut R) -> Vec2
where
    R: Rng,
{
    rng.gen()
}

pub fn random_vec3<R>(rng: &mut R) -> Vec3
where
    R: Rng,
{
    rng.gen()
}

pub fn random_vec4<R>(rng: &mut R) -> Vec4
where
    R: Rng,
{
    rng.gen()
}

pub fn random_nonzero_vec3<R>(rng: &mut R) -> Vec3
where
    R: Rng,
{
    loop {
        let v: Vec3 = rng.gen();
        if v.length_squared() > 0.01 {
            return v;
        }
    }
}

pub fn random_quat<R>(rng: &mut R) -> Quat
where
    R: Rng,
{
    let yaw = rng.gen();
    let pitch = rng.gen();
    let roll = rng.gen();
    Quat::from_rotation_ypr(yaw, pitch, roll)
}

pub fn random_mat2<R>(rng: &mut R) -> Mat2
where
    R: Rng,
{
    rng.gen()
}

pub fn random_mat3<R>(rng: &mut R) -> Mat3
where
    R: Rng,
{
    rng.gen()
}

pub fn random_srt_mat4<R>(rng: &mut R) -> Mat4
where
    R: Rng,
{
    Mat4::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
    )
}
