use crate::vec::Vec3;

#[derive(Debug, Default)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self {orig, dir}
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}
