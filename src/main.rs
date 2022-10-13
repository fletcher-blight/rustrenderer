extern crate gl;

use gl::types::*;
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
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    #[rustfmt::skip]
    let vertices_cube: [f32; 108] = [
        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        -0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

        0.5,  0.5,  0.5,
        0.5,  0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,

        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,

        -0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
    ];

    let program_cube =
        program_id_from_shaders(&[include_str!("cube.vert"), include_str!("cube.frag")])?;
    let program_light =
        program_id_from_shaders(&[include_str!("light.vert"), include_str!("light.frag")])?;

    let mut vao_cube: GLuint = 0;
    let mut vao_light: GLuint = 0;
    unsafe {
        let mut vbo: GLuint = 0;
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
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenVertexArrays(1, &mut vao_cube);
        gl::GenVertexArrays(1, &mut vao_light);

        for vao in [vao_cube, vao_light] {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }
    }

    unsafe {
        gl::UseProgram(program_cube);
    }
    let loc_cube_model: GLuint = find_uniform(program_cube, "Model")?;
    let loc_cube_view: GLuint = find_uniform(program_cube, "View")?;
    let loc_cube_projection: GLuint = find_uniform(program_cube, "Projection")?;
    let loc_cube_light_colour: GLuint = find_uniform(program_cube, "LightColour")?;
    let loc_cube_object_colour: GLuint = find_uniform(program_cube, "ObjectColour")?;

    unsafe {
        gl::UseProgram(program_light);
    }
    let loc_light_model: GLuint = find_uniform(program_light, "Model")?;
    let loc_light_view: GLuint = find_uniform(program_light, "View")?;
    let loc_light_projection: GLuint = find_uniform(program_light, "Projection")?;

    let projection = nalgebra_glm::perspective(
        window.size().0 as f32 / window.size().1 as f32,
        num::Float::to_radians(45.0),
        0.1,
        100.0,
    );

    let camera_pos = nalgebra_glm::vec3(5.0, 1.5, 5.0);

    let view = nalgebra_glm::look_at(
        &camera_pos,
        &nalgebra_glm::vec3(0.0, 0.0, 0.0),
        &nalgebra_glm::vec3(0.0, 1.0, 0.0),
    );

    let light_pos = nalgebra_glm::vec3(1.2, 1.2, -1.0);

    let mut event_pump = sdl.event_pump()?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => (),
            }
        }

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(program_cube);

            let model = nalgebra_glm::translate(&num::one(), &nalgebra_glm::vec3(0.0, 0.0, 0.0));

            gl::UniformMatrix4fv(
                loc_cube_model as i32,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&model).as_ptr(),
            );
            gl::UniformMatrix4fv(
                loc_cube_view as i32,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&view).as_ptr(),
            );
            gl::UniformMatrix4fv(
                loc_cube_projection as i32,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&projection).as_ptr(),
            );
            gl::Uniform3f(loc_cube_light_colour as i32, 1.0, 1.0, 1.0);
            gl::Uniform3f(loc_cube_object_colour as i32, 1.0, 0.5, 0.31);

            gl::BindVertexArray(vao_cube);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // =====================

            gl::UseProgram(program_light);

            let model = nalgebra_glm::scale(
                &nalgebra_glm::translate(&num::one(), &light_pos),
                &nalgebra_glm::vec3(0.2, 0.2, 0.2),
            );

            gl::UniformMatrix4fv(
                loc_light_model as i32,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&model).as_ptr(),
            );
            gl::UniformMatrix4fv(
                loc_light_view as i32,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&view).as_ptr(),
            );
            gl::UniformMatrix4fv(
                loc_light_projection as i32,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&projection).as_ptr(),
            );

            gl::BindVertexArray(vao_light);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            assert_eq!(gl::GetError(), 0);
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

fn find_uniform(program_id: GLuint, name: &str) -> Result<GLuint, String> {
    let cstr = CString::new(name).map_err(error_to_string())?;
    let id = unsafe { gl::GetUniformLocation(program_id, cstr.as_ptr() as *const GLchar) };
    if id < 0 {
        return Err(format!("Could not find {}", name));
    }
    Ok(id as GLuint)
}

fn compile_shader(source: &str, kind: GLuint) -> Result<GLuint, String> {
    let cstr = CString::new(source).map_err(error_to_string())?;

    let id: GLuint;
    unsafe {
        id = gl::CreateShader(kind);
        gl::ShaderSource(id, 1, &cstr.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    };

    let mut success: GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        return Err(format!("Failed to compile {}", source));
    }

    Ok(id)
}

fn program_id_from_shaders(sources: &[&str; 2]) -> Result<GLuint, String> {
    let shaders = [
        compile_shader(sources[0], gl::VERTEX_SHADER)?,
        compile_shader(sources[1], gl::FRAGMENT_SHADER)?,
    ];

    let id = unsafe { gl::CreateProgram() };
    for shader in &shaders {
        unsafe {
            gl::AttachShader(id, *shader);
        }
    }

    let mut success: GLint = 0;
    unsafe {
        gl::LinkProgram(id);
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: GLint = 0;
        unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) };
        let error = CString::new(
            std::iter::repeat(' ')
                .take(len as usize)
                .collect::<String>(),
        )
        .map_err(error_to_string())?;
        unsafe {
            gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar)
        };
        return Err(format!(
            "Failed to link program: {}",
            error.to_string_lossy().into_owned(),
        ));
    }

    for shader in &shaders {
        unsafe {
            gl::DetachShader(id, *shader);
        }
    }
    Ok(id)
}
