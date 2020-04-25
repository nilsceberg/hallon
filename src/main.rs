mod render_target;
mod display_device;
mod math;

use math::*;
use display_device::DisplayDevice;

fn main() {
	let mut rt = render_target::RenderTarget::new(40, 30);
	let dd = display_device::ConsoleDisplay { rgb: true };

	rt.set_pixel(5, 5, &Vec4::new(1.0, 0.0, 0.0, 1.0));
	rt.set_pixel(2, 4, &Vec4::new(0.0, 0.5, 0.0, 1.0));

	let white = Vec4::new(1.0, 1.0, 1.0, 1.0);

	for x in 0..rt.width {
		rt.set_pixel(x, 0, &white);
		rt.set_pixel(x, rt.height - 1, &white);
	}

	for y in 0..rt.height {
		rt.set_pixel(0, y, &white);
		rt.set_pixel(rt.width - 1, y, &white);
	}

	dd.show(&rt);
}
