use crate::vec::Vec3;

#[derive(Debug, Default)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}
