extern crate rand;

use super::math::*;
use rand::prelude::*;

pub struct FragmentInput<'a> {
    pub position: &'a Vec4,
}

pub trait FragmentShader {
    fn fragment_color(&self, input: &FragmentInput) -> Vec4;
}

pub struct SolidShader(pub Vec4);
impl FragmentShader for SolidShader {
    fn fragment_color(&self, _input: &FragmentInput) -> Vec4 {
        let &SolidShader(color) = self;
        color
    }
}

pub struct RainbowShader;
impl FragmentShader for RainbowShader {
    fn fragment_color(&self, _input: &FragmentInput) -> Vec4 {
        let mut rng = rand::thread_rng();
        return Vec4::new(rng.gen(), rng.gen(), rng.gen(), 1.0);
    }
}
