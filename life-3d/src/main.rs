use std::{
    ffi::{c_char, c_void, CStr},
    ptr::null,
};

use glad_gl::gl;
use glfw::Context;

use life_3d::{
    camera::Camera,
    game::{Cell, GameOfLife},
    math::{Quaternion, Vec3},
    renderer::{Mesh, Renderer},
    shader_program_from_resources, shaders,
};

extern "system" fn opengl_debug_callback(
    _source: gl::GLenum,
    _error_type: gl::GLenum,
    _id: std::ffi::c_uint,
    _severity: gl::GLenum,
    _length: gl::GLsizei,
    message: *const c_char,
    _user_param: *mut c_void,
) {
    let message = unsafe { CStr::from_ptr(message) };
    eprintln!(
        "[OPENGL ERROR]: {}",
        message.to_str().unwrap_or("<INVALID UTF-8>")
    );
}

fn main() {
    let mut debug_opengl = false;

    for args in std::env::args() {
        if args == "--debug-opengl" {
            debug_opengl = true;
        }
    }

    let mut glfw = glfw::init(|error, message| eprintln!("[GLFW ERROR {:?}]: {}", error, message))
        .expect("Failed to initialize GLFW");

    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));

    if debug_opengl {
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
    }

    let (mut window, events) = glfw
        .create_window(1280, 720, "Life 3D", glfw::WindowMode::Windowed)
        .expect("Failed to create the GLFW window.");
    window.set_framebuffer_size_polling(true);

    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    gl::load(|sym| glfw.get_proc_address_raw(sym));

    if debug_opengl {
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(opengl_debug_callback, std::ptr::null());
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0,
                null(),
                gl::TRUE,
            );
        }
    }

    let shader_program = shader_program_from_resources!(shaders::MAIN_VERT, shaders::MAIN_FRAG);
    const CELL_SIZE: f32 = 0.2;
    let cell = Mesh::cube(CELL_SIZE);
    let renderer = Renderer::new(&cell);

    let window_size = window.get_size();
    let (window_width, window_height) = window_size;
    let (window_width, window_height): (f32, f32) = (window_width as f32, window_height as f32);

    let mut projection =
        life_3d::math::Mat4::perspective(window_width / window_height, 0.1, 100.0, 45.0);

    let mut rotation = Quaternion::new(&Vec3::new(1.0, 0.0, 0.0), 0.0);

    let (mut previous_mouse_x, mut previous_mouse_y) = (0.0, 0.0);
    let mut has_set_mouse_x = false;

    let mut game = GameOfLife::new();
    game.set_cell(0, 0, 0, Cell::Alive);
    game.set_cell(1, 1, 1, Cell::Alive);
    game.set_cell(2, 2, 2, Cell::Alive);
    game.set_cell(3, 3, 3, Cell::Alive);

    let mut camera = Camera::new(&Vec3::new(0.0, 0.0, 3.0), &Vec3::new(0.0, 0.0, -1.0));

    let mut delta_time;
    let mut previous_time = 0.0;

    while !window.should_close() {
        let current_time = glfw.get_time();
        delta_time = current_time - previous_time;
        previous_time = current_time;

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (mouse_x, mouse_y) = window.get_cursor_pos();
        if !has_set_mouse_x {
            previous_mouse_x = mouse_x;
            previous_mouse_y = mouse_y;
            has_set_mouse_x = true;
        }

        if let glfw::Action::Press = window.get_mouse_button(glfw::MouseButtonMiddle) {
            let (delta_mouse_x, delta_mouse_y) =
                (mouse_x - previous_mouse_x, mouse_y - previous_mouse_y);

            let horizontal_axis = Vec3::new(0.0, 1.0, 0.0).normalize();
            let vertical_axis = Vec3::new(1.0, 0.0, 0.0).normalize();
            let horizontal_rotation =
                Quaternion::new(&horizontal_axis, delta_mouse_x.to_radians() as f32 / 10.0);
            let vertical_rotation =
                Quaternion::new(&vertical_axis, delta_mouse_y.to_radians() as f32 / 10.0);

            rotation = horizontal_rotation * vertical_rotation * rotation;
        }

        if let glfw::Action::Press = window.get_key(glfw::Key::W) {
            camera.move_relative(Vec3::new(0.0, 0.0, delta_time as f32 * 3.0));
        }
        if let glfw::Action::Press = window.get_key(glfw::Key::S) {
            camera.move_relative(Vec3::new(0.0, 0.0, -delta_time as f32 * 3.0));
        }

        // let model = Mat4::translate(0.0, 0.0, -5.0) * rotation.to_rotation_matrix();
        // let view = Mat4::translate(0.0, 0.0, -3.0);
        let view = camera.view_matrix();

        let shader_program = shader_program.use_program();
        shader_program.set_uniform("cell_size", CELL_SIZE);
        shader_program.set_uniform("view", &view);
        shader_program.set_uniform("projection", &projection);
        game.render(&renderer, &shader_program, CELL_SIZE);

        /*shader_program.set_uniform("model", &model);
        renderer.render();*/

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                    let (width, height) = (width as f32, height as f32);

                    projection = life_3d::math::Mat4::perspective(width / height, 0.1, 100.0, 45.0);

                    eprintln!("projection = {:?}", projection);
                },
                _ => {}
            }
        }

        previous_mouse_x = mouse_x;
        previous_mouse_y = mouse_y;
    }
}
