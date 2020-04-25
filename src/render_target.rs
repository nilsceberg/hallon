pub struct RenderTarget {
	pub width: usize,
	pub height: usize,
	pub pixels: Vec<super::math::Vec4>
}

impl RenderTarget {
	pub fn new(width: usize, height: usize) -> RenderTarget {
		RenderTarget {
			width: width,
			height: height,
			pixels: vec![super::math::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }; width * height]
		}
	}

	pub fn set_pixel(&mut self, x: usize, y: usize, color: &super::math::Vec4) {
		self.pixels[y * self.width + x] = *color
	}
}

