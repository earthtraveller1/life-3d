use glad_gl::gl::{self, GLenum, GLuint};

pub enum BufferType {
    Vertex,
    Index,
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
        }
    }

    pub fn with_data<T>(buffer_type: BufferType, data: &[T]) -> Buffer {
        let buffer = Buffer::new(buffer_type);

        unsafe {
            gl::BindBuffer(buffer.get_target(), buffer.buffer);
            gl::BufferData(
                buffer.get_target(),
                std::mem::size_of::<T>().try_into().unwrap(),
                std::mem::transmute(data.as_ptr()),
                gl::ARRAY_BUFFER,
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
