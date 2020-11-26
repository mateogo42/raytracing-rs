use std::fmt;
use crate::vec::Vec3;
pub type Color = Vec3;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", (self.x * 255.999) as i32,
                              (self.y * 255.999) as i32, 
                              (self.z * 255.999) as i32)
    }
}
