pub trait VecN {
	fn length(&self) -> f32;
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2 {
		Vec2 {
			x: x,
			y: y,
		}
	}
}

impl Vec4 {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
		Vec4 {
			x: x,
			y: y,
			z: z,
			w: w,
		}
	}
}

impl VecN for Vec4 {
	fn length(&self) -> f32 {
		(self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
	}
}
