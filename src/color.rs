use std::fmt;

pub struct Color {
    r: f32,
    g: f32,
    b: f32
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r * 255.999, self.g * 255.999, self.b * 255.999)
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b:f32) -> Self {
        Self {r, g, b}
}
