use std::{ffi::CString, fs::File, io::Read};

use anyhow::Result;
use gl::types::{GLchar, GLint};
use std::ptr;

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self> {
        let mut shader = Self { id: 0 };

        let mut vertex_file = File::open(vertex_path).map_err(anyhow::Error::msg)?;
        let mut fragment_file = File::open(fragment_path).map_err(anyhow::Error::msg)?;

        let mut vertex_source = String::new();
        let mut fragment_source = String::new();

        vertex_file
            .read_to_string(&mut vertex_source)
            .map_err(anyhow::Error::msg)?;
        fragment_file
            .read_to_string(&mut fragment_source)
            .map_err(anyhow::Error::msg)?;

        let vertex_source_cstr =
            CString::new(vertex_source.as_bytes()).map_err(anyhow::Error::msg)?;
        let fragment_source_cstr =
            CString::new(fragment_source.as_bytes()).map_err(anyhow::Error::msg)?;

        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_source_cstr.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            // shader.check_compile_errors(vertex, "VERTEX");

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_source_cstr.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            // shader.check_compile_errors(fragment, "FRAGMENT");

            shader.id = gl::CreateProgram();
            gl::AttachShader(shader.id, vertex);
            gl::AttachShader(shader.id, fragment);
            gl::LinkProgram(shader.id);
            // shader.check_compile_errors(shader.id, "PROGRAM");

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }

        Ok(shader)
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
