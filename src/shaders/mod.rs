use super::math::*;

pub struct FragmentInput<'a> {
    pub position: &'a Vec4,
}

pub trait FragmentShader {
    fn fragment_color(&self, pos: &FragmentInput) -> Vec4;
}

pub struct SolidShader(pub Vec4);

impl FragmentShader for SolidShader {
    fn fragment_color(&self, _input: &FragmentInput) -> Vec4 {
        let &SolidShader(color) = self;
        color
    }
}
