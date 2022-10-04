extern crate sdl2;

fn main() {
  let sdl = sdl2::init().unwrap();
  let video_subsystem = sdl.video().unwrap();
  let _window = video_subsystem
    .window("Rust Renderer", 640, 480)
    .resizable()
    .build()
    .unwrap();
}