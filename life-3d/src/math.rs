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
        let tan_half_fov = (fov / 2.0).to_radians().tan();
        let z_range = z_near - z_far;

        Mat4 {
            data: [
                [1.0 / (aspect_ratio * tan_half_fov), 0.0, 0.0, 0.0],
                [0.0, 1.0 / tan_half_fov, 0.0, 0.0],
                [
                    0.0,
                    0.0,
                    (-z_near - z_far) / z_range,
                    2.0 * z_far * z_near / z_range,
                ],
                [0.0, 0.0, 1.0, 0.0],
            ],
        }
    }
}

unsafe impl ShaderUniform for Mat4 {
    unsafe fn set_uniform(&self, location: glad_gl::gl::GLint) {
        gl::UniformMatrix4fv(location, 1, gl::FALSE, self.data.as_ptr() as *const f32);
    }
}
