use super::math::*;

pub struct Camera {
    pub translation: Vec3,
}

impl Camera {
    pub fn view_matrix(&self) -> Mat4x4 {
        self.translation.mul(-1.0).translation()
    }
}
