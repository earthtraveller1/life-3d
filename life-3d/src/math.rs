use std::ops::Add;

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
    pub fn new(axis: &Vec3, angle: f32) {
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
                    2 * (x * z - w * y),
                    0,
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

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat4 {
    data: [[f32; 4]; 4],
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

unsafe impl ShaderUniform for Mat4 {
    unsafe fn set_uniform(&self, location: glad_gl::gl::GLint) {
        gl::UniformMatrix4fv(location, 1, gl::FALSE, self.data.as_ptr() as *const f32);
    }
}
