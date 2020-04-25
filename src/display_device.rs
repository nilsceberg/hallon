use super::math::*;

pub trait DisplayDevice {
    fn show(&self, rt: &super::render_target::RenderTarget);
    fn dimensions(&self) -> Option<(usize, usize)> {
        None
    }
}

pub struct ConsoleDisplay {
    pub rgb: bool,
}

impl DisplayDevice for ConsoleDisplay {
    fn show(&self, rt: &super::render_target::RenderTarget) {
        print!("\x1b[0;0H");
        let threshold = Vec4::new(0.5, 0.5, 0.5, 1.0).length();
        for y in 0..rt.height {
            for x in 0..rt.width {
                let pixel = rt.get_pixel(x, y);
                if self.rgb {
                    print!(
                        "\x1b[48;2;{};{};{}m  ",
                        (pixel.x * 255.0).round() as u8,
                        (pixel.y * 255.0).round() as u8,
                        (pixel.z * 255.0).round() as u8,
                    );
                } else {
                    if pixel.length() > threshold {
                        print!("██");
                    } else {
                        print!("  ");
                    }
                }
            }

            if y != rt.height - 1 {
                println!("\x1b[0m");
            }
        }
    }

    fn dimensions(&self) -> Option<(usize, usize)> {
        term_size::dimensions().map(|(w, h)| (w / 2, h))
    }
}
