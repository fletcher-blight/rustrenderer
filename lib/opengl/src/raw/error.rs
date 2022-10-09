extern crate gl;

#[derive(Debug)]
pub enum Error {
  InvalidEnum,
  InvalidFrameBufferOperation,
  InvalidIndex,
  InvalidOperation,
  InvalidValue,
  Unknown(u32),
}

pub fn check() -> Result<(), Error> {
  match unsafe { gl::GetError() } {
    0 => Ok(()),
    gl::INVALID_ENUM => Err(Error::InvalidEnum),
    gl::INVALID_FRAMEBUFFER_OPERATION => Err(Error::InvalidFrameBufferOperation),
    gl::INVALID_INDEX => Err(Error::InvalidIndex),
    gl::INVALID_OPERATION => Err(Error::InvalidOperation),
    gl::INVALID_VALUE => Err(Error::InvalidValue),
    value => Err(Error::Unknown(value)),
  }
}