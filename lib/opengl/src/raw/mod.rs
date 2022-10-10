pub mod error;
pub mod shader;
pub mod program;
pub mod vertex_array;
pub mod buffer;
pub mod uniform;
pub mod draw;
pub mod texture;

pub fn load_with<Loader>(loader: Loader)
  where Loader: Fn(&str) -> *const() {
  gl::load_with(|s| loader(s) as *const std::os::raw::c_void);
}