use anyhow::Result;
use gl::types::{self, GLuint};
use std::ptr;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self> {
        let mut shader = Self { id: 0 };
        Ok(shader)
    }

    // fn read_file_cstr(file_path: &str) -> Result<CString> {
    //     let mut file = File::open(file_path).map_err(anyhow::Error::msg)?;
    //     let mut source = String::new();
    //     file.read_to_string(&mut source)
    //         .map_err(anyhow::Error::msg)?;
    //     let source_cstr = CString::new(source.as_bytes()).map_err(anyhow::Error::msg)?;
    //     Ok(source_cstr)
    // }

    // unsafe fn link_shader(&self, type_: types::GLenum, source_cstr: CString) {
    //     let shader = gl::CreateShader(type_);
    //     gl::ShaderSource(shader, 1, &source_cstr.as_ptr(), ptr::null());
    //     gl::CompileShader(shader);
    //     gl::AttachShader(self.id, shader);
    //     gl::DeleteShader(shader);
    //     // shader.check_compile_errors(vertex, "VERTEX");
    // }

    pub unsafe fn use_shader(&self) {
        gl::UseProgram(self.id)
    }
}
