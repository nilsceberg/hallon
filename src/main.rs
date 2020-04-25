mod display_device;
mod geometry;
mod math;
mod rasterizer;
mod render_target;
mod renderer;
mod shaders;

use math::*;

fn main() {
    let dd = display_device::ConsoleDisplay { rgb: true };

    let mut meshes: Vec<geometry::Mesh> = vec![];

    meshes.push(geometry::Mesh {
        triangles: vec![
            // Front
            [
                Vec4::new(-1.0, -1.0, -1.0, 1.0),
                Vec4::new(-1.0, 1.0, -1.0, 1.0),
                Vec4::new(1.0, 1.0, -1.0, 1.0),
            ],
            [
                Vec4::new(-1.0, -1.0, -1.0, 1.0),
                Vec4::new(1.0, 1.0, -1.0, 1.0),
                Vec4::new(1.0, -1.0, -1.0, 1.0),
            ],
            // Right
            [
                Vec4::new(1.0, -1.0, -1.0, 1.0),
                Vec4::new(1.0, 1.0, -1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
            ],
            [
                Vec4::new(1.0, -1.0, -1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, -1.0, 1.0, 1.0),
            ],
            // Top
            [
                Vec4::new(-1.0, 1.0, -1.0, 1.0),
                Vec4::new(-1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
            ],
            [
                Vec4::new(-1.0, 1.0, -1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, -1.0, 1.0),
            ],
            // Back
            [
                Vec4::new(1.0, -1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(-1.0, 1.0, 1.0, 1.0),
            ],
            [
                Vec4::new(1.0, -1.0, 1.0, 1.0),
                Vec4::new(-1.0, 1.0, 1.0, 1.0),
                Vec4::new(-1.0, -1.0, 1.0, 1.0),
            ],
            // Left
            [
                Vec4::new(-1.0, -1.0, 1.0, 1.0),
                Vec4::new(-1.0, 1.0, 1.0, 1.0),
                Vec4::new(-1.0, 1.0, -1.0, 1.0),
            ],
            [
                Vec4::new(-1.0, -1.0, 1.0, 1.0),
                Vec4::new(-1.0, 1.0, -1.0, 1.0),
                Vec4::new(-1.0, -1.0, -1.0, 1.0),
            ],
            // Bottom
            [
                Vec4::new(1.0, -1.0, -1.0, 1.0),
                Vec4::new(1.0, -1.0, 1.0, 1.0),
                Vec4::new(-1.0, -1.0, 1.0, 1.0),
            ],
            [
                Vec4::new(1.0, -1.0, -1.0, 1.0),
                Vec4::new(-1.0, -1.0, 1.0, 1.0),
                Vec4::new(-1.0, -1.0, -1.0, 1.0),
            ],
        ],
    });

    let time_step = 0.1;
    let mut t: f32 = 0.0;
    let mut camera = Vec4::new(0.0, 0.0, -3.0, 1.0);

    loop {
        render(&dd, &meshes, camera);
        camera.x = t.sin() * 2.0;
        std::thread::sleep(std::time::Duration::from_millis(
            (time_step * 1000.0) as u64,
        ));
        t += time_step;
    }
}

fn render(dd: &dyn display_device::DisplayDevice, meshes: &Vec<geometry::Mesh>, camera: Vec4) {
    let mut rt = render_target::RenderTarget::new(dd.dimensions().unwrap_or((10, 10)));

    {
        let mut renderer = renderer::Renderer {
            aspect: rt.aspect_ratio(),
            target: &mut rt,
            fov: std::f32::consts::PI / 4.0,
            near: 0.1,
            far: 100.0,
            camera_position: camera,
        };

        let white = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let shader = shaders::SolidShader(white);
        for mesh in meshes {
            renderer.draw(mesh, &shader);
        }
    }

    dd.show(&rt);
}
