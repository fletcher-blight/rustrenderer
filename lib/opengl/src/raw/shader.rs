extern crate gl;
use gl::types::*;
use std::ffi::{ CString, NulError };

pub type Id = GLuint;

pub enum Type {
  Compute,
  Vertex,
  TessellationControl,
  TessellationEvaluation,
  Geometry,
  Fragment,
}

pub enum Error {
  UnknownType(i32)
}

pub fn create(value: Type) -> Id {
  unsafe { gl::CreateShader(value.into()) }
}

pub fn delete(id: Id) -> () {
  unsafe { gl::DeleteShader(id) };
}

pub fn compile(id: Id) -> () {
  unsafe { gl::CompileShader(id) };
}

pub fn get_type(id: Id) -> Result<Type, Error> {
  let mut param: GLint = 0;
  unsafe { gl::GetShaderiv(id, gl::SHADER_TYPE, &mut param) };
  match param as GLenum {
    gl::COMPUTE_SHADER => Ok(Type::Compute),
    gl::VERTEX_SHADER => Ok(Type::Vertex),
    gl::TESS_CONTROL_SHADER => Ok(Type::TessellationControl),
    gl::TESS_EVALUATION_SHADER => Ok(Type::TessellationEvaluation),
    gl::GEOMETRY_SHADER => Ok(Type::Geometry),
    gl::FRAGMENT_SHADER => Ok(Type::Fragment),
    unknown => Err(Error::UnknownType(unknown as i32)),
  }
}

pub fn is_flagged_for_delete(id: Id) -> bool {
  query(id, gl::DELETE_STATUS) != 0
}

pub fn was_compile_successful(id: Id) -> bool {
  query(id, gl::COMPILE_STATUS) != 0
}

pub fn info_log_length(id: Id) -> usize {
  query(id, gl::INFO_LOG_LENGTH) as usize
}

pub fn source_length(id: Id) -> usize {
  query(id, gl::SHADER_SOURCE_LENGTH) as usize
}

pub fn get_info_log(id: Id, len: usize) -> String {
  let space = std::iter::repeat(' ').take(len).collect::<String>();
  let info_log = CString::new(space).unwrap();
  unsafe { gl::GetShaderInfoLog(id, len as i32, std::ptr::null_mut(), info_log.as_ptr() as *mut GLchar) };
  info_log.to_string_lossy().into_owned()
}

pub fn set_source(id: Id, source: &str) -> Result<(), NulError> {
  let source = CString::new(source)?;
  unsafe { gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null()) };
  Ok(())
}

fn query(id: Id, property: GLenum) -> GLint {
  let mut res: GLint = 0;
  unsafe { gl::GetShaderiv(id, property, &mut res) };
  res
}

impl From<Type> for GLenum {
  fn from(value: Type) -> Self {
    match value {
      Type::Compute => gl::COMPUTE_SHADER,
      Type::Vertex => gl::VERTEX_SHADER,
      Type::TessellationControl => gl::TESS_CONTROL_SHADER,
      Type::TessellationEvaluation => gl::TESS_EVALUATION_SHADER,
      Type::Geometry => gl::GEOMETRY_SHADER,
      Type::Fragment => gl::FRAGMENT_SHADER,
    }
  }
}