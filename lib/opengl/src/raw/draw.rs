extern crate gl;
use gl::types::*;

#[derive(Copy, Clone)]
pub enum Buffer {
  Colour,
  Depth,
  Stencil,
}

pub enum Mode {
  Points,
  LineStrip,
  LineLoop,
  Lines,
  LineStripAdjacency,
  LinesAdjacency,
  TriangleStrip,
  TriangleFan,
  Triangles,
  TriangleStripAdjacency,
  TrianglesAdjacency,
  Patches,
}

pub enum Data {
  UnsignedByte,
  UnsignedShort,
  UnsignedInt,
}

pub fn background(r: f32, g: f32, b: f32, a: f32) -> () {
  unsafe { gl::ClearColor(r, g, b, a) };
}

pub fn clear(buffers: &[Buffer]) -> () {
  let bits  = buffers.iter()
    .map(|b| (*b).into())
    .fold(0, |res, b: GLbitfield| res | b);
  unsafe { gl::Clear(bits) };
}

pub fn arrays(mode: Mode, first: u32, count: u32) -> () {
  unsafe { gl::DrawArrays(mode.into(), first as i32, count as GLsizei) };
}

pub fn elements(mode: Mode, num_indices: u32, data: Data, offset: u32) -> () {
  let offset_ptr = offset as *const std::os::raw::c_void;
  unsafe { gl::DrawElements(mode.into(), num_indices as i32, data.into(), offset_ptr) };
}

impl From<Buffer> for GLbitfield {
  fn from(value: Buffer) -> Self {
    match value {
      Buffer::Colour => gl::COLOR_BUFFER_BIT,
      Buffer::Depth => gl::DEPTH_BUFFER_BIT,
      Buffer::Stencil => gl::STENCIL_BUFFER_BIT,
    }
  }
}

impl From<Mode> for GLenum {
  fn from(value: Mode) -> Self {
    match value {
      Mode::Points => gl::POINTS,
      Mode::LineStrip => gl::LINE_STRIP,
      Mode::LineLoop => gl::LINE_LOOP,
      Mode::Lines => gl::LINES,
      Mode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
      Mode::LinesAdjacency => gl::LINES_ADJACENCY,
      Mode::TriangleStrip => gl::TRIANGLE_STRIP,
      Mode::TriangleFan => gl::TRIANGLE_FAN,
      Mode::Triangles => gl::TRIANGLES,
      Mode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
      Mode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
      Mode::Patches => gl::PATCHES,
    }
  }
}

impl From<Data> for GLenum {
  fn from(value: Data) -> Self {
    match value {
      Data::UnsignedByte => gl::UNSIGNED_BYTE,
      Data::UnsignedShort => gl::UNSIGNED_SHORT,
      Data::UnsignedInt => gl::UNSIGNED_INT,
    }
  }
}