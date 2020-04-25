use super::math::*;

pub trait DisplayDevice {
	fn show(&self, rt: &super::render_target::RenderTarget);
}

pub struct ConsoleDisplay;

impl DisplayDevice for ConsoleDisplay {
	fn show(&self, rt: &super::render_target::RenderTarget) {
		let threshold = Vec4::new(0.5, 0.5, 0.5, 1.0).length();
		for y in 0..rt.height {
			for x in 0..rt.width {
				let pixel = rt.get_pixel(x, y);
				if pixel.length() > threshold {
					print!("██");
				}
				else {
					print!("  ");
				}
			}
			println!("");
		}
	}
}
