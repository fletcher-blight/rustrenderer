extern crate gl;

use gl::types::*;
use image::ImageFormat;
use sdl2::event::Event;
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

    let mut program_id: GLuint = 0;
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
    let program_id = program_id;

    let texture_wall_raw_data = include_bytes!("wall.jpg");
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
    let vertices: [f32; 20] = [
        // vertices         texture coords
        -0.5, -0.5, 0.0,    0.0, 0.0,       // bottom left
        -0.5, 0.5, 0.0,     0.0, 2.0,       // top left
        0.5, 0.5, 0.0,      2.0, 2.0,       // top right
        0.5, -0.5, 0.0,     2.0, 0.0,       // bottom right
    ];

    #[rustfmt::skip]
    let indices: [u32; 6] = [
        0, 1, 2,
        2, 0, 3,
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

        let mut ebo: GLuint = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>())
                .try_into()
                .map_err(error_to_string())?,
            indices.as_ptr() as *const std::os::raw::c_void,
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

    unsafe {
        gl::UseProgram(program_id);

        let loc = gl::GetUniformLocation(
            program_id,
            String::from("Texture1\0").as_ptr() as *const GLchar,
        );
        if loc < 0 {
            panic!("Could not find `Texture1`");
        }
        gl::Uniform1i(loc, 0);

        let loc = gl::GetUniformLocation(
            program_id,
            String::from("Texture2\0").as_ptr() as *const GLchar,
        );
        if loc < 0 {
            panic!("Could not find `Texture2`");
        }
        gl::Uniform1i(loc, 1);
    }

    let mut event_pump = sdl.event_pump()?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => (),
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.5, 0.9, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_wall);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_face);

            gl::UseProgram(program_id);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
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
