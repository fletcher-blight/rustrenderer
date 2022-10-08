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
  InvalidShaderSource(String),
  ShaderCompilation(String),
  ProgramLinkage(String),
  GL(GLError),
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

#[repr(u32)]
pub enum ClearBit {
  ColourBufferBit = gl::COLOR_BUFFER_BIT,
}

pub fn clear(bitfield: ClearBit) -> () {
  unsafe { gl::Clear(bitfield as GLenum) };
}

pub type Id = GLuint;

#[repr(u32)]
pub enum Shader {
  Vertex = gl::VERTEX_SHADER,
  Fragment = gl::FRAGMENT_SHADER,
}

pub fn create_shader(kind: Shader) -> Id {
  unsafe { gl::CreateShader(kind as GLuint) }
}

pub fn set_shader_source(id: Id, source: &str) -> Result<(), Error> {
  let source = CString::new(source).map_err(|ne| Error::InvalidShaderSource(format!("{}", ne)))?;
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