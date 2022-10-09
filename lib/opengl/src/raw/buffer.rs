extern crate gl;
use gl::types::*;

pub type Id = GLuint;

pub enum Target {
  Array,
  AtomicCounter,
  CopyRead,
  CopyWrite,
  DispatchIndirect,
  DrawIndirect,
  ElementArray,
  PixelPack,
  PixelUnpack,
  Query,
  ShaderStorage,
  Texture,
  TransformFeedback,
  Uniform,
}

pub enum Usage {
  StreamDraw,
  StreamRead,
  StreamCopy,
  StaticDraw,
  StaticRead,
  StaticCopy,
  DynamicDraw,
  DynamicRead,
  DynamicCopy,
}

pub fn create() -> Id {
  let mut id: Id = 0;
  unsafe { gl::GenBuffers(1, &mut id) };
  id
}

pub fn delete(ids: &[Id]) -> () {
  unsafe { gl::DeleteBuffers(ids.len() as GLsizei, ids.as_ptr()) };
}

pub fn bind(target: Target, id: Id) -> () {
  unsafe { gl::BindBuffer(target.into(), id) };
}

pub fn set_bound_data<Data>(target: Target, data: &[Data], usage: Usage) -> () {
  let size = (data.len() * std::mem::size_of::<Data>()) as GLsizeiptr;
  let data_ptr = data.as_ptr() as *const GLvoid;
  unsafe { gl::BufferData(target.into(), size, data_ptr, usage.into()) };
}

impl From<Target> for GLenum {
  fn from(value: Target) -> Self {
    match value {
      Target::Array => gl::ARRAY_BUFFER,
      Target::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
      Target::CopyRead => gl::COPY_READ_BUFFER,
      Target::CopyWrite => gl::COPY_WRITE_BUFFER,
      Target::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
      Target::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
      Target::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
      Target::PixelPack => gl::PIXEL_PACK_BUFFER,
      Target::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
      Target::Query => gl::QUERY_BUFFER,
      Target::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
      Target::Texture => gl::TEXTURE_BUFFER,
      Target::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
      Target::Uniform => gl::UNIFORM_BUFFER,
    }
  }
}

impl From<Usage> for GLenum {
  fn from(value: Usage) -> Self {
    match value {
      Usage::StreamDraw => gl::STREAM_DRAW,
      Usage::StreamRead => gl::STREAM_READ,
      Usage::StreamCopy => gl::STREAM_COPY,
      Usage::StaticDraw => gl::STATIC_DRAW,
      Usage::StaticRead => gl::STATIC_READ,
      Usage::StaticCopy => gl::STATIC_COPY,
      Usage::DynamicDraw => gl::DYNAMIC_DRAW,
      Usage::DynamicRead => gl::DYNAMIC_READ,
      Usage::DynamicCopy => gl::DYNAMIC_COPY,
    }
  }
}