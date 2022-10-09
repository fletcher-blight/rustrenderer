extern crate gl;
use gl::types::*;

pub type Id = GLuint;

pub enum Component {
  Dot,
  Line,
  Triangle,
  Square,
}

pub enum Data {
  SignedByte,
  UnsignedByte,
  SignedShort,
  UnsignedShort,
  SignedInt,
  UnsignedInt,
  HalfFloat,
  Float,
  Fixed,
  IntA2B10G10R10,
  UnsignedIntA2B10G10R10,
  UnsignedIntB10fG11fR11f,
}

pub fn create() -> Id {
  let mut id: Id = 0;
  unsafe { gl::GenVertexArrays(1, &mut id) };
  id
}

pub fn delete(ids: &[Id]) -> () {
  unsafe { gl::DeleteVertexArrays(ids.len() as GLsizei, ids.as_ptr()) };
}

pub fn bind(id: Id) -> () {
  unsafe { gl::BindVertexArray(id) };
}

pub fn configure_attribute(
  index: u32,
  component: Component,
  data: Data,
  normalise: bool,
  stride: u32,
  offset: u32)
  -> () {
  unsafe {
    gl::VertexAttribPointer(
      index,
      component.into(),
      data.into(),
      normalise as u8,
      stride as i32,
      offset as *const std::os::raw::c_void
    );
  }
}

pub fn enable(index: u32) -> () {
  unsafe { gl::EnableVertexAttribArray(index) };
}

impl From<Component> for GLint {
  fn from(value: Component) -> Self {
    match value {
      Component::Dot => 1,
      Component::Line => 2,
      Component::Triangle => 3,
      Component::Square => 4,
    }
  }
}

impl From<Data> for GLenum {
  fn from(value: Data) -> Self {
    match value {
      Data::SignedByte => gl::BYTE,
      Data::UnsignedByte => gl::UNSIGNED_BYTE,
      Data::SignedShort => gl::SHORT,
      Data::UnsignedShort => gl::UNSIGNED_SHORT,
      Data::SignedInt => gl::INT,
      Data::UnsignedInt => gl::UNSIGNED_INT,
      Data::HalfFloat => gl::HALF_FLOAT,
      Data::Float => gl::FLOAT,
      Data::Fixed => gl::FIXED,
      Data::IntA2B10G10R10 => gl::INT_2_10_10_10_REV,
      Data::UnsignedIntA2B10G10R10 => gl::UNSIGNED_INT_2_10_10_10_REV,
      Data::UnsignedIntB10fG11fR11f => gl::UNSIGNED_INT_10F_11F_11F_REV,
    }
  }
}