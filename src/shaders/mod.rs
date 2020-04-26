extern crate rand;

use super::math::*;
use rand::prelude::*;

pub struct FragmentInput<'a> {
    pub position: &'a Vec4,
    pub screen_uv: Vec2,
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

pub struct ScreenUVShader;
impl FragmentShader for ScreenUVShader {
    fn fragment_color(&self, input: &FragmentInput) -> Vec4 {
        Vec4::new(input.screen_uv.x, input.screen_uv.y, 0.0, 1.0)
    }
}

pub struct RainbowShader;
impl FragmentShader for RainbowShader {
    fn fragment_color(&self, _input: &FragmentInput) -> Vec4 {
        let mut rng = rand::thread_rng();
        return Vec4::new(rng.gen(), rng.gen(), rng.gen(), 1.0);
    }
}
