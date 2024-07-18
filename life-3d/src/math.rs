use std::ops::{Add, Div, Mul};

use glad_gl::gl;

use crate::shaders::ShaderUniform;

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        self.clone() / self.len()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Source: https://en.wikipedia.org/wiki/Cross_product#Computing
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
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
}

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Quaternion(Vec4);

impl Quaternion {
    // Note: `angle` must be in radians.
    // Axis must also be a unit vector.
    // Source: http://www.faqs.org/faqs/graphics/algorithms-faq/
    pub fn new(axis: &Vec3, angle: f32) -> Quaternion {
        Quaternion(Vec4::new(
            axis.x * (angle / 2.0).sin(),
            axis.y * (angle / 2.0).sin(),
            axis.z * (angle / 2.0).sin(),
            (angle / 2.0).cos(),
        ))
    }

    // Source: http://www.faqs.org/faqs/graphics/algorithms-faq/
    pub fn to_rotation_matrix(&self) -> Mat4 {
        let x = self.0.x;
        let y = self.0.y;
        let z = self.0.z;
        let w = self.0.w;

        Mat4 {
            data: [
                [
                    1.0 - 2.0 * (y * y + z * z),
                    2.0 * (x * y + w * z),
                    2.0 * (x * z - w * y),
                    0.0,
                ],
                [
                    2.0 * (x * y - w * z),
                    1.0 - 2.0 * (x * x + z * z),
                    2.0 * (y * z + w * x),
                    0.0,
                ],
                [
                    2.0 * (x * z + w * y),
                    2.0 * (y * z - w * x),
                    1.0 - 2.0 * (x * x + y * y),
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // Source: https://songho.ca/math/quaternion/quaternion.html#algebra
    pub fn conjugate(&self) -> Quaternion {
        Quaternion(Vec4::new(-self.0.x, -self.0.y, -self.0.z, self.0.w))
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        let self_v = Vec3::new(self.0.x, self.0.y, self.0.z);
        let rhs_v = Vec3::new(rhs.0.x, rhs.0.y, rhs.0.z);
        let new_v = self_v.cross(&rhs_v) + rhs_v.clone() * self.0.w + self_v.clone() * rhs.0.w;

        Quaternion(Vec4::new(
            new_v.x,
            new_v.y,
            new_v.z,
            self.0.w * rhs.0.w - self_v.dot(&rhs_v),
        ))
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat4 {
    pub(crate) data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn new(x: f32) -> Mat4 {
        Mat4 {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, x, 0.0, 0.0],
                [0.0, 0.0, x, 0.0],
                [0.0, 0.0, 0.0, x],
            ],
        }
    }

    pub fn perspective(aspect_ratio: f32, z_near: f32, z_far: f32, fov: f32) -> Mat4 {
        let tan_half_fov = (fov.to_radians() / 2.0).tan();

        assert_ne!(aspect_ratio, 0.0);
        assert_ne!(z_far, z_near);

        Mat4 {
            data: [
                [1.0 / (aspect_ratio * tan_half_fov), 0.0, 0.0, 0.0],
                [0.0, 1.0 / tan_half_fov, 0.0, 0.0],
                [0.0, 0.0, -(z_near + z_far) / (z_far - z_near), -1.0],
                [0.0, 0.0, -(2.0 * z_far * z_near) / (z_far - z_near), 0.0],
            ],
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Mat4;

    #[test]
    fn matrix_multiplication() {
        let first = Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, -5.0, 1.0],
            ],
        };
        let second = Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let expected = Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, -5.0, 1.0],
            ],
        };

        let obtained = first * second;
        assert_eq!(expected, obtained);
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Mat4::new(1.0);

        for column in 0..4 {
            for row in 0..4 {
                result.data[column][row] = rhs.data[column][0] * self.data[0][row]
                    + rhs.data[column][1] * self.data[1][row]
                    + rhs.data[column][2] * self.data[2][row]
                    + rhs.data[column][3] * self.data[3][row];
            }
        }

        result
    }
}

unsafe impl ShaderUniform for Mat4 {
    unsafe fn set_uniform(&self, location: glad_gl::gl::GLint) {
        gl::UniformMatrix4fv(location, 1, gl::FALSE, self.data.as_ptr() as *const f32);
    }
}

unsafe impl ShaderUniform for &Mat4 {
    unsafe fn set_uniform(&self, location: glad_gl::gl::GLint) {
        (*self).set_uniform(location);
    }
}
