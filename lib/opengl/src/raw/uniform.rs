extern crate gl;
use gl::types::*;
use crate::raw::program;
use std::ffi::CString;

pub type Id = GLuint;

pub fn get_uniform_location(id: program::Id, name: &str) -> Result<Id, String> {
  let name = CString::new(name).map_err(|e| e.to_string())?;
  let res: i32 = unsafe { gl::GetUniformLocation(id, name.as_ptr()) };
  if res < 0 {
    let name = name.to_string_lossy().into_owned();
    return Err(format!("Cannot find uniform variable named: {}", name));
  }
  Ok(res as Id)
}

pub fn set_uniform1f(uniform_id: Id, v1: f32) -> () {
  unsafe { gl::Uniform1f(uniform_id as i32, v1) };
}

pub fn set_uniform4f(uniform_id: Id, v1: f32, v2: f32, v3: f32, v4: f32) -> () {
  unsafe { gl::Uniform4f(uniform_id as i32, v1, v2, v3, v4) };
}
