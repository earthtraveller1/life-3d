use std::ffi::{CStr, CString};

use glad_gl::gl;

pub struct ShaderProgram(gl::GLuint);

// Creates an OpenGL shader of the specified type. `shader_type` must be of a
// valid shader type
unsafe fn create_shader(
    shader_source: &str,
    shader_type: gl::GLenum,
    shader_name: Option<&str>,
) -> gl::GLuint {
    let shader = gl::CreateShader(shader_type);

    let c_shader_source = CString::new(shader_source).expect("Shader source must be UTF-8");
    let c_shader_source = c_shader_source.as_ptr();
    let shader_source_len: gl::GLint = shader_source.len().try_into().unwrap();
    gl::ShaderSource(shader, 1, &c_shader_source, &shader_source_len);
    gl::CompileShader(shader);

    let mut status = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    let mut message_len = 0;
    gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut message_len);

    if status == 0 {
        let mut message = Vec::with_capacity(message_len as usize);
        gl::GetShaderInfoLog(
            shader,
            message_len,
            std::ptr::null_mut(),
            message.as_mut_ptr(),
        );
        message.set_len(message_len as usize);

        let message = CStr::from_ptr(message.as_ptr());
        eprintln!(
            "Failed to compile the shader {}:\n{}\n",
            shader_name.unwrap_or("<no-name>"),
            message.to_str().unwrap()
        );
    }

    shader
}

impl ShaderProgram {
    pub fn new(
        vertex_source: &str,
        vertex_name: Option<&str>,
        fragment_source: &str,
        fragment_name: Option<&str>,
    ) -> ShaderProgram {
        unsafe {
            let vertex = create_shader(vertex_source, gl::VERTEX_SHADER, vertex_name);
            let fragment = create_shader(fragment_source, gl::FRAGMENT_SHADER, fragment_name);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);

            let mut status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            if status == 0 {
                let mut message_len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut message_len);
                let mut message = Vec::with_capacity(message_len as usize);
                gl::GetProgramInfoLog(
                    program,
                    message_len,
                    std::ptr::null_mut(),
                    message.as_mut_ptr(),
                );

                let message = CStr::from_ptr(message.as_ptr());
                eprintln!(
                    "Failed to link {} and {} into a program:\n{}\n",
                    vertex_name.unwrap_or("<unamed vertex>"),
                    fragment_name.unwrap_or("<unamed fragment>"),
                    message.to_str().unwrap()
                );
            }

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            ShaderProgram(program)
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}
