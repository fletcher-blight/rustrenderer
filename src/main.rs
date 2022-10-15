extern crate gl;
mod camera;
mod shader;

use camera::Direction;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    sdl.mouse().set_relative_mouse_mode(true);
    let window = video_subsystem
        .window("Rust Renderer", 1200, 900)
        .opengl()
        .resizable()
        .build()
        .map_err(error_to_string())?;

    let _gl_context = window.gl_create_context()?;
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    #[rustfmt::skip]
    let vertices_cube: [f32; 216] = [
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
        0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
        0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
        0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
        0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

        0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
        0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
        0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
        0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
        0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
        0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
        0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
        0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
        0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
        0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
    ];

    let shader_cube =
        shader::compile_from_sources(include_str!("cube.vert"), include_str!("cube.frag"))?;
    let shader_light =
        shader::compile_from_sources(include_str!("light.vert"), include_str!("light.frag"))?;

    let mut vao_cube: GLuint = 0;
    let mut vao_light: GLuint = 0;
    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao_cube);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_cube.len() * std::mem::size_of::<f32>())
                .try_into()
                .map_err(error_to_string())?,
            vertices_cube.as_ptr() as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindVertexArray(vao_cube);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as i32,
            (3 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::GenVertexArrays(1, &mut vao_light);
        gl::BindVertexArray(vao_light);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }
    let vao_cube = vao_cube;
    let vao_light = vao_light;
    let vbo = vbo;

    let projection = nalgebra_glm::perspective(
        window.size().0 as f32 / window.size().1 as f32,
        num::Float::to_radians(45.0),
        0.1,
        100.0,
    );

    let mut camera = camera::start_from_world_pos(nalgebra_glm::vec3(0.0, 0.0, 3.0));
    let camera_speed: f32 = 10.0;
    let camera_sensitivity: f32 = 0.2;

    let mut current_movement: [Option<Direction>; 6] = [None, None, None, None, None, None];
    let mut frames: u64 = 0;
    let mut last_second: u32 = 0;

    let mut event_pump = sdl.event_pump()?;
    let timer = sdl.timer()?;
    let mut last_ticks = timer.performance_counter() as f64;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::A) => current_movement[0] = Some(Direction::Left),
                    Some(Keycode::D) => current_movement[1] = Some(Direction::Right),
                    Some(Keycode::W) => current_movement[2] = Some(Direction::Forward),
                    Some(Keycode::S) => current_movement[3] = Some(Direction::Backward),
                    Some(Keycode::Space) => current_movement[4] = Some(Direction::Up),
                    Some(Keycode::LAlt) => current_movement[5] = Some(Direction::Down),
                    _ => (),
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::A) => current_movement[0] = None,
                    Some(Keycode::D) => current_movement[1] = None,
                    Some(Keycode::W) => current_movement[2] = None,
                    Some(Keycode::S) => current_movement[3] = None,
                    Some(Keycode::Space) => current_movement[4] = None,
                    Some(Keycode::LAlt) => current_movement[5] = None,
                    _ => (),
                },
                _ => (),
            }
        }

        let now_ticks = timer.performance_counter() as f64;
        let delta_ticks = now_ticks - last_ticks;
        let freq_ticks = timer.performance_frequency() as f64;
        last_ticks = now_ticks;

        let delta_seconds = delta_ticks / freq_ticks;
        let camera_velocity = (delta_seconds * camera_speed as f64) as f32;
        let seconds = timer.ticks() as f32 / 1000.0;
        frames += 1;
        if seconds as u32 > last_second {
            last_second = seconds as u32;
            println!("FPS: {}", frames as f32 / seconds);
        }

        let mouse_state = sdl2::mouse::RelativeMouseState::new(&event_pump);
        camera.update_orientation(
            mouse_state.x() as f32,
            -mouse_state.y() as f32,
            camera_sensitivity,
        );

        for movement in &current_movement {
            match movement {
                Some(dir) => camera.update_position(*dir, camera_velocity),
                None => (),
            }
        }

        let mag = 2.5 + (seconds / 4.0).sin();
        let light_pos = nalgebra_glm::rotate_vec3(
            &nalgebra_glm::vec3(mag, 2.0 * (mag - 2.5), mag),
            seconds,
            &nalgebra_glm::vec3(0.0, 1.0, 0.0),
        );

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader_cube.enable();

            let model =
                nalgebra_glm::rotate(&num::one(), seconds, &nalgebra_glm::vec3(1.0, 1.0, 1.0));

            shader_cube.set_mat4("Model", &model)?;
            shader_cube.set_mat4("View", &camera.get_view_matrix())?;
            shader_cube.set_mat4("Projection", &projection)?;
            shader_cube.set_vec3("ObjectColour", &nalgebra_glm::vec3(1.0, 0.5, 0.31))?;
            shader_cube.set_vec3("LightColour", &nalgebra_glm::vec3(1.0, 1.0, 1.0))?;

            let light_scale = 0.3 * (seconds / 3.0).sin() + 0.4;

            let model = nalgebra_glm::scale(
                &nalgebra_glm::translate(&num::one(), &light_pos),
                &nalgebra_glm::vec3(light_scale, light_scale, light_scale),
            );

            shader_cube.set_float("Intensity", light_scale)?;
            shader_cube.set_mat4("LightModel", &model)?;
            shader_cube.set_vec3("ViewPos", &camera.get_position())?;

            gl::BindVertexArray(vao_cube);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // =====================

            shader_light.enable();
            shader_light.set_mat4("Model", &model)?;
            shader_light.set_mat4("View", &camera.get_view_matrix())?;
            shader_light.set_mat4("Projection", &projection)?;

            gl::BindVertexArray(vao_light);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            assert_eq!(gl::GetError(), 0);
        }

        window.gl_swap_window();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao_cube as *const GLuint);
        gl::DeleteVertexArrays(1, &vao_light as *const GLuint);
        gl::DeleteBuffers(1, &vbo as *const GLuint);
    }

    Ok(())
}

fn error_to_string<E>() -> fn(E) -> String
where
    E: std::fmt::Display,
{
    |e: E| e.to_string()
}
