use std::ffi::c_void;

use gl::types::{GLsizeiptr, GLuint};

pub struct Vertex {
    vbo: GLuint,
    vao: GLuint,
}

impl Vertex {
    pub fn new() -> Self {
        let buffer_array: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];

        let mut vertex = Self { vbo: 0, vao: 0 };
        unsafe {
            gl::GenVertexArrays(1, &mut vertex.vao);
            gl::GenBuffers(1, &mut vertex.vbo);

            gl::BindVertexArray(vertex.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vertex.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (9 * 4) as GLsizeiptr,
                buffer_array.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        vertex
    }

    pub fn vao(&self) -> GLuint {
        self.vao
    }
}
