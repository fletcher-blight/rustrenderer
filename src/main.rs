extern crate gl;
mod camera;
mod shader;
mod texture;

use crate::camera::Camera;
use camera::Direction;
use gl::types::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use sdl2::{EventPump, TimerSubsystem};

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
    let vertices_cube: [f32; 288] = [
        // positions            // normals              // texture coords
        -0.5, -0.5, -0.5,       0.0,  0.0, -1.0,        0.0,  0.0,
        0.5, -0.5, -0.5,        0.0,  0.0, -1.0,        1.0,  0.0,
        0.5,  0.5, -0.5,        0.0,  0.0, -1.0,        1.0,  1.0,
        0.5,  0.5, -0.5,        0.0,  0.0, -1.0,        1.0,  1.0,
        -0.5,  0.5, -0.5,       0.0,  0.0, -1.0,        0.0,  1.0,
        -0.5, -0.5, -0.5,       0.0,  0.0, -1.0,        0.0,  0.0,

        -0.5, -0.5,  0.5,       0.0,  0.0,  1.0,        0.0,  0.0,
        0.5, -0.5,  0.5,        0.0,  0.0,  1.0,        1.0,  0.0,
        0.5,  0.5,  0.5,        0.0,  0.0,  1.0,        1.0,  1.0,
        0.5,  0.5,  0.5,        0.0,  0.0,  1.0,        1.0,  1.0,
        -0.5,  0.5,  0.5,       0.0,  0.0,  1.0,        0.0,  1.0,
        -0.5, -0.5,  0.5,       0.0,  0.0,  1.0,        0.0,  0.0,

        -0.5,  0.5,  0.5,       -1.0,  0.0,  0.0,       1.0,  0.0,
        -0.5,  0.5, -0.5,       -1.0,  0.0,  0.0,       1.0,  1.0,
        -0.5, -0.5, -0.5,       -1.0,  0.0,  0.0,       0.0,  1.0,
        -0.5, -0.5, -0.5,       -1.0,  0.0,  0.0,       0.0,  1.0,
        -0.5, -0.5,  0.5,       -1.0,  0.0,  0.0,       0.0,  0.0,
        -0.5,  0.5,  0.5,       -1.0,  0.0,  0.0,       1.0,  0.0,

        0.5,  0.5,  0.5,        1.0,  0.0,  0.0,        1.0,  0.0,
        0.5,  0.5, -0.5,        1.0,  0.0,  0.0,        1.0,  1.0,
        0.5, -0.5, -0.5,        1.0,  0.0,  0.0,        0.0,  1.0,
        0.5, -0.5, -0.5,        1.0,  0.0,  0.0,        0.0,  1.0,
        0.5, -0.5,  0.5,        1.0,  0.0,  0.0,        0.0,  0.0,
        0.5,  0.5,  0.5,        1.0,  0.0,  0.0,        1.0,  0.0,

        -0.5, -0.5, -0.5,       0.0, -1.0,  0.0,        0.0,  1.0,
        0.5, -0.5, -0.5,        0.0, -1.0,  0.0,        1.0,  1.0,
        0.5, -0.5,  0.5,        0.0, -1.0,  0.0,        1.0,  0.0,
        0.5, -0.5,  0.5,        0.0, -1.0,  0.0,        1.0,  0.0,
        -0.5, -0.5,  0.5,       0.0, -1.0,  0.0,        0.0,  0.0,
        -0.5, -0.5, -0.5,       0.0, -1.0,  0.0,        0.0,  1.0,

        -0.5,  0.5, -0.5,       0.0,  1.0,  0.0,        0.0,  1.0,
        0.5,  0.5, -0.5,        0.0,  1.0,  0.0,        1.0,  1.0,
        0.5,  0.5,  0.5,        0.0,  1.0,  0.0,        1.0,  0.0,
        0.5,  0.5,  0.5,        0.0,  1.0,  0.0,        1.0,  0.0,
        -0.5,  0.5,  0.5,       0.0,  1.0,  0.0,        0.0,  0.0,
        -0.5,  0.5, -0.5,       0.0,  1.0,  0.0,        0.0,  1.0,
    ];

    let shader_lighting =
        shader::compile_from_sources(include_str!("lighting.vert"), include_str!("lighting.frag"))?;
    let shader_light_cube = shader::compile_from_sources(
        include_str!("light_cube.vert"),
        include_str!("light_cube.frag"),
    )?;
    let texture_wood_steel_border = texture::create(include_bytes!("wood_steel_border.png"))?;
    let texture_only_steel_border = texture::create(include_bytes!("steel_border.png"))?;

    let mut vao_cube: GLuint = 0;
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
            (8 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as i32,
            (3 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as i32,
            (6 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void,
        );
        gl::EnableVertexAttribArray(2);
    }
    let vao_cube = vao_cube;
    let vbo = vbo;

    let projection = nalgebra_glm::perspective(
        window.size().0 as f32 / window.size().1 as f32,
        num::Float::to_radians(45.0),
        0.1,
        100.0,
    );

    let mut camera = camera::start_from_world_pos(nalgebra_glm::vec3(0.0, 0.0, 3.0));
    let mut current_movement: [Option<Direction>; 6] = [None, None, None, None, None, None];
    let mut flashlight_state = true;

    let cube_radius: f32 = 10.0;
    let mut rng = rand::thread_rng();
    let mut create_random_vector = || {
        nalgebra_glm::vec3(
            cube_radius * rng.gen::<f32>() - (cube_radius / 2.0),
            cube_radius * rng.gen::<f32>() - (cube_radius / 2.0),
            cube_radius * rng.gen::<f32>() - (cube_radius / 2.0),
        )
    };

    let cube_positions: Vec<(nalgebra_glm::Vec3, nalgebra_glm::Vec3)> =
        std::iter::repeat_with(|| (create_random_vector(), create_random_vector()))
            .take(50)
            .collect();

    let point_light_positions: Vec<nalgebra_glm::Vec3> =
        std::iter::repeat_with(create_random_vector)
            .take(4)
            .collect();

    let mut event_pump = sdl.event_pump()?;
    let timer = sdl.timer()?;
    let mut last_ticks = timer.performance_counter() as f64;
    loop {
        let seconds = match process_events(
            &mut event_pump,
            &timer,
            &mut last_ticks,
            &mut camera,
            &mut current_movement,
            &mut flashlight_state,
        ) {
            Some(val) => val,
            None => break,
        };

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader_lighting.enable();

            shader_lighting.set_int("uMaterial.diffuse", 0)?;
            shader_lighting.set_int("uMaterial.specular", 1)?;

            shader_lighting.set_mat4("uProjection", &projection)?;
            shader_lighting.set_mat4("uView", &camera.get_view_matrix())?;
            shader_lighting.set_vec3("uViewPos", &camera.get_position())?;
            shader_lighting.set_float("uMaterial.shininess", 32.0)?;

            // Directional Lighting
            shader_lighting.set_vec3(
                "uDirectionalLight.direction",
                &nalgebra_glm::vec3(-0.2, -1.0, -0.3),
            )?;
            shader_lighting.set_vec3(
                "uDirectionalLight.ambient",
                &nalgebra_glm::vec3(0.05, 0.05, 0.05),
            )?;
            shader_lighting.set_vec3(
                "uDirectionalLight.diffuse",
                &nalgebra_glm::vec3(0.4, 0.4, 0.4),
            )?;
            shader_lighting.set_vec3(
                "uDirectionalLight.specular",
                &nalgebra_glm::vec3(0.5, 0.5, 0.5),
            )?;

            // Spot Lighting
            shader_lighting.set_vec3("uSpotLight.position", &camera.get_position())?;
            shader_lighting.set_vec3("uSpotLight.direction", &camera.get_front())?;
            shader_lighting.set_float(
                "uSpotLight.inner_cutoff",
                num::Float::to_radians(12.5 as f32).cos(),
            )?;
            shader_lighting.set_float(
                "uSpotLight.outer_cutoff",
                num::Float::to_radians(17.5 as f32).cos(),
            )?;
            shader_lighting.set_float("uSpotLight.attenuation_constant", 1.0)?;
            shader_lighting.set_float("uSpotLight.attenuation_linear", 0.07)?;
            shader_lighting.set_float("uSpotLight.attenuation_quadratic", 0.017)?;
            shader_lighting.set_vec3("uSpotLight.ambient", &nalgebra_glm::vec3(0.1, 0.1, 0.1))?;
            shader_lighting.set_vec3("uSpotLight.diffuse", &nalgebra_glm::vec3(1.0, 1.0, 1.0))?;
            shader_lighting.set_vec3("uSpotLight.specular", &nalgebra_glm::vec3(2.0, 2.0, 2.0))?;
            shader_lighting.set_int("uFlashlight", flashlight_state as i32)?;

            // Point Lighting
            for (i, position) in point_light_positions.iter().enumerate() {
                shader_lighting
                    .set_vec3(format!("uPointLights[{}].position", i).as_str(), position)?;
                shader_lighting.set_vec3(
                    format!("uPointLights[{}].ambient", i).as_str(),
                    &nalgebra_glm::vec3(0.05, 0.05, 0.05),
                )?;
                shader_lighting.set_vec3(
                    format!("uPointLights[{}].diffuse", i).as_str(),
                    &nalgebra_glm::vec3(0.8, 0.8, 0.8),
                )?;
                shader_lighting.set_vec3(
                    format!("uPointLights[{}].specular", i).as_str(),
                    &nalgebra_glm::vec3(1.0, 1.0, 1.0),
                )?;
                shader_lighting.set_float(
                    format!("uPointLights[{}].attenuation_constant", i).as_str(),
                    1.0,
                )?;
                shader_lighting.set_float(
                    format!("uPointLights[{}].attenuation_linear", i).as_str(),
                    0.09,
                )?;
                shader_lighting.set_float(
                    format!("uPointLights[{}].attenuation_quadratic", i).as_str(),
                    0.032,
                )?;
            }

            gl::ActiveTexture(gl::TEXTURE0);
            texture_wood_steel_border.bind();
            gl::ActiveTexture(gl::TEXTURE1);
            texture_only_steel_border.bind();

            for (position, axis) in &cube_positions {
                let model = nalgebra_glm::rotate(
                    &nalgebra_glm::translate(&nalgebra_glm::one(), &position),
                    seconds,
                    &axis,
                );

                shader_lighting.set_mat4("uModel", &model)?;

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            shader_light_cube.enable();

            shader_light_cube.set_mat4("uProjection", &projection)?;
            shader_light_cube.set_mat4("uView", &camera.get_view_matrix())?;
            for position in &point_light_positions {
                let model = nalgebra_glm::translate(&nalgebra_glm::one(), &position);
                shader_light_cube.set_mat4("uModel", &model)?;
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            assert_eq!(gl::GetError(), 0);
        }

        window.gl_swap_window();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao_cube as *const GLuint);
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

const CAMERA_SPEED: f32 = 10.0;
const CAMERA_SENSITIVITY: f32 = 0.2;
fn process_events(
    event_pump: &mut EventPump,
    timer: &TimerSubsystem,
    last_ticks: &mut f64,
    camera: &mut Camera,
    current_movement: &mut [Option<Direction>; 6],
    flashlight_state: &mut bool,
) -> Option<f32> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return None,
            Event::KeyDown { keycode, .. } => match keycode {
                Some(Keycode::A) => current_movement[0] = Some(Direction::Left),
                Some(Keycode::D) => current_movement[1] = Some(Direction::Right),
                Some(Keycode::W) => current_movement[2] = Some(Direction::Forward),
                Some(Keycode::S) => current_movement[3] = Some(Direction::Backward),
                Some(Keycode::Space) => current_movement[4] = Some(Direction::Up),
                Some(Keycode::LAlt) => current_movement[5] = Some(Direction::Down),
                Some(Keycode::F) => *flashlight_state = !*flashlight_state,
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
    let delta_ticks = now_ticks - *last_ticks;
    let freq_ticks = timer.performance_frequency() as f64;
    *last_ticks = now_ticks;

    let delta_seconds = delta_ticks / freq_ticks;
    let camera_velocity = (delta_seconds * CAMERA_SPEED as f64) as f32;
    let seconds = timer.ticks() as f32 / 1000.0;

    let mouse_state = sdl2::mouse::RelativeMouseState::new(&event_pump);
    camera.update_orientation(
        mouse_state.x() as f32,
        -mouse_state.y() as f32,
        CAMERA_SENSITIVITY,
    );

    for movement in current_movement {
        match movement {
            Some(dir) => camera.update_position(*dir, camera_velocity),
            None => (),
        }
    }

    Some(seconds)
}
