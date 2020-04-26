use crate::geometry::*;
use crate::math::*;
use std::io::BufRead;

use ::failure::format_err;
use ::failure::Error;

pub fn load(path: &std::path::Path) -> Result<Mesh, Error> {
    let mut positions: Vec<Vec3> = vec![];
    let mut triangles: Vec<Triangle> = vec![];

    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();

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
            Some("f") => triangles.push(parse_triangle(&mut words, &positions)?),
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
    let z = args
        .next()
        .ok_or(format_err!("no Z coordinate"))?
        .parse::<f32>()?;

    // W is always ignored
    let w = args.next().unwrap_or("1.0").parse::<f32>()?;

    Ok(Vec4::new(x, y, z, w))
}

fn parse_triangle(
    args: &mut std::str::SplitWhitespace,
    vertices: &Vec<Vec3>,
) -> Result<Triangle, Error> {
    Ok([
        parse_vertex(args, vertices)?,
        parse_vertex(args, vertices)?,
        parse_vertex(args, vertices)?,
    ])
}

fn parse_vertex(
    args: &mut std::str::SplitWhitespace,
    positions: &Vec<Vec3>,
) -> Result<Vertex, Error> {
    let mut args = args
        .next()
        .ok_or(format_err!("no vertex triple"))?
        .split('/');

    let v = args
        .next()
        .ok_or(format_err!("no vertex index"))?
        .parse::<usize>()?;
    let position = positions
        .get(v - 1)
        .ok_or(format_err!("vertex index {} out of bounds", v))?;

    /* TODO: add support for uv coords and normals */

    Ok(Vertex {
        position: *position,
        uv: Vec2::new(0.0, 0.0),
        color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    })
}
