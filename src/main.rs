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
            [
                Vec4::new(0.0, 0.0, 0.0, 1.0),
                Vec4::new(0.0, 1.0, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 0.0, 1.0),
            ],
            [
                Vec4::new(0.0, 0.0, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 0.0, 1.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
            ],
        ],
    });

    loop {
        render(&dd, &meshes);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn render(dd: &dyn display_device::DisplayDevice, meshes: &Vec<geometry::Mesh>) {
    let mut rt = render_target::RenderTarget::new(dd.dimensions().unwrap_or((10, 10)));

    {
        let mut renderer = renderer::Renderer {
            aspect: rt.aspect_ratio(),
            target: &mut rt,
            fov: std::f32::consts::PI / 4.0,
            near: 0.1,
            far: 100.0,
        };

        // let white = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let green_shader = shaders::SolidShader(Vec4::new(0.0, 1.0, 0.0, 1.0));

        for mesh in meshes {
            renderer.draw(mesh, &green_shader);
        }
    }

    dd.show(&rt);
}
