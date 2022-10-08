extern crate gl;
use gl::types::*;
use std::ffi::{ CStr, CString };

#[derive(Debug)]
pub enum Error {
  FailedToCompile(String),
}

#[derive(Debug)]
pub enum Kind {
  Vertex,
  Fragment,
}

pub struct Context {
  id: GLuint,
}

impl Context {
  pub fn from_source(source: &CStr, kind: Kind) -> Result<Context, Error> {
    let id = unsafe { compile_source(source, kind.into()) } ?;
    Ok(Context { id })
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { gl::DeleteShader(self.id) };
  }
}

impl From<Kind> for GLenum {
  fn from(other: Kind) -> Self {
    match other {
      Kind::Vertex => gl::VERTEX_SHADER,
      Kind::Fragment => gl::FRAGMENT_SHADER,
    }
  }
}

unsafe fn compile_source(source: &CStr, kind: GLenum) -> Result<GLuint, Error> {
  let id = gl::CreateShader(kind);
  gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
  gl::CompileShader(id);

  let mut success: GLint = 1;
  gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

  if success == 0 {
    let mut len: GLint = 0;
    gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

    let error = create_whitespace_cstring_with_len(len as usize);
    gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
    return Err(Error::FailedToCompile(error.to_string_lossy().into_owned()));
  }

  Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
  let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
  buffer.extend([b' '].iter().cycle().take(len as usize));
  unsafe { CString::from_vec_unchecked(buffer) }
}