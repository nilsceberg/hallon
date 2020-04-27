use super::geometry::*;
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
    depth: &mut RenderTarget,
    shader: &dyn FragmentShader,
    a: &Vertex,
    b: &Vertex,
    c: &Vertex,
    wireframe: bool,
) {
    if wireframe {
        line(rt, Some(depth), shader, a, b);
        line(rt, Some(depth), shader, b, c);
        line(rt, Some(depth), shader, c, a);
    } else {
        triangle_parallel(rt, depth, shader, a, b, c);
    }
}

// A method that works great in parallell (read: on hardware)
// but perhaps isn't great for CPUs. We check every pixel
// (as an optimization, only those in a rectangle that just covers the triangle),
// and check whether they are inside the triangle using cross products.

fn triangle_parallel(
    rt: &mut RenderTarget,
    depth: &mut RenderTarget,
    shader: &dyn FragmentShader,
    a: &Vertex,
    b: &Vertex,
    c: &Vertex,
) {
    let (x0, y0) = from_normalized(rt.dimensions(), &a.position.xy());
    let (x1, y1) = from_normalized(rt.dimensions(), &b.position.xy());
    let (x2, y2) = from_normalized(rt.dimensions(), &c.position.xy());

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
                pixel(
                    rt,
                    Some(depth),
                    shader,
                    x,
                    y,
                    &barycentric_interpolation(
                        [*a, *b, *c],
                        to_normalized(rt.dimensions(), (x, y)),
                    ),
                );
            }
        }
    }

    //let black = super::shaders::SolidShader(Vec4::new(0.0, 0.0, 0.0, 1.0));
    line(rt, Some(depth), shader, a, b);
    line(rt, Some(depth), shader, b, c);
    line(rt, Some(depth), shader, c, a);
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

pub fn barycentric_interpolation([a, b, c]: Triangle, position: Vec2) -> Vertex {
    // https://en.wikipedia.org/wiki/Barycentric_coordinate_system
    let px = position;
    let pa = a.position;
    let pb = b.position;
    let pc = c.position;

    let d = pb.x - pa.x;

    let wa = ((pb.y - pc.y) * (px.x - pc.x) + (pc.x - pb.x) * (px.y - pc.y))
        / ((pb.y - pc.y) * (pa.x - pc.x) + (pc.x - pb.x) * (pa.y - pc.y));
    let wb = ((pc.y - pa.y) * (px.x - pc.x) + (pa.x - pc.x) * (px.y - pc.y))
        / ((pb.y - pc.y) * (pa.x - pc.x) + (pc.x - pb.x) * (pa.y - pc.y));
    let wc = 1.0 - wa - wb;

    Vertex {
        position: a
            .position
            .mul(wa)
            .add(&b.position.mul(wb))
            .add(&c.position.mul(wc)),
        color: a.color.mul(wa).add(&b.color.mul(wb)).add(&c.color.mul(wc)),
        uv: a.uv.mul(wa).add(&b.uv.mul(wb)).add(&c.uv.mul(wc)),
        normal: a
            .normal
            .mul(wa)
            .add(&b.normal.mul(wb))
            .add(&c.normal.mul(wc)),
    }
}

pub fn nearest_interpolation([a, b, c]: Triangle, position: Vec2) -> Vertex {
    let pa = a.position.xy();
    let pb = b.position.xy();
    let pc = c.position.xy();

    let da = pa.sub(&position).length2();
    let db = pb.sub(&position).length2();
    let dc = pc.sub(&position).length2();

    if da <= db && da <= dc {
        return a;
    } else if db <= da && db <= dc {
        return b;
    } else {
        return c;
    }
}

// I guess barycentric is also linear, but this one is for lines. :)
pub fn linear_interpolation((a, b): (Vertex, Vertex), position: Vec2) -> Vertex {
    let pa = a.position.xy();
    let pb = b.position.xy();
    let d = pa.sub(&pb).length();
    let f = pa.sub(&position).length();
    let wa = 1.0 - f / d;
    let wb = f / d;

    Vertex {
        position: a.position.mul(wa).add(&b.position.mul(wb)),
        color: a.color.mul(wa).add(&b.color.mul(wb)),
        uv: a.uv.mul(wa).add(&b.uv.mul(wb)),
        normal: a.normal.mul(wa).add(&b.normal.mul(wb)),
    }
}

pub fn line_2d(rt: &mut RenderTarget, shader: &dyn FragmentShader, a: &Vec2, b: &Vec2) {
    line(
        rt,
        None,
        shader,
        &Vertex {
            position: Vec3::new(a.x, a.y, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            uv: *a,
            normal: Vec3::new(0.0, 0.0, -1.0),
        },
        &Vertex {
            position: Vec3::new(b.x, b.y, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            uv: *a,
            normal: Vec3::new(0.0, 0.0, -1.0),
        },
    );
}

pub fn line(
    rt: &mut RenderTarget,
    depth: Option<&mut RenderTarget>,
    shader: &dyn FragmentShader,
    a: &Vertex,
    b: &Vertex,
) {
    if depth.is_none() {
        return;
    }

    let depth = depth.unwrap();

    /* Naive: */
    let (x1, y1) = from_normalized(rt.dimensions(), &a.position.xy());
    let (x2, y2) = from_normalized(rt.dimensions(), &b.position.xy());

    pixel(rt, Some(depth), shader, x1, y1, a);

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
            pixel(
                rt,
                Some(depth),
                shader,
                x + x1,
                y + y1,
                &linear_interpolation((*a, *b), to_normalized(rt.dimensions(), (x + x1, y + y1))),
            );
        }
    } else {
        let slope = (dx as f32) / (dy as f32);
        let mut y = 0;
        while y != dy {
            y += dy.signum();
            let x = ((y as f32) * slope).round() as i32;
            pixel(
                rt,
                Some(depth),
                shader,
                x + x1,
                y + y1,
                &linear_interpolation((*a, *b), to_normalized(rt.dimensions(), (x + x1, y + y1))),
            );
        }
    }
}

fn pixel(
    rt: &mut RenderTarget,
    depth: Option<&mut RenderTarget>,
    shader: &dyn FragmentShader,
    x: i32,
    y: i32,
    interpolated_vertex: &Vertex,
) -> bool {
    if x >= 0 && x < (rt.width as i32) && y >= 0 && y < (rt.height as i32) {
        let input = FragmentInput {
            vertex: *interpolated_vertex,
            screen_uv: to_normalized(rt.dimensions(), (x, y)),
        };

        let draw = depth
            .map(|t| {
                let new_depth = interpolated_vertex.position.z;
                let old_depth = t.get_pixel(x as usize, y as usize).x;

                if new_depth < old_depth {
                    t.set_pixel(
                        x as usize,
                        y as usize,
                        &Vec4::new(new_depth, new_depth, new_depth, 1.0),
                    );
                    true
                } else {
                    false
                }
            })
            .unwrap_or(true);

        if draw {
            let color = shader.fragment_color(&input);
            rt.set_pixel(x as usize, y as usize, &color);
        }

        draw
    } else {
        false
    }
}
