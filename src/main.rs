mod render_target;
mod display_device;
mod rasterizer;
mod math;

use math::*;
use display_device::DisplayDevice;

fn main() {
	let mut rt = render_target::RenderTarget::new(30, 20);
	let dd = display_device::ConsoleDisplay { rgb: true };

	let white = Vec4::new(1.0, 1.0, 1.0, 1.0);

	for x in 0..rt.width {
		rt.set_pixel(x, 0, &white);
		rt.set_pixel(x, rt.height - 1, &white);
	}

	for y in 0..rt.height {
		rt.set_pixel(0, y, &white);
		rt.set_pixel(rt.width - 1, y, &white);
	}

	rasterizer::line(&mut rt, &Vec2::new(0.1, 0.2), &Vec2::new(0.3, 0.7));
	rasterizer::line(&mut rt, &Vec2::new(0.0, 0.0), &Vec2::new(1.0, 1.0));
	rasterizer::line(&mut rt, &Vec2::new(0.0, 1.0), &Vec2::new(1.0, 0.0));
	rasterizer::line(&mut rt, &Vec2::new(0.1, 0.8), &Vec2::new(0.8, 0.3));

	dd.show(&rt);
}
