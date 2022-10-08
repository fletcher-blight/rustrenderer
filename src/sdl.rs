pub struct Context {
  sdl: sdl2::Sdl,
  video_subsystem: sdl2::VideoSubsystem,
  window: sdl2::video::Window,
  _gl_context: sdl2::video::GLContext,
}

#[derive(Debug)]
pub enum WindowState {
  Commit,
  Exit,
}

impl Context {
  pub fn init(title: &str, width: u32, height: u32) -> Context {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    {
      let gl_attr = video_subsystem.gl_attr();
      gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
      gl_attr.set_context_version(4, 5);
    }

    let window = video_subsystem
      .window(title, width, height)
      .opengl()
      .resizable()
      .build()
      .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    Context { sdl, video_subsystem, window, _gl_context: gl_context }
  }

  pub fn proc_address_getter(&self) -> impl Fn(&'static str) -> *const () + '_{
    return |s| self.video_subsystem.gl_get_proc_address(s);
  }

  pub fn run_window_loop(&self, work: fn(sdl2::event::EventPollIterator) -> WindowState) {
    let mut event_pump = self.sdl.event_pump().unwrap();
    loop {
      let events = event_pump.poll_iter();
      let work_state = work(events);
      match work_state {
        WindowState::Commit => self.window.gl_swap_window(),
        WindowState::Exit => break,
      }
    }
  }
}

pub fn is_quit_event(event: &sdl2::event::Event) -> bool {
  match event {
    sdl2::event::Event::Quit{..} => true,
    _ => false,
  }
}
