extern crate gl;

use std::ffi::CString;
use gl::types::*;

#[derive(Debug)]
pub enum GLError {
  InvalidEnum,
  InvalidFrameBufferOperation,
  InvalidIndex,
  InvalidOperation,
  InvalidValue,
  Unknown(u32),
}

#[derive(Debug)]
pub enum Error {
  CanNotConvertToCString(String),
  ShaderCompilation(String),
  ProgramLinkage(String),
  GL(GLError),
  UnknownUniformName(String),
}

pub fn load_with<Loader>(loader: Loader)
  where Loader: Fn(&str) -> *const() {
  gl::load_with(|s| loader(s) as *const std::os::raw::c_void);
}

pub fn clear_colour(r: f32, g: f32, b: f32, a: f32) -> () {
  unsafe { gl::ClearColor(r, g, b, a) };
}

pub fn check_for_error() -> Result<(), Error> {
  match unsafe { gl::GetError() } {
    0 => Ok(()),
    gl::INVALID_ENUM => Err(Error::GL(GLError::InvalidEnum)),
    gl::INVALID_FRAMEBUFFER_OPERATION => Err(Error::GL(GLError::InvalidFrameBufferOperation)),
    gl::INVALID_INDEX => Err(Error::GL(GLError::InvalidIndex)),
    gl::INVALID_OPERATION => Err(Error::GL(GLError::InvalidOperation)),
    gl::INVALID_VALUE => Err(Error::GL(GLError::InvalidValue)),
    value => Err(Error::GL(GLError::Unknown(value))),
  }
}

pub enum ClearBit {
  ColourBufferBit,
}

impl From<ClearBit> for GLenum {
  fn from(value: ClearBit) -> Self {
    match value {
      ClearBit::ColourBufferBit => gl::COLOR_BUFFER_BIT,
    }
  }
}

pub fn clear(bitfield: ClearBit) -> () {
  unsafe { gl::Clear(bitfield.into()) };
}

pub type Id = GLuint;

pub enum ShaderType {
  Vertex,
  Fragment,
}

impl From<ShaderType> for GLenum {
  fn from(value: ShaderType) -> Self {
    match value {
      ShaderType::Vertex => gl::VERTEX_SHADER,
      ShaderType::Fragment => gl::FRAGMENT_SHADER,
    }
  }
}

pub fn create_shader(kind: ShaderType) -> Id {
  unsafe { gl::CreateShader(kind.into()) }
}

pub fn set_shader_source(id: Id, source: &str) -> Result<(), Error> {
  let source = CString::new(source).map_err(|e| Error::CanNotConvertToCString(e.to_string()))?;
  unsafe { gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null()) };
  Ok(())
}

pub fn compile_shader(id: Id) -> () {
  unsafe { gl::CompileShader(id) };
}

pub fn check_shader_compilation(id: Id) -> Result<(), Error> {
  let mut success: GLint = 1;
  unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };
  if success != 0 {
    return Ok(());
  }

  let mut len: GLint = 0;
  unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) };
  let error = create_whitespace_cstring_with_len(len as usize);
  unsafe { gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar )};

  Err(Error::ShaderCompilation(error.to_string_lossy().into_owned()))
}

pub fn create_program() -> Id {
  unsafe { gl::CreateProgram() }
}

pub fn attach_shader(program_id: Id, shader_id: Id) -> () {
  unsafe { gl::AttachShader(program_id, shader_id) };
}

pub fn detach_shader(program_id: Id, shader_id: Id) -> () {
  unsafe { gl::DetachShader(program_id, shader_id) };
}

pub fn link_program(program_id: Id) -> () {
  unsafe { gl::LinkProgram(program_id) };
}

pub fn check_program_linking(id: Id) -> Result<(), Error> {
  let mut success: GLint = 1;
  unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) };
  if success != 0 {
    return Ok(());
  }

  let mut len: GLint = 0;
  unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) };
  let error = create_whitespace_cstring_with_len(len as usize);
  unsafe { gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar )};

  Err(Error::ProgramLinkage(error.to_string_lossy().into_owned()))
}

