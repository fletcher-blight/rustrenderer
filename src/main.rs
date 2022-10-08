mod sdl;

use opengl::{ shader, program };
use std::ffi::CString;



fn main()
{
  let sdl = sdl::Context::init("Rust Renderer", 600, 480);
  opengl::set_loader(sdl.proc_address_getter());

  let vertex_shader = shader::Context::from_source(&CString::new("").unwrap(), shader::Kind::Vertex).unwrap();
  let fragment_shader = shader::Context::from_source(&CString::new("").unwrap(), shader::Kind::Fragment).unwrap();
  let _program = program::Context::from_shaders(&[vertex_shader, fragment_shader]);

  sdl.run_window_loop(|events| {
    for event in events {
      if sdl::is_quit_event(&event) {
        return sdl::WindowState::Exit;
      }
    }
    return sdl::WindowState::Commit;
  });

  //
  // let vert_shader = render_gl::Shader::from_vert_source(
  //   gl.clone(),
  //   &CString::new(include_str!("triangle.vert")).unwrap()
  // ).unwrap();
  //
  // let frag_shader = render_gl::Shader::from_frag_source(
  //   gl.clone(),
  //   &CString::new(include_str!("triangle.frag")).unwrap()
  // ).unwrap();
  //
  // let shader_program = render_gl::Program::from_shaders(gl.clone(), &[vert_shader, frag_shader]).unwrap();
  //
  // let vertices: Vec<f32> = vec![
  //   // positions        // colours
  //   -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,  // bottom right
  //   0.5, -0.5, 0.0,     0.0, 1.0, 0.0,  // bottom left
  //   0.0, 0.5, 0.0,      0.0, 0.0, 1.0,  // top centre
  // ];
  //
  // let mut vbo: GLuint = 0;
  // unsafe {
  //   gl.GenBuffers(1, &mut vbo);
  // }
  // unsafe {
  //   gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
  //   gl.BufferData(
  //     gl::ARRAY_BUFFER,
  //     (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
  //     vertices.as_ptr() as *const GLvoid,
  //     gl::STATIC_DRAW
  //   );
  //   gl.BindBuffer(gl::ARRAY_BUFFER, 0);
  // }
  //
  // let mut vao: GLuint = 0;
  // unsafe {
  //   gl.GenVertexArrays(1, &mut vao);
  // }
  // unsafe {
  //   gl.BindVertexArray(vao);
  //   gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
  //   gl.EnableVertexAttribArray(0);
  //   gl.VertexAttribPointer(
  //     0, // index of generic vertex attribute ("layout (location = 0)")
  //     3, // the number of components per generic vertex attribute
  //     gl::FLOAT,
  //     gl::FALSE, // normalized (int-to-float conversion)
  //     (6 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
  //     std::ptr::null() // offset of first component
  //   );
  //   gl.EnableVertexAttribArray(1);
  //   gl.VertexAttribPointer(
  //     1, // index of generic vertex attribute ("layout (location = 0)")
  //     3, // the number of components per generic vertex attribute
  //     gl::FLOAT,
  //     gl::FALSE, // normalized (int-to-float conversion)
  //     (6 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
  //     (3 * std::mem::size_of::<f32>()) as *const GLvoid // offset of first component
  //   );
  //   gl.BindBuffer(gl::ARRAY_BUFFER, 0);
  //   gl.BindVertexArray(0);
  // }
  //
  // unsafe {
  //   gl.Viewport(0, 0, window.size().0 as GLint, window.size().1 as GLint);
  //   gl.ClearColor(0.3, 0.3, 0.5, 1.0);
  // }
  //
  // let mut event_pump = sdl.event_pump().unwrap();
  // 'main: loop {
  //   for event in event_pump.poll_iter() {
  //     match event {
  //       sdl2::event::Event::Quit {..} => break 'main,
  //       _ => {},
  //     }
  //   }
  //
  //   unsafe {
  //     gl.Clear(gl::COLOR_BUFFER_BIT);
  //   }
  //
  //   shader_program.set_used();
  //   unsafe {
  //     gl.BindVertexArray(vao);
  //     gl.DrawArrays(gl::TRIANGLES, 0, 3);
  //   }
  //   window.gl_swap_window();
  // }

}