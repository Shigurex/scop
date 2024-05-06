use gl::types::{GLenum, GLsizeiptr, GLuint, GLvoid};

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
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut array_object = Self { id: 0 };
        unsafe {
            gl::GenVertexArrays(1, &mut array_object.id);
            gl::BindVertexArray(array_object.id);
        }
        array_object
    }

    #[allow(dead_code)]
    pub fn id(&self) -> GLuint {
        self.id
    }
}

#[allow(dead_code)]
pub struct Model {
    vao: ArrayObject,
    vbo: BufferObject,
    ebo: BufferObject,
    cbo: BufferObject,
}

impl Model {
    #[allow(dead_code)]
    pub fn new(vertex_buffer: Vec<u32>, element_buffer: Vec<u32>, color_buffer: Vec<u32>) -> Self {
        Self {
            vao: ArrayObject::new(),
            vbo: BufferObject::new(gl::ARRAY_BUFFER, vertex_buffer),
            ebo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, element_buffer),
            cbo: BufferObject::new(gl::ARRAY_BUFFER, color_buffer),
        }
    }

    #[allow(dead_code)]
    pub fn new_default() -> Self {
        Self {
            vao: ArrayObject::new(),
            vbo: BufferObject::new(gl::ARRAY_BUFFER, Vec::<u32>::new()),
            ebo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, Vec::<u32>::new()),
            cbo: BufferObject::new(gl::ARRAY_BUFFER, Vec::<u32>::new()),
        }
    }
}
