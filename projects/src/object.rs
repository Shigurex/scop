use anyhow::{anyhow, Result};

use self::{face::Face, vertex::Vertex};

mod face;
mod vertex;
pub struct Object {
    name: String,
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
}

impl Object {
    pub fn new(name: String, vertices: Vec<Vertex>, faces: Vec<Face>) -> Self {
        Self {
            name,
            vertices,
            faces,
        }
    }

    pub fn new_default() -> Self {
        Self {
            name: "42.obj".to_string(),
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn parse_obj(path: &str) -> Result<Object> {
    Ok(Object::new_default())
}
