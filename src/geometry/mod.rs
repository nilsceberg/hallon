use super::math::*;

pub type Triangle = [Vec4; 3];

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}
