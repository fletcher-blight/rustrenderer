extern crate sdl2;
extern crate gl;

mod render_gl;
mod resources;

use resources::Resources;
use std::path::Path;
use std::ffi::CString;
use gl::types::*;

fn main()
{
  let sdl = sdl2::init().unwrap();
  let video_subsystem = sdl.video().unwrap();
  let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(4, 5);

  let window = video_subsystem
    .window("Rust Renderer", 640, 480)
    .opengl()
    .resizable()
    .build()
    .unwrap();

  let _gl_context = window.gl_create_context().unwrap();
  let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

  let vert_shader = render_gl::Shader::from_vert_source(
    gl.clone(),
    &CString::new(include_str!("triangle.vert")).unwrap()
  ).unwrap();

  let frag_shader = render_gl::Shader::from_frag_source(
    gl.clone(),
    &CString::new(include_str!("triangle.frag")).unwrap()
  ).unwrap();

  let shader_program = render_gl::Program::from_shaders(gl.clone(), &[vert_shader, frag_shader]).unwrap();

  let vertices: Vec<f32> = vec![
    // positions        // colours
    -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,  // bottom right
    0.5, -0.5, 0.0,     0.0, 1.0, 0.0,  // bottom left
    0.0, 0.5, 0.0,      0.0, 0.0, 1.0,  // top centre
  ];

  let mut vbo: GLuint = 0;
  unsafe {
    gl.GenBuffers(1, &mut vbo);
  }
  unsafe {
    gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl.BufferData(
      gl::ARRAY_BUFFER,
      (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
      vertices.as_ptr() as *const GLvoid,
      gl::STATIC_DRAW
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
  }

  let mut vao: GLuint = 0;
  unsafe {
    gl.GenVertexArrays(1, &mut vao);
  }
  unsafe {
    gl.BindVertexArray(vao);
    gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl.EnableVertexAttribArray(0);
    gl.VertexAttribPointer(
      0, // index of generic vertex attribute ("layout (location = 0)")
      3, // the number of components per generic vertex attribute
      gl::FLOAT,
      gl::FALSE, // normalized (int-to-float conversion)
      (6 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
      std::ptr::null() // offset of first component
    );
    gl.EnableVertexAttribArray(1);
    gl.VertexAttribPointer(
      1, // index of generic vertex attribute ("layout (location = 0)")
      3, // the number of components per generic vertex attribute
      gl::FLOAT,
      gl::FALSE, // normalized (int-to-float conversion)
      (6 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
      (3 * std::mem::size_of::<f32>()) as *const GLvoid // offset of first component
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
  }

  unsafe {
    gl.Viewport(0, 0, window.size().0 as GLint, window.size().1 as GLint);
    gl.ClearColor(0.3, 0.3, 0.5, 1.0);
  }

  let mut event_pump = sdl.event_pump().unwrap();
  'main: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} => break 'main,
        _ => {},
      }
    }

    unsafe {
      gl.Clear(gl::COLOR_BUFFER_BIT);
    }

    shader_program.set_used();
    unsafe {
      gl.BindVertexArray(vao);
      gl.DrawArrays(gl::TRIANGLES, 0, 3);
    }
    window.gl_swap_window();
  }
}

