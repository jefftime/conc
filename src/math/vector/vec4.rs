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
