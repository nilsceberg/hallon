use super::math::*;
use super::render_target::RenderTarget;
use super::shaders::*;

fn approximate_pixel(size: usize, normalized: f32) -> i32 {
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

pub fn to_normalized((width, height): (usize, usize), (x, y): (i32, i32)) -> Vec2 {
    Vec2::new(
        map((0.0, width as f32 - 1.0), (-1.0, 1.0), x as f32),
        -map((0.0, height as f32 - 1.0), (-1.0, 1.0), y as f32),
    )
}

pub fn triangle(
    rt: &mut RenderTarget,
    shader: &dyn FragmentShader,
    a: &Vec2,
    b: &Vec2,
    c: &Vec2,
    wireframe: bool,
) {
    if wireframe {
        line(rt, shader, a, b);
        line(rt, shader, b, c);
        line(rt, shader, c, a);
    } else {
        triangle_parallel(rt, shader, a, b, c);
    }
}

// A method that works great in parallell (read: on hardware)
// but perhaps isn't great for CPUs. We check every pixel
// (as an optimization, only those in a rectangle that just covers the triangle),
// and check whether they are inside the triangle using cross products.

fn triangle_parallel(
    rt: &mut RenderTarget,
    shader: &dyn FragmentShader,
    a: &Vec2,
    b: &Vec2,
    c: &Vec2,
) {
    let (x0, y0) = from_normalized(rt.dimensions(), a);
    let (x1, y1) = from_normalized(rt.dimensions(), b);
    let (x2, y2) = from_normalized(rt.dimensions(), c);

    let left = x0.min(x1).min(x2);
    let right = x0.max(x1).max(x2);
    let top = y0.min(y1).min(y2);
    let bottom = y0.max(y1).max(y2);

    for x in left..(right + 1) {
        for y in top..(bottom + 1) {
            if right_side((x0, y0), (x1, y1), (x, y))
                && right_side((x1, y1), (x2, y2), (x, y))
                && right_side((x2, y2), (x0, y0), (x, y))
            {
                pixel(rt, shader, x, y);
            }
        }
    }

    let black = super::shaders::SolidShader(Vec4::new(0.0, 0.0, 0.0, 1.0));
    line(rt, &black, a, b);
    line(rt, &black, b, c);
    line(rt, &black, c, a);
}

fn right_side((ax, ay): (i32, i32), (bx, by): (i32, i32), (px, py): (i32, i32)) -> bool {
    // Calculate AB and AP
    let abx = bx - ax;
    let aby = by - ay;

    let apx = px - ax;
    let apy = py - ay;

    // Calculate z of cross product
    let z = abx * apy - aby * apx;

    // Change to less-than for front-face culling
    z > 0
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
            screen_uv: to_normalized(rt.dimensions(), (x, y)),
        };

        let color = shader.fragment_color(&input);
        rt.set_pixel(x as usize, y as usize, &color);

        true
    } else {
        false
    }
}
