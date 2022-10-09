extern crate gl;
use gl::types::*;
use std::ffi::CString;

pub type Id = GLuint;

pub fn create() -> Id {
  unsafe { gl::CreateProgram() }
}

pub fn delete(id: Id) -> () {
  unsafe { gl::DeleteProgram(id) };
}

pub fn attach(program_id: Id, shader_id: Id) -> () {
  unsafe { gl::AttachShader(program_id, shader_id) };
}

pub fn detach(program_id: Id, shader_id: Id) -> () {
  unsafe { gl::DetachShader(program_id, shader_id) };
}

pub fn link(id: Id) -> () {
  unsafe { gl::LinkProgram(id) };
}

pub fn is_flagged_for_delete(id: Id) -> bool {
  query(id, gl::DELETE_STATUS) != 0
}

pub fn was_link_successful(id: Id) -> bool {
  query(id, gl::LINK_STATUS) != 0
}

pub fn was_validation_successful(id: Id) -> bool {
  query(id, gl::VALIDATE_STATUS) != 0
}

pub fn info_log_length(id: Id) -> usize {
  query(id, gl::INFO_LOG_LENGTH) as usize
}

pub fn attached_shaders(id: Id) -> usize {
  query(id, gl::ATTACHED_SHADERS) as usize
}

pub fn active_atomic_counter_buffers(id: Id) -> usize {
  query(id, gl::ACTIVE_ATOMIC_COUNTER_BUFFERS) as usize
}

pub fn active_attributes(id: Id) -> usize {
  query(id, gl::ACTIVE_ATTRIBUTES) as usize
}

pub fn max_active_attribute_name_length(id: Id) -> usize {
  query(id, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH) as usize
}

pub fn active_uniforms(id: Id) -> usize {
  query(id, gl::ACTIVE_UNIFORMS) as usize
}

pub fn max_active_uniform_name_length(id: Id) -> usize {
  query(id, gl::ACTIVE_UNIFORM_MAX_LENGTH) as usize
}

pub fn binary_length(id: Id) -> usize {
  query(id, gl::PROGRAM_BINARY_LENGTH) as usize
}

pub fn max_output_vertices(id: Id) -> usize {
  query(id, gl::GEOMETRY_VERTICES_OUT) as usize
}

pub fn get_info_log(id: Id, len: usize) -> String {
  let space = std::iter::repeat(' ').take(len).collect::<String>();
  let info_log = CString::new(space).unwrap();
  unsafe { gl::GetProgramInfoLog(id, len as i32, std::ptr::null_mut(), info_log.as_ptr() as *mut GLchar) };
  info_log.to_string_lossy().into_owned()
}

pub fn enable(id: Id) -> () {
  unsafe { gl::UseProgram(id) };
}

fn query(id: Id, property: GLenum) -> GLint {
  let mut res: GLint = 0;
  unsafe { gl::GetProgramiv(id, property, &mut res) };
  res
}