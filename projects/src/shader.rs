use std::{ffi::CString, fs::File, io::Read, ptr};

use anyhow::Result;
use gl::types::{self, GLuint};

#[allow(dead_code)]
pub struct Shader {
    id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    #[allow(dead_code)]
    pub unsafe fn new(path: &str, shader_type: types::GLenum) -> Result<Self> {
        let shader = Self {
            id: gl::CreateShader(shader_type),
        };
        let mut file = File::open(path).map_err(anyhow::Error::msg)?;
        let mut source = String::new();
        file.read_to_string(&mut source)
            .map_err(anyhow::Error::msg)?;
        let source_cstr = CString::new(source.as_bytes()).map_err(anyhow::Error::msg)?;
        gl::ShaderSource(shader.id, 1, &source_cstr.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);
        // TODO: check compile errors here
        Ok(shader)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    #[allow(dead_code)]
    pub unsafe fn new(shaders: Vec<Shader>) -> Result<Self> {
        let shader_program = Self {
            id: gl::CreateProgram(),
        };
        for shader in shaders {
            gl::AttachShader(shader_program.id, shader.id());
        }
        gl::LinkProgram(shader_program.id);
        // TODO: check compile errors here
        Ok(shader_program)
    }

    #[allow(dead_code)]
    pub unsafe fn use_shader_program(&self) {
        gl::UseProgram(self.id)
    }
}

pub fn make_shader_program(vertex_path: &str, fragement_path: &str) -> Result<ShaderProgram> {
    unsafe {
        let vertex_shader = Shader::new(vertex_path, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(fragement_path, gl::FRAGMENT_SHADER)?;
        let shader_program = ShaderProgram::new(vec![vertex_shader, fragment_shader])?;
        Ok(shader_program)
    }
}
