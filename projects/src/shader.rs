use std::ffi::CString;

use gl::types::GLuint;

pub struct Shader {
    id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

impl Shader {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, String> {
        let shader = Self { id: 0 };
        Ok(shader)
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
    fn new() -> Result<Self, String> {
        let program = Self { id: 0 };
        Ok(program)
    }

    //  name must correspond to the uniform variable name in GLSL
    #[allow(dead_code)]
    unsafe fn set_uniform_location(&self, name: &str) -> Result<i32, String> {
        let name_cstr =
            CString::new(name).map_err(|_| String::from("failed to convert &str to cstr"))?;
        Ok(gl::GetUniformLocation(self.id, name_cstr.as_ptr()))
    }

    #[allow(dead_code)]
    fn set_uniform_4f(&self, name: &str) -> Result<(), String> {
        unsafe {
            let location = self.set_uniform_location(name)?;
            gl::Uniform4f(location, 0., 0., 0., 0.);
        }
        Ok(())
    }
}

pub fn make_shader_program(
    _vertex_path: &str,
    _fragement_path: &str,
) -> Result<ShaderProgram, String> {
    let _vertex_shader = Shader::new()?;
    let _fragment_shader = Shader::new()?;
    let shader_program = ShaderProgram::new()?;
    Ok(shader_program)
}
