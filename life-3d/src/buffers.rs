use glad_gl::gl::{self, GLenum, GLuint};

pub enum BufferType {
    Vertex,
    Index,
    ShaderStorage,
}

pub struct Buffer {
    buffer: GLuint,
    buffer_type: BufferType,
}

impl Buffer {
    pub fn new(buffer_type: BufferType) -> Buffer {
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);

            Buffer {
                buffer,
                buffer_type,
            }
        }
    }

    pub fn get_target(&self) -> GLenum {
        match self.buffer_type {
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.get_target(), self.buffer) };
    }

    pub fn bind_base(&self, index: GLuint) {
        unsafe { gl::BindBufferBase(self.get_target(), index, self.buffer) };
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(self.get_target(), 0) };
    }

    pub fn with_data<T>(buffer_type: BufferType, data: &[T]) -> Buffer {
        let buffer = Buffer::new(buffer_type);

        unsafe {
            gl::BindBuffer(buffer.get_target(), buffer.buffer);
            gl::BufferData(
                buffer.get_target(),
                data.len() as isize * std::mem::size_of::<T>() as isize,
                std::mem::transmute(data.as_ptr()),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(buffer.get_target(), 0);
        }

        buffer
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer);
        }
    }
}

pub struct VertexArray {
    vertex_array: GLuint,
}

pub unsafe trait BufferAttributes {
    unsafe fn vertex_attributes();
}

impl VertexArray {
    pub fn new() -> VertexArray {
        unsafe {
            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);

            Self { vertex_array }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vertex_array) };
    }

    pub fn unbind() {
        unsafe { gl::BindVertexArray(0) };
    }

    pub fn bind_buffer_and_attributes<Attributes: BufferAttributes>(&self, buffer: &Buffer) {
        unsafe {
            self.bind();
            buffer.bind();
            Attributes::vertex_attributes();
            buffer.unbind();
            Self::unbind();
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array);
        }
    }
}
