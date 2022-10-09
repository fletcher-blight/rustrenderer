#[derive(Debug)]
enum Error {
  Initialisation(String),
  Shaders(String),
}

impl From<sdl2::video::WindowBuildError> for Error {
  fn from(error: sdl2::video::WindowBuildError) -> Self {
    Error::Initialisation(format!("{}", error))
  }
}

impl From<opengl::Error> for Error {
  fn from(error: opengl::Error) -> Self {
    Error::Shaders(format!("{:?}", error))
  }
}

fn main() -> Result<(), Error> {
  let sdl = sdl2::init().map_err(|s| Error::Initialisation(s))?;
  let video_subsystem = sdl.video().map_err(|s| Error::Initialisation(s))?;
  {
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
  }
  let window = video_subsystem
    .window("Rust Renderer", 720, 480)
    .opengl()
    .resizable()
    .build()?;
  let _gl_context = window.gl_create_context().map_err(|s| Error::Initialisation(s))?;
  let _gl = opengl::load_with(|s| video_subsystem.gl_get_proc_address(s));

  opengl::check_for_error()?;

  let vert_shader = shader_from_source(opengl::ShaderType::Vertex, include_str!("triangle.vert"))?;
  let frag_shader = shader_from_source(opengl::ShaderType::Fragment, include_str!("triangle.frag"))?;
  let program = program_from_shaders(&[vert_shader, frag_shader])?;

  let vertices: [f32; 18] = [
    // position       colour
    0.0, 0.5, 0.0,    1.0, 0.0, 0.0,
    0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
    -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,
  ];

  let vao = opengl::gen_vertex_array();
  let vbo = opengl::gen_buffer();
  opengl::bind_vertex_array(vao);
  opengl::bind_buffer(opengl::BufferType::Array, vbo);
  opengl::set_buffer_data(opengl::BufferType::Array, &vertices, opengl::DrawType::Static);
  opengl::set_vertex_attrib_pointer(
    0,
    3,
    opengl::AttributeType::Float,
    false,
    6 * std::mem::size_of::<f32>() as i32,
    0);
  opengl::enable_vertex_attrib_array(0);
  opengl::set_vertex_attrib_pointer(
    1,
    3,
    opengl::AttributeType::Float,
    false,
    6 * std::mem::size_of::<f32>() as i32,
    3 * std::mem::size_of::<f32>() as u32);
  opengl::enable_vertex_attrib_array(1);
  opengl::bind_buffer(opengl::BufferType::Array, 0);
  opengl::bind_vertex_array(0);
  opengl::check_for_error()?;

  opengl::clear_colour(0.3, 0.3, 0.5, 1.0);
  let mut event_pump = sdl.event_pump().map_err(|s| Error::Initialisation(s))?;
  'main: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} => break 'main,
        _ => {},
      }
    }

    opengl::clear(opengl::ClearBit::ColourBufferBit);

    opengl::use_program(program);
    opengl::bind_vertex_array(vao);
    opengl::draw_arrays(opengl::DrawMode::Triangles, 0, 3);

    opengl::check_for_error()?;
    window.gl_swap_window();
  }

  Ok(())
}

fn shader_from_source(shader: opengl::ShaderType, source: &str) -> Result<opengl::Id, Error> {
  let id = opengl::create_shader(shader);
  opengl::set_shader_source(id, &std::ffi::CString::new(source).map_err(|s| Error::Shaders(format!("Invalid CString of source: {}", s)))?);
  opengl::compile_shader(id);
  opengl::check_shader_compilation(id)?;
  opengl::check_for_error()?;
  Ok(id)
}

fn program_from_shaders(shaders: &[opengl::Id]) -> Result<opengl::Id, Error> {
  let id = opengl::create_program();
  for shader in shaders {
    opengl::attach_shader(id, *shader);
  }

  opengl::link_program(id);
  opengl::check_program_linking(id)?;
  opengl::check_for_error()?;

  for shader in shaders {
    opengl::detach_shader(id, *shader);
  }
  Ok(id)
}