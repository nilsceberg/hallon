use crate::geometry::*;
use crate::math::*;
use std::collections::HashMap;
use std::io::BufRead;

use ::failure::format_err;
use ::failure::Error;

pub fn load(path: &std::path::Path) -> Result<HashMap<String, Vec4>, Error> {
    let mut colors = HashMap::new();

    static NO_COLOR: Vec4 = Vec4 {
        x: 0.5,
        y: 0.5,
        z: 0.5,
        w: 1.0,
    };

    let mut current_material = "default".to_owned();
    colors.insert(current_material.to_owned(), NO_COLOR);

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
            Some("newmtl") => {
                current_material = words
                    .next()
                    .ok_or(format_err!("no material name"))?
                    .to_owned();
            }
            Some("Kd") => {
                colors.insert(current_material.to_owned(), parse_vec4(&mut words)?);
            }
            //Some(d) => eprintln!("Unrecognized directive: {}", d),
            _ => {
                // Do nothing
            }
        }
    }

    Ok(colors)
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
