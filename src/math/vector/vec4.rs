use std::ops::Mul;

use crate::math::Mat4;

use super::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x)
            + (self.y * self.y)
            + (self.z * self.z)
            + (self.w * self.w))
            .sqrt()
    }

    /// Return normal vector but leave original (self) unchanged
    pub fn normal(&self) -> Vec4 {
        let magnitude = self.magnitude();

        Vec4 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    /// Mutates the vector and sets it to its normal
    pub fn normalize(&mut self) {
        *self = self.normal();
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Mul<Mat4> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let x = self.x * rhs.get(0, 0)
            + self.y * rhs.get(1, 0)
            + self.z * rhs.get(2, 0)
            + self.w * rhs.get(3, 0);
        let y = self.x * rhs.get(0, 1)
            + self.y * rhs.get(1, 1)
            + self.z * rhs.get(2, 1)
            + self.w * rhs.get(3, 1);
        let z = self.x * rhs.get(0, 2)
            + self.y * rhs.get(1, 2)
            + self.z * rhs.get(2, 2)
            + self.w * rhs.get(3, 2);
        let w = self.x * rhs.get(0, 3)
            + self.y * rhs.get(1, 3)
            + self.z * rhs.get(2, 3)
            + self.w * rhs.get(3, 3);

        Vec4 { x, y, z, w }
    }
}
