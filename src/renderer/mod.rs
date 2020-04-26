use super::camera::*;
use super::geometry::*;
use super::math::*;
use super::rasterizer;
use super::render_target::RenderTarget;
use super::shaders::*;

pub struct Renderer<'a> {
    pub target: &'a mut RenderTarget,
    projection_matrix: Mat4x4,
    camera: &'a Camera,
}

fn projection_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4x4 {
    let u = 1.0 / fov.tan();
    let a = 1.0 / aspect;
    let d = 1.0 / (far - near);

    Mat4x4([
        [u * a, 0.0, 0.0, 0.0],
        [0.0, u, 0.0, 0.0],
        [0.0, 0.0, d, -d * near],
        [0.0, 0.0, 1.0, 0.0],
    ])
}

fn to_screen_space(matrix: &Mat4x4, point: &Vec3) -> Vec3 {
    let mut point = Vec4::new(point.x, point.y, point.z, 1.0);
    point = matrix.mul(&point);

    if point.z > 0.0 {
        Vec3::new(point.x / point.w, point.y / point.w, point.z)
    } else {
        // If Z is negative, the coordinate is behind the near clipping plane,
        // so don't divide by depth (as this could result in a division by zero for points
        // that are exactly on the camera's Z coordinate!).
        // This should be handled in a better way, but for now, let's just return
        // a dummy coordinate (which should make it obvious visually).
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Renderer<'_> {
    pub fn new<'a>(
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
        target: &'a mut RenderTarget,
        camera: &'a Camera,
    ) -> Renderer<'a> {
        Renderer {
            target: target,
            camera: camera,
            projection_matrix: projection_matrix(fov, aspect, near, far),
        }
    }

    pub fn draw<'b>(&'b mut self, mesh: &Mesh, transform: &Mat4x4, fragment: &dyn FragmentShader) {
        for [a, b, c] in &mesh.triangles {
            let matrix = self
                .projection_matrix
                .mat_mul(&self.camera.view_matrix().mat_mul(transform));

            let a = to_screen_space(&matrix, &a.position);
            let b = to_screen_space(&matrix, &b.position);
            let c = to_screen_space(&matrix, &c.position);

            let a = Vec2::new(a.x, a.y);
            let b = Vec2::new(b.x, b.y);
            let c = Vec2::new(c.x, c.y);

            rasterizer::triangle(self.target, fragment, &a, &b, &c, false);
        }
    }
}
