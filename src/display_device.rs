pub trait DisplayDevice {
	fn show(&self, rt: &super::render_target::RenderTarget);
}

pub struct ConsoleDisplay;

impl DisplayDevice for ConsoleDisplay {
	fn show(&self, rt: &super::render_target::RenderTarget) {
		for y in 0..rt.height {
			for x in 0..rt.width {
				println!("{:?}", rt.pixels[y * rt.width + x]);
			}
		}
	}
}