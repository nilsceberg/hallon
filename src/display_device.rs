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

fn set_terminal_rgb((r, g, b): (u8, u8, u8)) {
    print!("\x1b[48;2;{};{};{}m", r, g, b);
}

impl ConsoleDisplay {
    pub fn setup(&self) {
        print!("\x1b[?25l");
    }

    pub fn prepare(&self) {
        print!("\x1b[0;0H");
    }

    pub fn restore(&self) {
        print!("\x1b[?25h");
    }
}

impl DisplayDevice for ConsoleDisplay {
    fn show(&self, rt: &super::render_target::RenderTarget) {
        let threshold = Vec4::new(0.5, 0.5, 0.5, 1.0).length();

        let mut rgb: (u8, u8, u8) = (0, 0, 0);

        if self.rgb {
            set_terminal_rgb(rgb);
        }

        for y in 0..rt.height {
            for x in 0..rt.width {
                let pixel = rt.get_pixel(x, y);
                if self.rgb {
                    let new_rgb = (
                        (pixel.x * 255.0).round() as u8,
                        (pixel.y * 255.0).round() as u8,
                        (pixel.z * 255.0).round() as u8,
                    );

                    if new_rgb != rgb {
                        rgb = new_rgb;
                        set_terminal_rgb(rgb);
                    }

                    print!("  ");
                } else {
                    if pixel.length() > threshold {
                        print!("██");
                    } else {
                        print!("  ");
                    }
                }
            }

            if y != rt.height - 1 {
                println!("");
            } else if self.rgb {
                print!("\x1b[0m");
            }
        }
    }

    fn dimensions(&self) -> Option<(usize, usize)> {
        term_size::dimensions().map(|(w, h)| (w / 2, h))
    }
}
