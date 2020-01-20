use super::Vec4;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<Vec4> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4 {
        rng.gen::<[f32; 4]>().into()
    }
}
