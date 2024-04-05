use std::{ffi::CString, fs::File, io::Read};

use anyhow::Result;
use gl::types::{self, GLuint};
use std::ptr;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self> {
        let mut shader = Self { id: 0 };

        let vertex_source_cstr = Shader::read_file_cstr(vertex_path)?;
        let fragment_source_cstr = Shader::read_file_cstr(fragment_path)?;

        unsafe {
            shader.id = gl::CreateProgram();
            shader.link_shader(gl::VERTEX_SHADER, vertex_source_cstr);
            shader.link_shader(gl::FRAGMENT_SHADER, fragment_source_cstr);
            gl::LinkProgram(shader.id);
            // shader.check_compile_errors(shader.id, "PROGRAM");
        }

        Ok(shader)
    }

    fn read_file_cstr(file_path: &str) -> Result<CString> {
        let mut file = File::open(file_path).map_err(anyhow::Error::msg)?;
        let mut source = String::new();
        file.read_to_string(&mut source)
            .map_err(anyhow::Error::msg)?;
        let source_cstr = CString::new(source.as_bytes()).map_err(anyhow::Error::msg)?;
        Ok(source_cstr)
    }

    unsafe fn link_shader(&self, type_: types::GLenum, source_cstr: CString) {
        let shader = gl::CreateShader(type_);
        gl::ShaderSource(shader, 1, &source_cstr.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        gl::AttachShader(self.id, shader);
        gl::DeleteShader(shader);
        // shader.check_compile_errors(vertex, "VERTEX");
    }

    pub unsafe fn use_shader(&self) {
        gl::UseProgram(self.id)
    }

    // ERROR: NEED TO MODIFY CODE
    // unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
    //     let mut success = gl::FALSE as GLint;
    //     let mut info_log = Vec::with_capacity(1024);
    //     info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
    //     if type_ != "PROGRAM" {
    //         gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    //         if success != gl::TRUE as GLint {
    //             gl::GetShaderInfoLog(
    //                 shader,
    //                 1024,
    //                 ptr::null_mut(),
    //                 info_log.as_mut_ptr() as *mut GLchar,
    //             );
    //             let info_log_string = match String::from_utf8(info_log) {
    //                 Ok(log) => log,
    //                 Err(vec) => panic!("failed to convert to compilation log from buffer: {}", vec),
    //             };
    //             println!(
    //                 "failed to compile shader code: type={}, log={}",
    //                 type_, info_log_string
    //             );
    //         }
    //     } else {
    //         gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
    //         if success != gl::TRUE as GLint {
    //             gl::GetProgramInfoLog(
    //                 shader,
    //                 1024,
    //                 ptr::null_mut(),
    //                 info_log.as_mut_ptr() as *mut GLchar,
    //             );
    //             let info_log_string = match String::from_utf8(info_log) {
    //                 Ok(log) => log,
    //                 Err(vec) => panic!("failed to convert to link log from buffer: {}", vec),
    //             };
    //             println!(
    //                 "failed to link shader code: type={}, log={}",
    //                 type_, info_log_string
    //             );
    //         }
    //     }
    // }
}
