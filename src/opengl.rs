extern crate gl;
use gl::types::*;

pub fn load_with<Loader>(loader: Loader)
  where Loader: Fn(&str) -> *const() {
  gl::load_with(|s| loader(s) as *const std::os::raw::c_void);
}

pub fn clear_colour(r: f32, g: f32, b: f32, a: f32) {
  unsafe { gl::ClearColor(r, g, b, a) };
}

#[repr(u32)]
pub enum ClearBit {
  ColourBufferBit = gl::COLOR_BUFFER_BIT,
}

pub fn clear(bitfield: ClearBit) {
  unsafe { gl::Clear(bitfield as GLenum) };
}