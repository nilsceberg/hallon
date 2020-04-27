use super::math::*;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub uv: Vec2,
    pub normal: Vec3,
    pub color: Vec4,
}

pub type Triangle = [Vertex; 3];

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn from_faces(faces: Vec<[Vec3; 3]>) -> Mesh {
        let mut triangles: Vec<Triangle> = vec![];

        for [a, b, c] in faces {
            triangles.push([
                Vertex {
                    position: a,
                    uv: Vec2::new(a.x, a.y),
                    color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    normal: Vec3::new(0.0, 1.0, 0.0),
                },
                Vertex {
                    position: b,
                    uv: Vec2::new(b.x, b.y),
                    color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    normal: Vec3::new(0.0, 1.0, 0.0),
                },
                Vertex {
                    position: c,
                    uv: Vec2::new(c.x, c.y),
                    color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    normal: Vec3::new(0.0, 1.0, 0.0),
                },
            ]);
        }

        Mesh { triangles }
    }
}
