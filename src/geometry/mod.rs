use super::math::*;

pub type Triangle = [Vec3; 3];

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}
