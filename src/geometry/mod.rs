use super::math::*;

pub type Triangle = [Vec3; 3];

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}
