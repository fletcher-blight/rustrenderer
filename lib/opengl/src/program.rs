extern crate gl;
use gl::types::*;

use crate::shader;

#[derive(Debug)]
pub enum Error {
  FailedToLink,
}

pub struct Context {
  id: GLuint,
}

impl Context {
  pub fn from_shaders(shaders: &[shader::Context]) -> Result<Context, Error>{
    return Err(Error::FailedToLink);
  }
}