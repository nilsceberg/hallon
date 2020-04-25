use super::math::*;
use super::render_target::RenderTarget;
use super::shaders::*;

pub fn triangle(rt: &mut RenderTarget, shader: &dyn FragmentShader, a: &Vec2, b: &Vec2, c: &Vec2) {
    line(rt, shader, a, b);
    line(rt, shader, b, c);
    line(rt, shader, c, a);
}

pub fn line(rt: &mut RenderTarget, shader: &dyn FragmentShader, a: &Vec2, b: &Vec2) {
    /* Bresenham's algorithm? */

    /* Naive: */
    let width = (rt.width as f32) - 1.0;
    let height = (rt.height as f32) - 1.0;

    let x1 = ((a.x + 1.0) * 0.5 * width).round() as i32;
    let y1 = ((1.0 - (a.y + 1.0) * 0.5) * height).round() as i32;

    let x2 = ((b.x + 1.0) * 0.5 * width).round() as i32;
    let y2 = ((1.0 - (b.y + 1.0) * 0.5) * height).round() as i32;

    pixel(rt, shader, x1, y1);

    if x1 == x2 && y1 == y2 {
        return;
    }

    let dx = x2 - x1;
    let dy = y2 - y1;

    if dx.abs() >= dy.abs() {
        let slope = (dy as f32) / (dx as f32);
        let mut x = 0;
        while x != dx {
            x += dx.signum();
            let y = ((x as f32) * slope).floor() as i32;
            pixel(rt, shader, x + x1, y + y1);
        }
    } else {
        let slope = (dx as f32) / (dy as f32);
        let mut y = 0;
        while y != dy {
            y += dy.signum();
            let x = ((y as f32) * slope).floor() as i32;
            pixel(rt, shader, x + x1, y + y1);
        }
    }
}

fn pixel(rt: &mut RenderTarget, shader: &dyn FragmentShader, x: i32, y: i32) -> bool {
    if x >= 0 && x < (rt.width as i32) && y >= 0 && y < (rt.height as i32) {
        let input = FragmentInput {
            position: &Vec4::new(
                (x as f32) * (rt.width as f32),
                (y as f32) * (rt.height as f32),
                0.0,
                1.0,
            ),
        };

        let color = shader.fragment_color(&input);
        rt.set_pixel(x as usize, y as usize, &color);

        true
    } else {
        false
    }
}
