use std::ffi::c_void;

use anyhow::Result;
use gl::types::{GLenum, GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};

pub struct BufferObject {
    id: GLuint,
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl BufferObject {
    pub fn new<T>(target: GLenum, buffer: Vec<T>) -> Self {
        let mut buffer_object = Self { id: 0 };
        unsafe {
            gl::GenBuffers(1, &mut buffer_object.id);
            gl::BindBuffer(target, buffer_object.id);
            gl::BufferData(
                target,
                (buffer.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                buffer.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
        buffer_object
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

pub struct ArrayObject {
    id: GLuint,
}

impl Drop for ArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl ArrayObject {
    pub fn new() -> Self {
        let mut array_object = Self { id: 0 };
        unsafe {
            gl::GenVertexArrays(1, &mut array_object.id);
            gl::BindVertexArray(array_object.id);
        }
        array_object
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

pub struct Model {
    vao: ArrayObject,
    vbo: BufferObject,
    ebo: BufferObject,
}

impl Model {
    pub fn new() -> Result<Self> {
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

        let model = Self {
            vao: ArrayObject::new(),
            vbo: BufferObject::new(gl::ARRAY_BUFFER, vertices.to_vec()),
            ebo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, indices.to_vec()),
        };

        unsafe {
            // layout (location = 0) in vec3 Pos
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::TRUE,
                3 * std::mem::size_of::<GLfloat>() as GLsizei,
                std::ptr::null::<c_void>(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Ok(model)
    }

    pub fn vao(&self) -> GLuint {
        self.vao.id()
    }
}
