extern crate ctrlc;
extern crate failure;

mod camera;
mod display_device;
mod geometry;
mod loaders;
mod math;
mod object;
mod rasterizer;
mod render_target;
mod renderer;
mod shaders;

use display_device::DisplayDevice;
use math::*;

fn main() {
    let dd = display_device::ConsoleDisplay { rgb: true };

    static STOP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    ctrlc::set_handler(|| {
        STOP.store(true, std::sync::atomic::Ordering::Relaxed);
    })
    .unwrap();

    let cube_mesh = geometry::Mesh {
        triangles: vec![
            // Front
            [
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(1.0, 1.0, -1.0),
            ],
            [
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(1.0, -1.0, -1.0),
            ],
            // Right
            [
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(1.0, 1.0, 1.0),
            ],
            [
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, -1.0, 1.0),
            ],
            // Top
            [
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(-1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
            ],
            [
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, -1.0),
            ],
            // Back
            [
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(-1.0, 1.0, 1.0),
            ],
            [
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(-1.0, 1.0, 1.0),
                Vec3::new(-1.0, -1.0, 1.0),
            ],
            // Left
            [
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(-1.0, 1.0, 1.0),
                Vec3::new(-1.0, 1.0, -1.0),
            ],
            [
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(-1.0, -1.0, -1.0),
            ],
            // Bottom
            [
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(-1.0, -1.0, 1.0),
            ],
            [
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(-1.0, -1.0, -1.0),
            ],
        ],
    };

    let mut objects: Vec<object::Object> = vec![];

    // Load sphere
    let sphere_path = std::path::Path::new("models/sphere.obj");
    let sphere_mesh = loaders::obj::load(&sphere_path).unwrap();

    objects.push(object::Object::new(&cube_mesh));
    //objects.push(object::Object::new(&sphere_mesh));

    let time_step = 1.0 / 30.0;
    let mut t: f32 = 0.0;
    let mut camera = camera::Camera {
        translation: Vec3::new(0.0, 0.0, -3.0),
    };

    dd.setup();
    let mut dimensions: (usize, usize) = (10, 10);
    let mut rt = render_target::RenderTarget::new(dimensions);

    while !STOP.load(std::sync::atomic::Ordering::Relaxed) {
        let new_dimensions = dd.dimensions().unwrap_or((20, 10));
        if new_dimensions != dimensions {
            dimensions = new_dimensions;
            rt = render_target::RenderTarget::new(dimensions);
        }

        objects[0].rotation.y = t;

        dd.prepare();
        rt.clear(&Vec4::new(0.3, 0.3, 0.3, 1.0));

        render(&mut rt, &objects, &camera);

        rasterizer::line(
            &mut rt,
            &shaders::SolidShader(Vec4::new(1.0, 0.0, 0.0, 1.0)),
            &Vec2::new(-1.0, -1.0),
            &Vec2::new(-1.0, 1.0),
        );

        rasterizer::line(
            &mut rt,
            &shaders::SolidShader(Vec4::new(1.0, 0.0, 0.0, 1.0)),
            &Vec2::new(1.0, -1.0),
            &Vec2::new(1.0, 1.0),
        );

        rasterizer::line(
            &mut rt,
            &shaders::SolidShader(Vec4::new(1.0, 0.0, 0.0, 1.0)),
            &Vec2::new(-1.0, 1.0),
            &Vec2::new(1.0, 1.0),
        );

        rasterizer::line(
            &mut rt,
            &shaders::SolidShader(Vec4::new(1.0, 0.0, 0.0, 1.0)),
            &Vec2::new(-1.0, -1.0),
            &Vec2::new(1.0, -1.0),
        );

        dd.show(&rt);

        std::thread::sleep(std::time::Duration::from_millis(
            (time_step * 1000.0) as u64,
        ));

        t += time_step;
    }
    dd.restore();
}

fn render(
    rt: &mut render_target::RenderTarget,
    objects: &Vec<object::Object>,
    camera: &camera::Camera,
) {
    let mut renderer = renderer::Renderer::new(
        std::f32::consts::PI / 4.0,
        rt.aspect_ratio(),
        0.1,
        100.0,
        rt,
        &camera,
    );

    let white = Vec4::new(1.0, 1.0, 1.0, 1.0);
    let shader = shaders::SolidShader(white);
    for object in objects {
        renderer.draw(object.mesh, &object.transform(), &shader);
    }
}
