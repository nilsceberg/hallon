extern crate rand;

use super::geometry::*;
use super::math::*;
use rand::prelude::*;

pub struct FragmentInput {
    pub vertex: Vertex,
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

pub struct DebugShader;
impl FragmentShader for DebugShader {
    fn fragment_color(&self, input: &FragmentInput) -> Vec4 {
        Vec4::new(
            map((-1.0, 1.0), (0.0, 1.0), input.vertex.normal.x),
            map((-1.0, 1.0), (0.0, 1.0), input.vertex.normal.y),
            map((-1.0, 1.0), (0.0, 1.0), input.vertex.normal.z),
            1.0,
        )
    }
}

pub struct DiffuseShader(pub Vec3);
impl FragmentShader for DiffuseShader {
    fn fragment_color(&self, input: &FragmentInput) -> Vec4 {
        let &DiffuseShader(light_direction) = self;
        let light_direction = light_direction.mul(-1.0);

        // This expects vertex normals in world-space.
        Vec4::from_vec3(
            input.vertex.color.xyz().mul(map(
                (-1.0, 1.0),
                (0.0, 1.0),
                input.vertex.normal.dot(&light_direction),
            )),
            1.0,
        )
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
