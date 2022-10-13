extern crate gl;

use gl::types::*;
use image::ImageFormat;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::ffi::CString;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

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

    let program_id: GLuint;
    unsafe {
        let source_vertex =
            CString::new(include_str!("triangle.vert")).map_err(error_to_string())?;
        let source_fragment =
            CString::new(include_str!("triangle.frag")).map_err(error_to_string())?;

        let shader_vertex = gl::CreateShader(gl::VERTEX_SHADER);
        let shader_fragment = gl::CreateShader(gl::FRAGMENT_SHADER);

        gl::ShaderSource(shader_vertex, 1, &source_vertex.as_ptr(), std::ptr::null());
        gl::ShaderSource(
            shader_fragment,
            1,
            &source_fragment.as_ptr(),
            std::ptr::null(),
        );

        gl::CompileShader(shader_vertex);
        gl::CompileShader(shader_fragment);

        let mut success: GLint = 0;
        gl::GetShaderiv(shader_vertex, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            panic!("Vertex shader failed ");
        }
        gl::GetShaderiv(shader_fragment, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            panic!("Fragment shader failed ");
        }

        program_id = gl::CreateProgram();
        gl::AttachShader(program_id, shader_vertex);
        gl::AttachShader(program_id, shader_fragment);
        gl::LinkProgram(program_id);
    }

    let texture_wall_raw_data = include_bytes!("container.jpg");
    let texture_wall_image =
        image::load_from_memory_with_format(texture_wall_raw_data, ImageFormat::Jpeg)
            .map_err(error_to_string())?;

    let texture_face_raw_data = include_bytes!("face.png");
    let mut texture_face_image =
        image::load_from_memory_with_format(texture_face_raw_data, ImageFormat::Png)
            .map_err(error_to_string())?;
    image::imageops::flip_vertical_in_place(&mut texture_face_image);
    let texture_face_image = texture_face_image;

    #[rustfmt::skip]
    let vertices: [f32; 180] = [
        // vertices         texture coords
        -0.5, -0.5, -0.5,  0.0, 0.0,
        0.5, -0.5, -0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5,  0.5,  0.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  1.0, 1.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
    ];

    let mut vao: GLuint = 0;
    let mut texture_wall: GLuint = 0;
    let mut texture_face: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo: GLuint = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>())
                .try_into()
                .map_err(error_to_string())?,
            vertices.as_ptr() as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as i32,
            (3 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::GenTextures(1, &mut texture_wall);
        gl::BindTexture(gl::TEXTURE_2D, texture_wall);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            texture_wall_image
                .width()
                .try_into()
                .map_err(error_to_string())?,
            texture_wall_image
                .height()
                .try_into()
                .map_err(error_to_string())?,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            texture_wall_image.as_bytes().as_ptr() as *const std::os::raw::c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::GenTextures(1, &mut texture_face);
        gl::BindTexture(gl::TEXTURE_2D, texture_face);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            texture_face_image
                .width()
                .try_into()
                .map_err(error_to_string())?,
            texture_face_image
                .height()
                .try_into()
                .map_err(error_to_string())?,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            texture_face_image.as_bytes().as_ptr() as *const std::os::raw::c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    let vao = vao;
    let texture_wall = texture_wall;
    let texture_face = texture_face;

    let mix_id: GLint = find_uniform(program_id, "MixPerc")?;
    let model_id: GLint = find_uniform(program_id, "model")?;
    let view_id: GLint = find_uniform(program_id, "view")?;
    let projection_id: GLint = find_uniform(program_id, "projection")?;

    unsafe {
        gl::UseProgram(program_id);
        let loc = find_uniform(program_id, "Texture1")?;
        gl::Uniform1i(loc, 0);
        let loc = find_uniform(program_id, "Texture2")?;
        gl::Uniform1i(loc, 1);
    }

    let mut mix_perc: f32 = 0.3;
    let mix_inc: f32 = 0.1;
    let cube_rotate_inc: f32 = 5.0;
    let cube_rotate_axis = nalgebra_glm::vec3(0.5, 1.0, 0.0);

    let mut model = nalgebra_glm::rotate(
        &num::one(),
        num::Float::to_radians(-55.0),
        &nalgebra_glm::vec3(1.0, 0.0, 0.0),
    );
    let view = nalgebra_glm::translate(&num::one(), &nalgebra_glm::vec3(0.0, 0.0, -3.0));
    let projection = nalgebra_glm::perspective(
        window.size().0 as f32 / window.size().1 as f32,
        num::Float::to_radians(45.0),
        0.1,
        100.0,
    );

    let cube_positions = [
        nalgebra_glm::vec3(0.0, 0.0, 0.0),
        nalgebra_glm::vec3(2.0, 5.0, -15.0),
        nalgebra_glm::vec3(-1.5, -2.2, -2.5),
        nalgebra_glm::vec3(-3.8, -2.0, -12.3),
        nalgebra_glm::vec3(2.4, -0.4, -3.5),
        nalgebra_glm::vec3(-1.7, 3.0, -7.5),
        nalgebra_glm::vec3(1.3, -2.0, -2.5),
        nalgebra_glm::vec3(1.5, 2.0, -2.5),
        nalgebra_glm::vec3(1.5, 0.2, -1.5),
        nalgebra_glm::vec3(-1.3, 1.0, -1.5),
    ];

    let mut event_pump = sdl.event_pump()?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => mix_perc = num::clamp(mix_perc + mix_inc, 0.0, 1.0),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => mix_perc = num::clamp(mix_perc - mix_inc, 0.0, 1.0),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    model = nalgebra_glm::rotate(
                        &model,
                        num::Float::to_radians(cube_rotate_inc),
                        &cube_rotate_axis,
                    )
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    model = nalgebra_glm::rotate(
                        &model,
                        num::Float::to_radians(-cube_rotate_inc),
                        &cube_rotate_axis,
                    )
                }
                _ => (),
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.5, 0.9, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_wall);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_face);

            gl::UseProgram(program_id);

            gl::UniformMatrix4fv(
                view_id,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&view).as_ptr(),
            );
            gl::UniformMatrix4fv(
                projection_id,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&projection).as_ptr(),
            );
            gl::Uniform1f(mix_id, mix_perc);

            gl::BindVertexArray(vao);
            for (i, pos) in cube_positions.iter().enumerate() {
                let this_model = nalgebra_glm::translate(&num::one(), &pos);
                let this_model = nalgebra_glm::rotate(
                    &this_model,
                    num::Float::to_radians(20.0 * i as f32),
                    &nalgebra_glm::vec3(1.0, 0.3, 0.5),
                ) * model;
                gl::UniformMatrix4fv(
                    model_id,
                    1,
                    gl::FALSE,
                    nalgebra_glm::value_ptr(&this_model).as_ptr(),
                );
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        window.gl_swap_window();
    }

    Ok(())
}

fn error_to_string<E>() -> fn(E) -> String
where
    E: std::fmt::Display,
{
    |e: E| e.to_string()
}

fn find_uniform(program_id: GLuint, name: &str) -> Result<GLint, String> {
    let name = CString::new(name).map_err(error_to_string())?;
    let id = unsafe { gl::GetUniformLocation(program_id, name.as_ptr() as *const GLchar) };
    if id < 0 {
        return Err(String::from("Could not find `MixPerc`"));
    }
    Ok(id)
}
