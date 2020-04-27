use super::geometry::*;
use super::math::*;

#[derive(Copy, Clone)]
pub struct Object<'a> {
    pub mesh: &'a Mesh,

    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}

impl<'a> Object<'a> {
    pub fn new(mesh: &'a Mesh) -> Self {
        Object {
            mesh,
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn transform(&self) -> Mat4x4 {
        // We scale first, then rotate, and finally translate
        self.translation
            .translation()
            .mat_mul(&self.rotation.rotation())
            .mat_mul(&self.scale.scaling())
    }

    pub fn normal_transform(&self) -> Mat4x4 {
        // We scale first, then rotate, and finally translate
        self.rotation.rotation()
    }
}
