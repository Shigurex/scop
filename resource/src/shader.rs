use std::{ffi::CString, fs::File, io::Read};

use anyhow::Result;
use gl::types::GLuint;
use std::ptr;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self> {
        let mut shader = Self { id: 0 };

        let mut vertex_file = File::open(vertex_path).map_err(anyhow::Error::msg)?;
        let mut fragment_file = File::open(fragment_path).map_err(anyhow::Error::msg)?;

        let mut vertex_source = String::new();
        let mut fragment_source = String::new();

        let vertex_source_cstr =
            CString::new(vertex_source.as_bytes()).map_err(anyhow::Error::msg)?;
        let fragment_source_cstr =
            CString::new(fragment_source.as_bytes()).map_err(anyhow::Error::msg)?;

        vertex_file
            .read_to_string(&mut vertex_source)
            .map_err(anyhow::Error::msg)?;
        fragment_file
            .read_to_string(&mut fragment_source)
            .map_err(anyhow::Error::msg)?;

        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_source_cstr.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            // shader.check_compile_errors(vertex_shader, "vertex");

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_source_cstr.as_ptr(), ptr::null());
            gl::CompileShader(fragment);

            shader.id = gl::CreateProgram();
            gl::AttachShader(shader.id, vertex);
            gl::AttachShader(shader.id, fragment);
            gl::LinkProgram(shader.id);

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }

        Ok(shader)
    }

    pub unsafe fn use_shader(&self) {
        gl::UseProgram(self.id)
    }
}
