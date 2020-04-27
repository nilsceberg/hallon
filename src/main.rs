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
use std::collections::HashMap;

fn main() {
    let dd = display_device::ConsoleDisplay { rgb: true };

    static STOP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    ctrlc::set_handler(|| {
        STOP.store(true, std::sync::atomic::Ordering::Relaxed);
    })
    .unwrap();

    let tri_mesh = geometry::Mesh {
        triangles: vec![[
            geometry::Vertex {
                position: Vec3::new(-1.0, -1.0, 0.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                uv: Vec2::new(0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, -1.0),
            },
            geometry::Vertex {
                position: Vec3::new(-1.0, 1.0, 0.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                uv: Vec2::new(0.0, 1.0),
                normal: Vec3::new(0.0, 0.0, -1.0),
            },
            geometry::Vertex {
                position: Vec3::new(1.0, 1.0, 0.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                uv: Vec2::new(1.0, 1.0),
                normal: Vec3::new(0.0, 0.0, -1.0),
            },
        ]],
    };

    let mut objects: Vec<object::Object> = vec![];

    let rabbit_mesh = loaders::obj::load(&std::path::Path::new("models/rabbit.obj"), None).unwrap();
    let tree_mesh = loaders::obj::load(&std::path::Path::new("models/tree.obj"), None).unwrap();

    let cube_mesh = loaders::obj::load(
        &std::path::Path::new("models/cube.obj"),
        Some(&HashMap::new()),
    )
    .unwrap();

    //objects.push(object::Object::new(&cube_mesh));
    let mut rabbit = object::Object::new(&rabbit_mesh);
    let mut tree = object::Object::new(&tree_mesh);

    rabbit.translation.x = 1.0;
    rabbit.translation.y = -0.5;

    tree.translation.x = -1.0;
    tree.translation.y = -1.0;
    tree.scale = tree.scale.mul(0.5);

    objects.push(rabbit);
    objects.push(tree);

    let time_step = 1.0 / 30.0;
    let mut t: f32 = 0.0;
    let mut camera = camera::Camera {
        translation: Vec3::new(0.0, 0.0, -1.5),
    };

    dd.setup();
    let mut dimensions: (usize, usize) = (10, 10);
    let mut rt = render_target::RenderTarget::new(dimensions);
    let mut depth = render_target::RenderTarget::new(dimensions);

    while !STOP.load(std::sync::atomic::Ordering::Relaxed) {
        let new_dimensions = dd.dimensions().unwrap_or((20, 10));
        if new_dimensions != dimensions {
            dimensions = new_dimensions;
            rt = render_target::RenderTarget::new(dimensions);
            depth = render_target::RenderTarget::new(dimensions);
        }

        for object in &mut objects {
            object.rotation.y = t;
        }

        dd.prepare();
        rt.clear(&Vec4::new(0.3, 0.3, 0.3, 1.0));
        depth.clear(&Vec4::new(1.0, 1.0, 1.0, 1.0));

        render(&mut rt, &mut depth, &objects, &camera);

        // Currently broken because line doesn't work without a depth map
        // because I can't figure out how to make it work with the borrow checker.
        rasterizer::line_2d(
            &mut rt,
            &shaders::SolidShader(Vec4::new(1.0, 0.0, 0.0, 1.0)),
            &Vec2::new(-1.0, -1.0),
            &Vec2::new(-1.0, 1.0),
        );

        rasterizer::line_2d(
            &mut rt,
            &shaders::SolidShader(Vec4::new(1.0, 0.0, 0.0, 1.0)),
            &Vec2::new(1.0, -1.0),
            &Vec2::new(1.0, 1.0),
        );

        rasterizer::line_2d(
            &mut rt,
            &shaders::SolidShader(Vec4::new(1.0, 0.0, 0.0, 1.0)),
            &Vec2::new(-1.0, 1.0),
            &Vec2::new(1.0, 1.0),
        );

        rasterizer::line_2d(
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
    depth: &mut render_target::RenderTarget,
    objects: &Vec<object::Object>,
    camera: &camera::Camera,
) {
    let mut renderer = renderer::Renderer::new(
        std::f32::consts::PI / 4.0,
        rt.aspect_ratio(),
        0.1,
        10.0,
        rt,
        depth,
        &camera,
    );

    let white = Vec4::new(1.0, 1.0, 1.0, 1.0);
    let shader = shaders::SolidShader(white);
    let diffuse_shader = shaders::DiffuseShader(Vec3::new(-0.707, -0.707, 0.0));
    for object in objects {
        renderer.draw(object.mesh, &object.transform(), &diffuse_shader);
    }
}
