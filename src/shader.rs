extern crate gl;
use gl::types::*;
use std::ffi::CString;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn enable(self: &Self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_float(self: &Self, name: &str, value: f32) -> Result<(), String> {
        let loc = find_uniform(self.id, name)?;
        unsafe {
            gl::Uniform1f(loc as i32, value);
        }
        Ok(())
    }

    pub fn set_vec3(self: &Self, name: &str, vec: &nalgebra_glm::Vec3) -> Result<(), String> {
        let loc = find_uniform(self.id, name)?;
        unsafe {
            gl::Uniform3f(loc as i32, *vec.index(0), *vec.index(1), *vec.index(2));
        }
        Ok(())
    }

    pub fn set_mat4(self: &Self, name: &str, mat: &nalgebra_glm::Mat4) -> Result<(), String> {
        let loc = find_uniform(self.id, name)?;
        unsafe {
            gl::UniformMatrix4fv(
                loc as i32,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&mat).as_ptr(),
            );
        }
        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub fn compile_from_sources(vertex_source: &str, fragment_source: &str) -> Result<Shader, String> {
    program_id_from_shaders(&vertex_source, &fragment_source)
}

fn find_uniform(program_id: GLuint, name: &str) -> Result<GLuint, String> {
    let cstr = CString::new(name).map_err(error_to_string())?;
    let id = unsafe { gl::GetUniformLocation(program_id, cstr.as_ptr() as *const GLchar) };
    if id < 0 {
        return Err(format!("Could not find {}", name));
    }
    Ok(id as GLuint)
}

struct ShaderComponent {
    id: GLuint,
}

impl Drop for ShaderComponent {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

struct ShaderLink {
    program_id: GLuint,
    component_id: GLuint,
}

impl ShaderLink {
    fn create(program_id: GLuint, component_id: GLuint) -> Self {
        unsafe {
            gl::AttachShader(program_id, component_id);
        }
        ShaderLink {
            program_id,
            component_id,
        }
    }
}

impl Drop for ShaderLink {
    fn drop(&mut self) {
        unsafe {
            gl::DetachShader(self.program_id, self.component_id);
        }
    }
}

fn compile_shader(source: &str, kind: GLuint) -> Result<ShaderComponent, String> {
    let cstr = CString::new(source).map_err(error_to_string())?;

    let shader_component = unsafe {
        let id = gl::CreateShader(kind);
        gl::ShaderSource(id, 1, &cstr.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        ShaderComponent { id }
    };

    let mut success: GLint = 0;
    unsafe {
        gl::GetShaderiv(shader_component.id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        return Err(format!("Failed to compile {}", source));
    }

    Ok(shader_component)
}

fn program_id_from_shaders(vertex_source: &str, fragment_source: &str) -> Result<Shader, String> {
    let shader = Shader {
        id: unsafe { gl::CreateProgram() },
    };

    let vertex_component = compile_shader(&vertex_source, gl::VERTEX_SHADER)?;
    let fragment_component = compile_shader(&fragment_source, gl::FRAGMENT_SHADER)?;

    let _vertex_link = ShaderLink::create(shader.id, vertex_component.id);
    let _fragment_link = ShaderLink::create(shader.id, fragment_component.id);

    let mut success: GLint = 0;
    unsafe {
        gl::LinkProgram(shader.id);
        gl::GetProgramiv(shader.id, gl::LINK_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: GLint = 0;
        unsafe { gl::GetProgramiv(shader.id, gl::INFO_LOG_LENGTH, &mut len) };
        let error = CString::new(
            std::iter::repeat(' ')
                .take(len as usize)
                .collect::<String>(),
        )
        .map_err(error_to_string())?;
        unsafe {
            gl::GetProgramInfoLog(
                shader.id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            )
        };
        return Err(format!(
            "Failed to link program: {}",
            error.to_string_lossy().into_owned(),
        ));
    }

    Ok(shader)
}

fn error_to_string<E>() -> fn(E) -> String
where
    E: std::fmt::Display,
{
    |e: E| e.to_string()
}
