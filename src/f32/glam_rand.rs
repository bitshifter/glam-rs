use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<Mat2> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat2 {
        Mat2::from_cols_array(&rng.gen())
    }
}

impl Distribution<Mat3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat3 {
        Mat3::from_cols_array(&rng.gen())
    }
}

impl Distribution<Mat4> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat4 {
        Mat4::from_cols_array(&rng.gen())
    }
}

impl Distribution<Quat> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quat {
        use core::f32::consts::PI;
        let yaw = -PI + rng.gen::<f32>() * 2.0 * PI;
        let pitch = -PI + rng.gen::<f32>() * 2.0 * PI;
        let roll = -PI + rng.gen::<f32>() * 2.0 * PI;
        Quat::from_rotation_ypr(yaw, pitch, roll)
    }
}

impl Distribution<Vec2> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2 {
        rng.gen::<[f32; 2]>().into()
    }
}

impl Distribution<Vec3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        rng.gen::<[f32; 3]>().into()
    }
}

impl Distribution<Vec3A> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3A {
        rng.gen::<[f32; 3]>().into()
    }
}

impl Distribution<Vec4> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4 {
        rng.gen::<[f32; 4]>().into()
    }
}

#[test]
fn test_mat2_rand() {
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a = Mat2::from_cols_array(&rng1.gen::<[f32; 4]>());
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b = rng2.gen::<Mat2>();
    assert_eq!(a, b);
}

#[test]
fn test_mat3_rand() {
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a = Mat3::from_cols_array(&rng1.gen::<[f32; 9]>());
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b = rng2.gen::<Mat3>();
    assert_eq!(a, b);
}

#[test]
fn test_mat4_rand() {
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a = Mat4::from_cols_array(&rng1.gen::<[f32; 16]>());
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b = rng2.gen::<Mat4>();
    assert_eq!(a, b);
}

#[test]
fn test_quat_rand() {
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a: Quat = rng1.gen();
    assert!(a.is_normalized());
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b: Quat = rng2.gen();
    assert_eq!(a, b);
}

#[test]
fn test_vec2_rand() {
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a: (f32, f32) = rng1.gen();
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b: Vec2 = rng2.gen();
    assert_eq!(a, b.into());
}

#[test]
fn test_vec3_rand() {
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a: (f32, f32, f32) = rng1.gen();
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b: Vec3 = rng2.gen();
    assert_eq!(a, b.into());
}

#[test]
fn test_vec4_rand() {
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a: (f32, f32, f32, f32) = rng1.gen();
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b: Vec4 = rng2.gen();
    assert_eq!(a, b.into());
}
