use std::ops;
use std::fmt;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn len(&self) -> f32 {
        let len_squared = self.x * self.x + self.y * self.y + self.z * self.z;
        len_squared.sqrt() 
    } 

}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)

    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z 
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(u.y * v.z - u.z * v.y,
              u.z * v.x - u.x * v.z,
              u.x * v.y - u.y * v.x)
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.len()
}


mod tests {
    use super::*;

    #[test]
    fn addition() {
        let lhs = Vec3 {x: 1., y: 2., z: 3.}; 
        let rhs = Vec3 {x: 4., y: 5., z: 6.};

        let res = Vec3 {x: 5., y: 7., z: 9.};
        assert_eq!(lhs + rhs, res);
    }
    #[test]
    fn substraction() {
        let lhs = Vec3 {x: 1., y: 2., z: 3.}; 
        let rhs = Vec3 {x: 4., y: 5., z: 6.};

        let res = Vec3 {x: -3., y: -3., z: -3.};

        assert_eq!(lhs - rhs, res);
    }
    #[test]
    fn multiplication() {
        let lhs = Vec3 {x: 1., y: 2., z: 3.}; 
        let rhs = Vec3 {x: 4., y: 5., z: 6.};

        let res = Vec3 {x: 4., y: 10., z: 18.};

        assert_eq!(lhs * rhs, res);
    }
}
