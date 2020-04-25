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

fn to_view_space(fov: f32, aspect: f32, near: f32, far: f32, point: &Vec4) -> Vec4 {
    let u = 1.0 / (fov.tan() * point.z);
    Vec4::new(
        point.x / aspect * u,
        point.y * u,
        (point.z - near) / (far - near),
        1.0,
    )
}

impl Renderer<'_> {
    pub fn draw<'b>(&'b mut self, mesh: &Mesh, fragment: &dyn FragmentShader) {
        for [a, b, c] in &mesh.triangles {
            let camera_z = -3.0;
            // Camera space
            let mut csa = *a;
            csa.z -= camera_z;

            let mut csb = *b;
            csb.z -= camera_z;

            let mut csc = *c;
            csc.z -= camera_z;

            // View space
            let vsa = to_view_space(self.fov, self.aspect, self.near, self.far, &csa);
            let vsb = to_view_space(self.fov, self.aspect, self.near, self.far, &csb);
            let vsc = to_view_space(self.fov, self.aspect, self.near, self.far, &csc);

            // Screen space (ie view space without z and w)
            let ssa = Vec2::new(vsa.x, vsa.y);
            let ssb = Vec2::new(vsb.x, vsb.y);
            let ssc = Vec2::new(vsc.x, vsc.y);

            rasterizer::triangle(self.target, fragment, &ssa, &ssb, &ssc);
        }
    }
}
