use image::ImageFormat;
use opengl::raw::draw::{Data, Mode};
use opengl::raw::texture::{DataFormat, DataType, Filter, InternalFormat, Property, Target, Wrap};

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

impl From<opengl::raw::error::Error> for Error {
  fn from(error: opengl::raw::error::Error) -> Self {
    Error::Shaders(format!("{:?}", error))
  }
}

impl From<image::ImageError> for Error {
  fn from(error: image::ImageError) -> Self {
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
  let _gl = opengl::raw::load_with(|s| video_subsystem.gl_get_proc_address(s));

  opengl::raw::error::check().unwrap();

  let vert_shader = shader_from_source(opengl::raw::shader::Type::Vertex, include_str!("triangle.vert"))?;
  let frag_shader = shader_from_source(opengl::raw::shader::Type::Fragment, include_str!("triangle.frag"))?;
  let program = program_from_shaders(&[vert_shader, frag_shader])?;

  let texture_file_contents = include_bytes!("wall.jpg");
  let texture_image = image::load_from_memory_with_format(texture_file_contents, ImageFormat::Jpeg)?;
  let texture_data = texture_image.as_bytes();
  let vertices: [f32; 32] = [
    // positions          // colors           // texture coords
    0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
    0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
    -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
    -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0,  // top left
  ];
  let indices: [u32; 6] = [
    0, 1, 3, // first triangle
    1, 2, 3  // second triangle
  ];

  let vao = opengl::raw::vertex_array::create();
  let vbo = opengl::raw::buffer::create();
  let ebo = opengl::raw::buffer::create();

  opengl::raw::vertex_array::bind(vao);

  opengl::raw::buffer::bind(opengl::raw::buffer::Target::Array, vbo);
  opengl::raw::buffer::set_bound_data(opengl::raw::buffer::Target::Array, &vertices, opengl::raw::buffer::Usage::StaticDraw);

  opengl::raw::buffer::bind(opengl::raw::buffer::Target::ElementArray, ebo);
  opengl::raw::buffer::set_bound_data(opengl::raw::buffer::Target::ElementArray, &indices, opengl::raw::buffer::Usage::StaticDraw);

  opengl::raw::vertex_array::configure_attribute(
    0,
    opengl::raw::vertex_array::Component::Triangle,
    opengl::raw::vertex_array::Data::Float,
    false,
    8 * std::mem::size_of::<f32>() as u32,
    0
  );
  opengl::raw::vertex_array::enable(0);
  opengl::raw::vertex_array::configure_attribute(
    1,
    opengl::raw::vertex_array::Component::Triangle,
    opengl::raw::vertex_array::Data::Float,
    false,
    8 * std::mem::size_of::<f32>() as u32,
    3 * std::mem::size_of::<f32>() as u32
  );
  opengl::raw::vertex_array::enable(1);
  opengl::raw::vertex_array::configure_attribute(
    2,
    opengl::raw::vertex_array::Component::Line,
    opengl::raw::vertex_array::Data::Float,
    false,
    8 * std::mem::size_of::<f32>() as u32,
    6 * std::mem::size_of::<f32>() as u32
  );
  opengl::raw::vertex_array::enable(2);
  opengl::raw::error::check()?;



  let tbo = opengl::raw::texture::create();
  opengl::raw::texture::bind(Target::Value2D, tbo);
  opengl::raw::texture::set_texture_property(Target::Value2D, Property::WrapS, Wrap::Repeat);
  opengl::raw::texture::set_texture_property(Target::Value2D, Property::WrapT, Wrap::Repeat);
  opengl::raw::texture::set_texture_property(Target::Value2D, Property::MinifyFilter, Filter::Linear);
  opengl::raw::texture::set_texture_property(Target::Value2D, Property::MagnifyFilter, Filter::Linear);
  opengl::raw::texture::apply_2d_image(
    Target::Value2D,
    0,
    InternalFormat::RGB,
    texture_image.width(),
    texture_image.height(),
    DataFormat::RGB,
    DataType::U8,
    texture_data);
  opengl::raw::texture::generate_mipmap(Target::Value2D);

  opengl::raw::draw::background(0.3, 0.3, 0.5, 1.0);
  let mut event_pump = sdl.event_pump().map_err(|s| Error::Initialisation(s))?;
  'main: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} => break 'main,
        _ => {},
      }
    }

    opengl::raw::draw::clear(&[opengl::raw::draw::Buffer::Colour]);

    opengl::raw::texture::bind(Target::Value2D, tbo);
    opengl::raw::program::enable(program);
    opengl::raw::vertex_array::bind(vao);
    opengl::raw::draw::elements(Mode::Triangles, 6, Data::UnsignedInt, 0);
    // opengl::raw::draw::arrays(opengl::raw::draw::Mode::Triangles, 0, 3);

    opengl::raw::error::check()?;
    window.gl_swap_window();
  }

  Ok(())
}

fn shader_from_source(shader: opengl::raw::shader::Type, source: &str) -> Result<opengl::raw::shader::Id, Error> {
  use opengl::raw::shader::*;
  let id = create(shader);
  set_source(id, &source).map_err(|e| crate::Error::Shaders(e.to_string()))?;
  compile(id);
  if was_compile_successful(id) {
    return Ok(id);
  }
  let len = info_log_length(id);
  let info_log = get_info_log(id, len);
  Err(crate::Error::Shaders(info_log))
}

fn program_from_shaders(shaders: &[opengl::raw::shader::Id]) -> Result<opengl::raw::program::Id, Error> {
  use opengl::raw::program::*;
  let id = create();
  for shader in shaders {
    attach(id, *shader);
  }
  link(id);
  for shader in shaders {
    detach(id, *shader);
  }

  if was_link_successful(id) {
    return Ok(id);
  }

  let len = info_log_length(id);
  let info_log = get_info_log(id, len);
  Err(crate::Error::Shaders(info_log))
}