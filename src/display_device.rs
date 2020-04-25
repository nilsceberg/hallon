use super::math::*;

pub trait DisplayDevice {
	fn show(&self, rt: &super::render_target::RenderTarget);
}

pub struct ConsoleDisplay {
	pub rgb: bool
}

impl DisplayDevice for ConsoleDisplay {
	fn show(&self, rt: &super::render_target::RenderTarget) {
		print!("\x1b[0;0H");
		let threshold = Vec4::new(0.5, 0.5, 0.5, 1.0).length();
		for y in 0..rt.height {
			for x in 0..rt.width {
				let pixel = rt.get_pixel(x, y);
				if self.rgb {
					print!("\x1b[48;2;{};{};{}m  ",
						(pixel.x * 255.0).round(),
						(pixel.y * 255.0).round(),
						(pixel.z * 255.0).round(),
					);
				}
				else {
					if pixel.length() > threshold {
						print!("██");
					}
					else {
						print!("  ");
					}
				}
			}
			println!("\x1b[0m");
		}
	}
}
