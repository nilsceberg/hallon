use super::geometry::*;
use super::math::*;
use super::rasterizer;
use super::render_target::RenderTarget;
use super::shaders::*;

pub struct Renderer<'a> {
    pub target: &'a mut RenderTarget,
    view_matrix: Mat4x4,
    pub camera_position: Vec4,
}

fn view_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4x4 {
    let u = 1.0 / fov.tan();
    let a = 1.0 / aspect;
    let d = 1.0 / (far - near);

    Mat4x4([
        [u * a, 0.0, 0.0, 0.0],
        [0.0, u, 0.0, 0.0],
        [0.0, 0.0, d, d * near],
        [0.0, 0.0, 1.0, 0.0],
    ])
}

fn to_view_space(matrix: &Mat4x4, point: &Vec4) -> Vec4 {
    let mut point = *point;
    point.w = 1.0;
    point = matrix.mul(&point);
    point.x /= point.w;
    point.y /= point.w;
    point
}

impl Renderer<'_> {
    pub fn new<'a>(
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
        target: &'a mut RenderTarget,
        camera_position: &Vec4,
    ) -> Renderer<'a> {
        Renderer {
            target: target,
            camera_position: *camera_position,
            view_matrix: view_matrix(fov, aspect, near, far),
        }
    }

    pub fn draw<'b>(&'b mut self, mesh: &Mesh, fragment: &dyn FragmentShader) {
        for [a, b, c] in &mesh.triangles {
            // Camera space
            let mut csa = *a;
            csa.x -= self.camera_position.x;
            csa.y -= self.camera_position.y;
            csa.z -= self.camera_position.z;

            let mut csb = *b;
            csb.x -= self.camera_position.x;
            csb.y -= self.camera_position.y;
            csb.z -= self.camera_position.z;

            let mut csc = *c;
            csc.x -= self.camera_position.x;
            csc.y -= self.camera_position.y;
            csc.z -= self.camera_position.z;

            // View space
            let vsa = to_view_space(&self.view_matrix, &csa);
            let vsb = to_view_space(&self.view_matrix, &csb);
            let vsc = to_view_space(&self.view_matrix, &csc);

            // Screen space (ie view space without z and w)
            let ssa = Vec2::new(vsa.x, vsa.y);
            let ssb = Vec2::new(vsb.x, vsb.y);
            let ssc = Vec2::new(vsc.x, vsc.y);

            rasterizer::triangle(self.target, fragment, &ssa, &ssb, &ssc);
        }
    }
}
