use super::math::*;
use super::render_target::RenderTarget;
use super::shaders::*;

pub fn map((a0, b0): (f32, f32), (a1, b1): (f32, f32), x: f32) -> f32 {
    (x - a0) / (b0 - a0) * (b1 - a1) + a1
}

pub fn approximate_pixel(size: usize, normalized: f32) -> i32 {
    let size = size as f32;
    let center_offset = 1.0 / (2.0 * size);
    let floating = map(
        (-1.0 + center_offset, 1.0 - center_offset),
        (0.0, size - 1.0),
        normalized,
    );
    floating.round() as i32
}

pub fn from_normalized((width, height): (usize, usize), point: &Vec2) -> (i32, i32) {
    (
        approximate_pixel(width, point.x),
        approximate_pixel(height, -point.y),
    )
}

pub fn triangle(rt: &mut RenderTarget, shader: &dyn FragmentShader, a: &Vec2, b: &Vec2, c: &Vec2) {
    line(rt, shader, a, b);
    line(rt, shader, b, c);
    line(rt, shader, c, a);
}

pub fn line(rt: &mut RenderTarget, shader: &dyn FragmentShader, a: &Vec2, b: &Vec2) {
    /* Naive: */
    let (x1, y1) = from_normalized(rt.dimensions(), a);
    let (x2, y2) = from_normalized(rt.dimensions(), b);

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
            let y = ((x as f32) * slope).round() as i32;
            pixel(rt, shader, x + x1, y + y1);
        }
    } else {
        let slope = (dx as f32) / (dy as f32);
        let mut y = 0;
        while y != dy {
            y += dy.signum();
            let x = ((y as f32) * slope).round() as i32;
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
