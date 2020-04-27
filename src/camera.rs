use super::math::*;

pub struct Camera {
    pub translation: Vec3,
    pub rotation: Vec3,
}

impl Camera {
    pub fn view_matrix(&self) -> Mat4x4 {
        self.rotation
            .mul(-1.0)
            .camera_rotation()
            .mat_mul(&self.translation.mul(-1.0).translation())
    }
}
