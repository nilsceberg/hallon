mod display_device;
mod math;
mod rasterizer;
mod render_target;
mod shaders;

use math::*;

fn main() {
    let dd = display_device::ConsoleDisplay { rgb: true };

    loop {
        render(&dd);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn render(dd: &dyn display_device::DisplayDevice) {
    let mut rt = render_target::RenderTarget::new(dd.dimensions().unwrap_or((10, 10)));

    let white = Vec4::new(1.0, 1.0, 1.0, 1.0);

    for x in 0..rt.width {
        rt.set_pixel(x, 0, &white);
        rt.set_pixel(x, rt.height - 1, &white);
    }

    for y in 0..rt.height {
        rt.set_pixel(0, y, &white);
        rt.set_pixel(rt.width - 1, y, &white);
    }

    let green_shader = shaders::SolidShader(Vec4::new(0.0, 1.0, 0.0, 1.0));
    rasterizer::line(
        &mut rt,
        &shaders::RainbowShader,
        &Vec2::new(0.1, 0.2),
        &Vec2::new(0.3, 0.7),
    );
    rasterizer::line(
        &mut rt,
        &green_shader,
        &Vec2::new(0.1, 0.8),
        &Vec2::new(0.8, 0.3),
    );
    rasterizer::line(
        &mut rt,
        &green_shader,
        &Vec2::new(0.0, 0.0),
        &Vec2::new(1.0, 1.0),
    );
    rasterizer::line(
        &mut rt,
        &shaders::RainbowShader,
        &Vec2::new(0.0, 1.0),
        &Vec2::new(1.0, 0.0),
    );

    dd.show(&rt);
}