pub fn use_program(id: Id) -> () {
  unsafe { gl::UseProgram(id) };
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
  // allocate buffer of correct size
  let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
  // fill it with len spaces
  buffer.extend([b' '].iter().cycle().take(len));
  // convert buffer to CString
  unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn gen_buffer() -> Id {
  let mut bo: GLuint = 0;
  unsafe { gl::GenBuffers(1, &mut bo) };
  bo
}

pub enum BufferType {
  Array,
  ElementArray,
}

impl From<BufferType> for GLenum {
  fn from(value: BufferType) -> Self {
    match value {
      BufferType::Array => gl::ARRAY_BUFFER,
      BufferType::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
    }
  }
}

pub fn bind_buffer(buffer_type: BufferType, id: Id) -> () {
  unsafe { gl::BindBuffer(buffer_type.into(), id) };
}

pub enum DrawType {
  Static,
}

impl From<DrawType> for GLenum {
  fn from(value: DrawType) -> Self {
    match value {
      DrawType::Static => gl::STATIC_DRAW,
    }
  }
}

pub fn set_buffer_data<Data>(buffer_type: BufferType, buffer: &[Data], draw_type: DrawType) -> () {
  unsafe {
    gl::BufferData(
      buffer_type.into(),
      (buffer.len() * std::mem::size_of::<Data>()) as GLsizeiptr,
      buffer.as_ptr() as *const GLvoid,
      draw_type.into()
    )
  };
}

pub fn gen_vertex_array() -> Id {
  let mut vao: Id = 0;
  unsafe { gl::GenVertexArrays(1, &mut vao) };
  vao
}

pub fn bind_vertex_array(vao: Id) -> () {
  unsafe { gl::BindVertexArray(vao) };
}

pub enum AttributeType {
  Float,
  UnsignedInt,
}

impl From<AttributeType> for GLenum {
  fn from(value: AttributeType) -> Self {
    match value {
      AttributeType::Float => gl::FLOAT,
      AttributeType::UnsignedInt => gl::UNSIGNED_INT,
    }
  }
}

pub fn set_vertex_attrib_pointer(
  index: u32,
  components_per_attribute: i32,
  attribute_type: AttributeType,
  normalise: bool,
  stride: i32,
  offset: u32)
  -> ()
{
  unsafe {
    gl::VertexAttribPointer(
      index,
      components_per_attribute,
      attribute_type.into(),
      normalise as u8,
      stride,
      offset as *const std::os::raw::c_void
    )
  };
}

pub fn enable_vertex_attrib_array(index: u32) -> () {
  unsafe { gl::EnableVertexAttribArray(index) };
}

pub enum DrawMode {
  Triangles,
}

impl From<DrawMode> for GLenum {
  fn from(value: DrawMode) -> Self {
    match value {
      DrawMode::Triangles => gl::TRIANGLES,
    }
  }
}

pub fn draw_arrays(mode: DrawMode, start: i32, num_indices: i32) -> () {
  unsafe { gl::DrawArrays(mode.into(), start, num_indices) };
}

pub fn draw_elements(mode: DrawMode, num_indices: i32, attribute_type: AttributeType, offset: i32) -> () {
  unsafe { gl::DrawElements(mode.into(), num_indices, attribute_type.into(), offset as *const std::os::raw::c_void) };
}

pub fn get_uniform_location(shader_id: Id, name: &str) -> Result<Id, Error> {
  let name = CString::new(name).map_err(|e| Error::CanNotConvertToCString(e.to_string()))?;
  let res: i32 = unsafe { gl::GetUniformLocation(shader_id, name.as_ptr()) };
  if res < 0 {
    return Err(Error::UnknownUniformName(name.to_string_lossy().into_owned()));
  }
  Ok(res as Id)
}

pub fn set_uniform1f(uniform_id: Id, v1: f32) -> () {
  unsafe { gl::Uniform1f(uniform_id as i32, v1) };
}

pub fn set_uniform4f(uniform_id: Id, v1: f32, v2: f32, v3: f32, v4: f32) -> () {
  unsafe { gl::Uniform4f(uniform_id as i32, v1, v2, v3, v4) };
}