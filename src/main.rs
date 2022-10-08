mod opengl;

#[derive(Debug)]
enum Error {
  Initialisation(String)
}

impl From<sdl2::video::WindowBuildError> for Error {
  fn from(error: sdl2::video::WindowBuildError) -> Self {
    Error::Initialisation(format!("{}", error))
  }
}

fn main() -> Result<(), Error> {
  let sdl = sdl2::init().map_err(|s| Error::Initialisation(s))?;
  let video_subsystem = sdl.video().map_err(|s| Error::Initialisation(s))?;
  {
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
  }
  let window = video_subsystem
    .window("Rust Renderer", 720, 480)
    .opengl()
    .resizable()
    .build()?;
  let _gl_context = window.gl_create_context().map_err(|s| Error::Initialisation(s))?;

  opengl::load_with(|s| video_subsystem.gl_get_proc_address(s));
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
    window.gl_swap_window();
  }

  Ok(())
}