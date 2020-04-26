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

use math::*;

fn main() {
    let dd = display_device::ConsoleDisplay { rgb: true };

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

    //objects.push(object::Object::new(&cube_mesh));
    objects.push(object::Object::new(&sphere_mesh));

    let time_step = 1.0 / 30.0;
    let mut t: f32 = 0.0;
    let mut camera = camera::Camera {
        translation: Vec3::new(0.0, 0.0, -2.0),
    };

    loop {
        render(&dd, &objects, &camera);
        objects[0].rotation.y = t;
        objects[0].rotation.z = t;
        objects[0].rotation.z = t;

        //objects[0].scale.y = 1.5 + (t * 4.0 + 3.1415 * 0.5).sin() * 0.5;
        //objects[0].scale.x = 1.0 - (t * 4.0 + 3.1415 * 0.5).sin() * 0.2;
        //objects[0].scale.z = 1.0 - (t * 4.0 + 3.1415 * 0.5).sin() * 0.2;

        std::thread::sleep(std::time::Duration::from_millis(
            (time_step * 1000.0) as u64,
        ));

        t += time_step;
    }
}

fn render(
    dd: &dyn display_device::DisplayDevice,
    objects: &Vec<object::Object>,
    camera: &camera::Camera,
) {
    let mut rt = render_target::RenderTarget::new(dd.dimensions().unwrap_or((10, 10)));

    {
        let mut renderer = renderer::Renderer::new(
            std::f32::consts::PI / 4.0,
            rt.aspect_ratio(),
            0.1,
            100.0,
            &mut rt,
            &camera,
        );

        let white = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let shader = shaders::SolidShader(white);
        for object in objects {
            renderer.draw(object.mesh, &object.transform(), &shader);
        }
    }

    dd.show(&rt);
}
