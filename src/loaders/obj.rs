use crate::geometry::*;
use crate::math::*;
use std::collections::HashMap;
use std::io::BufRead;

use ::failure::format_err;
use ::failure::Error;

pub fn load(
    path: &std::path::Path,
    material_colors: &HashMap<String, Vec4>,
) -> Result<Mesh, Error> {
    let mut positions: Vec<Vec3> = vec![];
    let mut uv: Vec<Vec2> = vec![];
    let mut normals: Vec<Vec3> = vec![];

    let mut triangles: Vec<Triangle> = vec![];

    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();

    static NO_COLOR: Vec4 = Vec4 {
        x: 1.0,
        y: 0.0,
        z: 1.0,
        w: 1.0,
    };
    let mut current_color = material_colors.get("default").unwrap_or(&NO_COLOR);

    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.chars().nth(0) == Some('#') {
            // Comment; ignore
            continue;
        }

        let mut words = line.split_whitespace();
        match words.next() {
            Some("v") => positions.push(parse_vec4(&mut words)?.xyz()),
            Some("vt") => uv.push(parse_vec4(&mut words)?.xyz().xy()),
            Some("vn") => normals.push(parse_vec4(&mut words)?.xyz()),
            Some("f") => triangles.push(parse_triangle(
                &mut words,
                &positions,
                &normals,
                &uv,
                current_color,
            )?),
            Some("usemtl") => {
                current_color = words
                    .next()
                    .and_then(|mtl| material_colors.get(mtl))
                    .unwrap_or(&NO_COLOR)
            }
            //Some(d) => eprintln!("Unrecognized directive: {}", d),
            _ => {
                // Do nothing
            }
        }
    }

    Ok(Mesh { triangles })
}

fn parse_vec4(args: &mut std::str::SplitWhitespace) -> Result<Vec4, Error> {
    let x = args
        .next()
        .ok_or(format_err!("no X coordinate"))?
        .parse::<f32>()?;
    let y = args
        .next()
        .ok_or(format_err!("no Y coordinate"))?
        .parse::<f32>()?;

    let z = args.next().unwrap_or("0.0").parse::<f32>()?;

    let w = args.next().unwrap_or("1.0").parse::<f32>()?;

    Ok(Vec4::new(x, y, z, w))
}

fn parse_triangle(
    args: &mut std::str::SplitWhitespace,
    vertices: &Vec<Vec3>,
    normals: &Vec<Vec3>,
    uv: &Vec<Vec2>,
    color: &Vec4,
) -> Result<Triangle, Error> {
    Ok([
        parse_vertex(args, vertices, normals, uv, color)?,
        parse_vertex(args, vertices, normals, uv, color)?,
        parse_vertex(args, vertices, normals, uv, color)?,
    ])
}

fn parse_vertex(
    args: &mut std::str::SplitWhitespace,
    positions: &Vec<Vec3>,
    normals: &Vec<Vec3>,
    uvs: &Vec<Vec2>,
    color: &Vec4,
) -> Result<Vertex, Error> {
    static DEFAULT_UV: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    static DEFAULT_NORMAL: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    let mut args = args
        .next()
        .ok_or(format_err!("no vertex triple"))?
        .split('/');

    let v = args
        .next()
        .ok_or(format_err!("no vertex index"))?
        .parse::<usize>()?;

    let uv = args
        .next()
        .and_then(|x| x.parse::<usize>().ok())
        .and_then(|i| uvs.get(i - 1))
        .unwrap_or(&DEFAULT_UV);

    let normal = args
        .next()
        .and_then(|x| x.parse::<usize>().ok())
        .and_then(|i| normals.get(i - 1))
        .unwrap_or(&DEFAULT_NORMAL);

    let position = positions
        .get(v - 1)
        .ok_or(format_err!("vertex index {} out of bounds", v))?;

    /* TODO: add support for uv coords and normals */

    Ok(Vertex {
        position: *position,
        uv: *uv,
        color: *color,
        normal: *normal,
    })
}
