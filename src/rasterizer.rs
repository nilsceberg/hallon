use super::math::*;
use super::render_target::RenderTarget;

pub fn line(rt: &mut RenderTarget, a: &Vec2, b: &Vec2) {
    /* Bresenham's algorithm? */

    /* Naive: */
    let width = rt.width as f32;
    let height = rt.height as f32;

    let x1 = (a.x * width).round() as i32;
    let y1 = ((1.0 - a.y) * height).round() as i32;

    let x2 = (b.x * width).round() as i32;
    let y2 = ((1.0 - b.y) * height).round() as i32;

    if x1 == x2 && y1 == y2 {
        pixel(rt, x1, x2);
        return;
    }

    let dx = x2 - x1;
    let dy = y2 - y1;

    pixel(rt, x1, y1);

    if dx.abs() >= dy.abs() {
        let slope = (dy as f32) / (dx as f32);
        let mut x = 0;
        while x != dx {
            x += dx.signum();
            let y = ((x as f32) * slope).floor() as i32;
            pixel(rt, x + x1, y + y1);
        }
    } else {
        let slope = (dx as f32) / (dy as f32);
        let mut y = 0;
        while y != dy {
            y += dy.signum();
            let x = ((y as f32) * slope).floor() as i32;
            pixel(rt, x + x1, y + y1);
        }
    }
}

fn pixel(rt: &mut RenderTarget, x: i32, y: i32) -> bool {
    if x >= 0 && x < (rt.width as i32) && y >= 0 && y < (rt.height as i32) {
        rt.set_pixel(x as usize, y as usize, &Vec4::new(1.0, 0.0, 0.0, 1.0));
        true
    } else {
        false
    }
}
