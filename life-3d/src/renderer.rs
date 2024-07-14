use glad_gl::gl::{self, GLsizei};

use crate::{
    buffers::{Buffer, BufferType, BufferAttributes, VertexArray},
    math::{Vec2, Vec3},
    shaders::ShaderProgram,
};

use std::{
    mem::{offset_of, size_of},
    os::raw::c_void,
};

#[repr(C)]
struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

unsafe impl BufferAttributes for Vertex {
    unsafe fn vertex_attributes() {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>() as i32,
            offset_of!(Vertex, position) as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>() as i32,
            offset_of!(Vertex, normal) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>() as i32,
            offset_of!(Vertex, uv) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);
        
        
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

pub enum Axis {
    X,
    Y,
    Z,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn cube(size: f32) -> Mesh {
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        };

        mesh.append_cube_face(size, Axis::X, true);
        mesh.append_cube_face(size, Axis::X, false);
        mesh.append_cube_face(size, Axis::Y, true);
        mesh.append_cube_face(size, Axis::Y, false);
        mesh.append_cube_face(size, Axis::Z, true);
        mesh.append_cube_face(size, Axis::Z, false);

        mesh
    }

    pub fn append_cube_face(&mut self, size: f32, axis: Axis, positive: bool) {
        let size = size * 0.5;
        let values = [
            Vec2::new(size, size),
            Vec2::new(size, -size),
            Vec2::new(-size, -size),
            Vec2::new(-size, size),
        ];

        let depth_value = if positive { size } else { -size };
        // Save it here, as we will be appending stuff to the vertices vector
        // later on.
        let vertex_offset: u32 = self.vertices.len().try_into().unwrap();

        match axis {
            Axis::X => {
                for value in values.as_ref() {
                    self.vertices.push(Vertex {
                        position: Vec3::new(depth_value, value.y, value.x),
                        normal: Vec3::new(1.0, 0.0, 0.0),
                        uv: Vec2::new(0.0, 0.0), // TODO: Add the shader coordinates later.
                    })
                }
            }
            Axis::Y => {
                for value in values.as_ref() {
                    self.vertices.push(Vertex {
                        position: Vec3::new(value.x, depth_value, value.y),
                        normal: Vec3::new(0.0, 1.0, 0.0),
                        uv: Vec2::new(0.0, 0.0), // TODO: Add the shader coordinates later.
                    })
                }
            }
            Axis::Z => {
                for value in values.as_ref() {
                    self.vertices.push(Vertex {
                        position: Vec3::new(value.x, value.y, depth_value),
                        normal: Vec3::new(0.0, 0.0, 1.0),
                        uv: Vec2::new(0.0, 0.0), // TODO: Add the shader coordinates later.
                    })
                }
            }
        }

        let indices = [0, 1, 2, 0, 3, 2];

        for index in indices {
            self.indices.push(index + vertex_offset);
        }
    }
}

pub struct Renderer {
    vertex_buffer: Buffer,
    element_buffer: Buffer,
    vertex_array: VertexArray,

    indices_count: GLsizei,
}

impl Renderer {
    pub fn new(target_mesh: &Mesh) -> Renderer {
        let vertex_buffer = Buffer::with_data(BufferType::Vertex, target_mesh.vertices.as_slice());        
        let element_buffer = Buffer::with_data(BufferType::Index, target_mesh.indices.as_slice());

        let vertex_array = VertexArray::new();
        vertex_array.bind_buffer_and_attributes::<Vertex>(&vertex_buffer);

        Renderer {
            vertex_buffer,
            element_buffer,
            vertex_array,
            indices_count: target_mesh.indices.len() as i32,
        }
    }

    pub fn render(&self) {
        self.vertex_array.bind();
        self.element_buffer.bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, self.indices_count, gl::UNSIGNED_INT, std::ptr::null()) };
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::*;
    
    #[test]
    fn cube_face_tests() {
        let mut mesh = Mesh::new();
        mesh.append_cube_face(1.0, Axis::Z, true);

        let expected_positions = [
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(-0.5, 0.5, 0.5),
        ];

        for (vertex, expected_position) in mesh.vertices.iter().zip(expected_positions.iter()) {
            assert_eq!(vertex.normal, Vec3::new(0.0, 0.0, 1.0));
            assert_eq!(vertex.position, expected_position.clone());
        }
    }

    #[test]
    fn side_cube_face_test() {
        let mut mesh = Mesh::new();
        mesh.append_cube_face(1.0, Axis::X, false);

        let expected_positions = [
            Vec3::new(-0.5, 0.5, 0.5),
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(-0.5, 0.5, -0.5),
        ];

        for (vertex, expected_position) in mesh.vertices.iter().zip(expected_positions.iter()) {
            assert_eq!(vertex.normal, Vec3::new(1.0, 0.0, 0.0));
            assert_eq!(vertex.position, expected_position.clone());
        }
    }
}
