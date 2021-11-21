use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let i = (self.y * other.z) - (self.z * other.y);
        let j = (self.x * other.z) - (self.z * other.x);
        let k = (self.x * other.y) - (self.y * other.x);

        Vec3 { x: i, y: -j, z: k }
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    /// Return normal vector but leave original (self) unchanged
    pub fn normal(&self) -> Vec3 {
        let magnitude = self.magnitude();

        Vec3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    /// Mutates the vector and sets it to its normal
    pub fn normalize(&mut self) {
        *self = self.normal();
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, other: f32) -> Self::Output {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self::Output {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3;

    #[test]
    fn dot_valid_1() {
        let v1 = Vec3::new(1_f32, 2_f32, 3_f32);
        let v2 = Vec3::new(4_f32, 5_f32, 6_f32);

        assert_eq!(v1.dot(&v2), 32_f32);
    }

    #[test]
    fn dot_valid_2() {
        let v1 = Vec3::new(-1_f32, 2_f32, -3_f32);
        let v2 = Vec3::new(4_f32, 5_f32, 6_f32);

        assert_eq!(v1.dot(&v2), -12_f32);
    }

    #[test]
    fn cross_valid_1() {
        let v1 = Vec3::new(0_f32, 0_f32, 0_f32);
        let v2 = Vec3::new(0_f32, 0_f32, 0_f32);

        assert_eq!(
            v1.cross(&v2),
            Vec3 {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            }
        );
    }

    #[test]
    fn cross_valid_2() {
        let v1 = Vec3::new(1_f32, 2_f32, 3_f32);
        let v2 = Vec3::new(4_f32, 5_f32, 6_f32);

        assert_eq!(
            v1.cross(&v2),
            Vec3 {
                x: -3_f32,
                y: 6_f32,
                z: -3_f32
            }
        );
    }

    #[test]
    fn cross_valid_3() {
        let v1 = Vec3::new(-1_f32, -2_f32, -3_f32);
        let v2 = Vec3::new(-4_f32, -5_f32, -6_f32);

        assert_eq!(
            v1.cross(&v2),
            Vec3 {
                x: -3_f32,
                y: 6_f32,
                z: -3_f32
            }
        );
    }

    #[test]
    fn cross_valid_4() {
        let v1 = Vec3::new(1_f32, 2_f32, 3_f32);
        let v2 = Vec3::new(-4_f32, -5_f32, -6_f32);

        assert_eq!(
            v1.cross(&v2),
            Vec3 {
                x: 3_f32,
                y: -6_f32,
                z: 3_f32
            }
        );
    }

    #[test]
    fn cross_valid_5() {
        let v1 = Vec3::new(1_f32, 1_f32, 1_f32);
        let v2 = Vec3::new(-1_f32, -1_f32, -1_f32);

        assert_eq!(
            v1.cross(&v2),
            Vec3 {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32
            }
        );
    }
}
