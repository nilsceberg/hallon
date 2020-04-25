use super::geometry::*;
use super::math::*;
use super::rasterizer;
use super::render_target::RenderTarget;
use super::shaders::*;

pub struct Renderer<'a> {
    pub target: &'a mut RenderTarget,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Renderer<'_> {
    pub fn draw<'b>(&'b mut self, mesh: &Mesh, fragment: &dyn FragmentShader) {
        for [a, b, c] in &mesh.triangles {
            let ssa = Vec2::new(a.x, a.y);
            let ssb = Vec2::new(b.x, b.y);
            let ssc = Vec2::new(c.x, c.y);

            rasterizer::triangle(self.target, fragment, &ssa, &ssb, &ssc);
        }
    }
}
