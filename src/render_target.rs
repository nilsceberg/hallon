use super::math::*;

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pixels: Vec<super::math::Vec4>,
}

impl RenderTarget {
    pub fn new((width, height): (usize, usize)) -> RenderTarget {
        RenderTarget {
            width: width,
            height: height,
            pixels: vec![
                Vec4 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 1.0
                };
                width * height
            ],
        }
    }

    pub fn clear(&mut self, color: &Vec4) {
        for pixel in &mut self.pixels {
            *pixel = *color;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Vec4) {
        self.pixels[y * self.width + x] = Vec4::new(
            clamp((0.0, 1.0), color.x),
            clamp((0.0, 1.0), color.y),
            clamp((0.0, 1.0), color.z),
            clamp((0.0, 1.0), color.w),
        );
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vec4 {
        self.pixels[y * self.width + x]
    }

    pub fn aspect_ratio(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
