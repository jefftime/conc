use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    data: [f32; 16],
}

impl Mat4 {
    pub fn zeroed() -> Mat4 {
        Mat4 { data: [0_f32; 16] }
    }

    pub fn identity() -> Mat4 {
        let mut result = Mat4::zeroed();
        *result.get_mut(0, 0) = 1_f32;
        *result.get_mut(1, 1) = 1_f32;
        *result.get_mut(2, 2) = 1_f32;
        *result.get_mut(3, 3) = 1_f32;
        result
    }

    pub fn get<'a>(&'a self, row: usize, col: usize) -> &'a f32 {
        &self.data[(row * 4) + col]
    }

    pub fn get_mut<'a>(&'a mut self, row: usize, col: usize) -> &'a mut f32 {
        &mut self.data[(row * 4) + col]
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, other: Mat4) -> Self::Output {
        let mut result = Mat4::zeroed();
        for row in 0..4 {
            for col in 0..4 {
                *result.get_mut(row, col) =
                    self.get(row, col) + other.get(row, col);
            }
        }

        result
    }
}

impl Add<f32> for Mat4 {
    type Output = Self;

    fn add(self, other: f32) -> Self::Output {
        let mut result = Mat4::zeroed();
        for row in 0..4 {
            for col in 0..4 {
                *result.get_mut(row, col) = self.get(row, col) + other
            }
        }

        result
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, other: Mat4) -> Self::Output {
        let mut result = Mat4::zeroed();
        for row in 0..4 {
            for col in 0..4 {
                *result.get_mut(row, col) =
                    self.get(row, col) - other.get(row, col);
            }
        }

        result
    }
}

impl Sub<f32> for Mat4 {
    type Output = Self;

    fn sub(self, other: f32) -> Self::Output {
        let mut result = Mat4::zeroed();
        for row in 0..4 {
            for col in 0..4 {
                *result.get_mut(row, col) = self.get(row, col) - other
            }
        }

        result
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Mat4) -> Self::Output {
        let mut result = Mat4::zeroed();
        for row in 0..4 {
            for col in 0..4 {
                *result.get_mut(row, col) = (self.get(row, 0)
                    * other.get(0, col))
                    + (self.get(row, 1) * other.get(1, col))
                    + (self.get(row, 2) * other.get(2, col))
                    + (self.get(row, 3) * other.get(3, col))
            }
        }

        result
    }
}

mod tests {
    use crate::math::Mat4;

    #[test]
    pub fn mat4_zeroed() {
        let mat = Mat4::zeroed();

        assert_eq!(
            mat,
            Mat4 {
                data: [
                    0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32,
                    0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32
                ]
            }
        );
    }

    #[test]
    pub fn mat4_identity() {
        let mat = Mat4::identity();

        assert_eq!(
            mat,
            Mat4 {
                data: [
                    1_f32, 0_f32, 0_f32, 0_f32, 0_f32, 1_f32, 0_f32, 0_f32,
                    0_f32, 0_f32, 1_f32, 0_f32, 0_f32, 0_f32, 0_f32, 1_f32
                ]
            }
        );
    }

    #[test]
    pub fn mat4_add_valid() {
        let m1 = Mat4::identity();
        let m2 = Mat4::identity();

        assert_eq!(
            m1 + m2,
            Mat4 {
                data: [
                    2_f32, 0_f32, 0_f32, 0_f32, 0_f32, 2_f32, 0_f32, 0_f32,
                    0_f32, 0_f32, 2_f32, 0_f32, 0_f32, 0_f32, 0_f32, 2_f32
                ]
            }
        );
    }

    #[test]
    pub fn mat4_sub_valid() {
        let m1 = Mat4::identity();
        let m2 = Mat4::identity();

        assert_eq!(m1 - m2, Mat4::zeroed());
    }

    #[test]
    pub fn mat4_mul_valid_1() {
        let mut m1 = Mat4::zeroed();
        let mut m2 = Mat4::zeroed();

        let mut count = 0_f32;
        for row in 0..4 {
            for col in 0..4 {
                *m1.get_mut(row, col) = count;
                *m2.get_mut(row, col) = count;
                count += 1_f32;
            }
        }

        assert_eq!(
            m1 * m2,
            Mat4 {
                data: [
                    56_f32, 62_f32, 68_f32, 74_f32, 152_f32, 174_f32, 196_f32,
                    218_f32, 248_f32, 286_f32, 324_f32, 362_f32, 344_f32,
                    398_f32, 452_f32, 506_f32
                ]
            }
        )
    }

    #[test]
    pub fn mat4_mul_valid_2() {
        let mut m1 = Mat4::zeroed();
        let mut m2 = Mat4::zeroed();

        let mut count = 0_f32;
        for row in 0..4 {
            for col in 0..4 {
                *m1.get_mut(row, col) = count;
                *m2.get_mut(row, col) = -count;
                count += 1_f32;
            }
        }

        assert_eq!(
            m1 * m2,
            Mat4 {
                data: [
                    -56_f32, -62_f32, -68_f32, -74_f32, -152_f32, -174_f32,
                    -196_f32, -218_f32, -248_f32, -286_f32, -324_f32, -362_f32,
                    -344_f32, -398_f32, -452_f32, -506_f32
                ]
            }
        )
    }
}
