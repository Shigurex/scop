use std::ffi::c_void;

use gl::types::{GLsizeiptr, GLuint};

pub struct Vertex {
    vbo: GLuint,
    vao: GLuint,
    ebo: GLuint,
}

impl Vertex {
    pub fn new() -> Self {
        let vertices: [f32; 12] = [
            0.5, 0.5, 0.0, // top right
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0, // bottom let
            -0.5, 0.5, 0.0, // top left
        ];

        let indices: [u32; 6] = [
            0, 1, 3, // first Triangle
            1, 2, 3, // second Triangle
        ];

        let mut vertex = Self {
            vbo: 0,
            vao: 0,
            ebo: 0,
        };
        unsafe {
            gl::GenVertexArrays(1, &mut vertex.vao);
            gl::GenBuffers(1, &mut vertex.vbo);
            gl::GenBuffers(1, &mut vertex.ebo);

            gl::BindVertexArray(vertex.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vertex.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (12 * 4) as GLsizeiptr,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vertex.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (6 * 4) as GLsizeiptr,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * 4,
                std::ptr::null::<c_void>(),
            );
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
