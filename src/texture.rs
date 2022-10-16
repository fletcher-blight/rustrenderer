extern crate gl;
use gl::types::*;

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

pub fn create(data: &[u8]) -> Result<Texture, String> {
    let image = image::load_from_memory(data).map_err(error_to_string())?;

    let mut id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut id);
    }
    let texture = Texture { id };
    texture.bind();
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            image.width().try_into().map_err(error_to_string())?,
            image.height().try_into().map_err(error_to_string())?,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            image.as_bytes().as_ptr() as *const std::os::raw::c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    Ok(texture)
}

fn error_to_string<E>() -> fn(E) -> String
where
    E: std::fmt::Display,
{
    |e: E| e.to_string()
}
