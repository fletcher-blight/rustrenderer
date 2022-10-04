
use gl::types::*;
use std::ffi::{ CStr, CString };

pub struct Shader {
  gl: gl::Gl,
  id: GLuint,
}

pub struct Program {
  gl: gl::Gl,
  id: GLuint,
}

impl Shader {
  pub fn from_source(gl: gl::Gl, source: &CStr, kind: GLenum) -> Result<Shader, String> {
    let id = shader_from_source(&gl, source, kind)?;
    Ok(Shader { gl, id })
  }

  pub fn from_vert_source(gl: gl::Gl, source: &CStr) -> Result<Shader, String> {
    Shader::from_source(gl, source, gl::VERTEX_SHADER)
  }

  pub fn from_frag_source(gl: gl::Gl, source: &CStr) -> Result<Shader, String> {
    Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
  }

  pub fn id(&self) -> GLuint {
    self.id
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteShader(self.id);
    }
  }
}

impl Program {
  pub fn from_shaders(gl: gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
    let id = program_from_shaders(&gl, shaders)?;
    Ok(Program { gl, id })
  }

  pub fn set_used(&self) {
    unsafe {
      self.gl.UseProgram(self.id);
    }
  }
}

impl Drop for Program {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteProgram(self.id);
    }
  }
}

fn shader_from_source(gl: &gl::Gl, source: &CStr, kind: GLenum) -> Result<GLuint, String> {
  let id = unsafe { gl.CreateShader(kind) };
  unsafe {
    gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
    gl.CompileShader(id);
  }

  let mut success: GLint = 1;
  unsafe {
    gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
  }

  if success == 0 {
    let mut len: GLint = 0;
    unsafe {
      gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }
    let error = create_whitespace_cstring_with_len(len as usize);
    unsafe {
      gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
    }
    return Err(error.to_string_lossy().into_owned());
  }

  Ok(id)
}

fn program_from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<GLuint, String> {
  let id = unsafe { gl.CreateProgram() };
  for shader in shaders {
    unsafe { gl.AttachShader(id, shader.id()); }
  }
  unsafe { gl.LinkProgram(id); }

  let mut success: GLint = 1;
  unsafe {
    gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
  }
  if success == 0 {
    let mut len: GLint = 0;
    unsafe {
      gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }
    let error = create_whitespace_cstring_with_len(len as usize);
    unsafe {
      gl.GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
    }
    return Err(error.to_string_lossy().into_owned());
  }

  for shader in shaders {
    unsafe { gl.DetachShader(id, shader.id()); }
  }

  Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
  let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
  buffer.extend([b' '].iter().cycle().take(len as usize));
  unsafe { CString::from_vec_unchecked(buffer) }
}