use std::{
    ffi::{c_char, c_void, CStr},
    ptr::null,
};

use glad_gl::gl;
use glfw::Context;

use life_3d::{
    camera::ThirdPersonCamera,
    game::{Cell, Cursor, GameOfLife, ARENA_SIZE},
    math::{Mat4, Vec3},
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

    let (mut window, events) = glfw.with_primary_monitor(|glfw, monitor| {
        let monitor = monitor.unwrap();
        let video_mode = monitor.get_video_mode().unwrap();
        let width = (0.75 * video_mode.width as f32) as u32;
        let height = ((9.0 / 16.0) * width as f32) as u32;

        glfw.create_window(width, height, "Life 3D", glfw::WindowMode::Windowed)
            .expect("Failed to create the GLFW window.")
    });

    window.set_framebuffer_size_polling(true);
    window.set_scroll_polling(true);
    window.set_key_polling(true);

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
    const CELL_SIZE: f32 = 0.1;
    let cell = Mesh::cube(CELL_SIZE);
    let mut renderer = Renderer::new(&cell);

    let window_size = window.get_size();
    let (window_width, window_height) = window_size;
    let (window_width, window_height): (f32, f32) = (window_width as f32, window_height as f32);

    let mut projection =
        life_3d::math::Mat4::perspective(window_width / window_height, 0.1, 100.0, 45.0);

    let (mut previous_mouse_x, mut previous_mouse_y) = (0.0, 0.0);
    let mut has_set_mouse_x = false;

    let mut game = GameOfLife::new();

    game.set_cell(ARENA_SIZE/2, ARENA_SIZE/2, ARENA_SIZE/2, Cell::Alive);

    // let mut camera = Camera::new(&Vec3::new(0.0, 0.0, 3.0), &Vec3::new(0.0, 0.0, -1.0));

    let mut delta_time;
    let mut previous_time = 0.0;
    let mut zoom_speed = 0.0;
    let max_zoom_speed = 150.0;

    let max_tick_progress = 0.25;
    let mut tick_progress = 0.25;
    let mut tick_speed = 1.0;
    let mut paused = true;

    let mut cursor = Cursor::new();

    let mut camera = ThirdPersonCamera::new(Vec3::new(0.0, 0.0, 0.0), 5.0, 0.0, 0.0);

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

            let sensitivity = 5.0;
            camera.rotate_camera(
                sensitivity * (delta_time * delta_mouse_x) as f32,
                sensitivity * (delta_time * delta_mouse_y) as f32,
            );
        }

        if (zoom_speed as f32).abs() > 1.0 {
            camera.move_camera(-zoom_speed * (delta_time as f32));
        }

        if zoom_speed > 0.0 {
            zoom_speed -= 50.0 * (delta_time as f32);
        } else if zoom_speed < 0.0 {
            zoom_speed += 50.0 * (delta_time as f32);
        }

        zoom_speed = zoom_speed.clamp(-max_zoom_speed, max_zoom_speed);

        if tick_progress >= max_tick_progress {
            tick_progress = 0.0;
            game.update_game();
        } else {
            if !paused {
                tick_progress += tick_speed * delta_time;
            }
        }

        let view = camera.view_matrix();

        {
            let shader_program = shader_program.use_program();
            shader_program.set_uniform("cell_size", CELL_SIZE);
            shader_program.set_uniform("view", &view);
            shader_program.set_uniform("model", Mat4::new(1.0));
            shader_program.set_uniform("projection", &projection);
            game.render(&mut renderer, CELL_SIZE, &cursor);
        }

        cursor.render(&game, &renderer, CELL_SIZE, &projection, &view);

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                    let (width, height) = (width as f32, height as f32);

                    projection = life_3d::math::Mat4::perspective(width / height, 0.1, 100.0, 45.0);
                },
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    let factor = yoffset / yoffset.abs();

                    zoom_speed += (factor as f32) * max_zoom_speed * (delta_time as f32);
                    zoom_speed = zoom_speed.clamp(-max_zoom_speed, max_zoom_speed);
                }
                glfw::WindowEvent::Key(key, _, action, _modifiers) => match action {
                    glfw::Action::Press => match key {
                        glfw::Key::Space => {
                            paused = !paused;
                        }
                        glfw::Key::Enter => {
                            game.flip_at_cursor(&cursor);
                        }
                        glfw::Key::KpAdd => {
                            tick_speed += 1.0;
                            tick_speed = tick_speed.clamp(1.0, 5.0);
                        }
                        glfw::Key::KpSubtract => {
                            tick_speed -= 1.0;
                            tick_speed = tick_speed.clamp(1.0, 5.0);
                        }
                        glfw::Key::W => {
                            cursor.move_x(-1);
                        }
                        glfw::Key::S => {
                            cursor.move_x(1);
                        }
                        glfw::Key::A => {
                            cursor.move_z(1);
                        }
                        glfw::Key::D => {
                            cursor.move_z(-1);
                        }
                        glfw::Key::Q => {
                            cursor.move_y(1);
                        }
                        glfw::Key::E => {
                            cursor.move_y(-1);
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        previous_mouse_x = mouse_x;
        previous_mouse_y = mouse_y;
    }
}
