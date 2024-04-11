use self::{face::Face, vertex::Vertex};
use crate::parser::Parser;
use anyhow::Result;

mod face;
mod vertex;

// enum ObjectContents {
//     MTLLIB(String),
//     NAME(String),
//     VERTEX(f32, f32, f32),
//     USEMTL(String),
//     SHADER(bool),
//     FACE(Vec<usize>),
// }

#[allow(dead_code)]
pub struct Object {
    name: String,
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
}

impl Parser<Self> for Object {
    fn format(contents: Vec<Vec<String>>) -> Result<Self> {
        let object = Self::new_default();
        for line in contents {
            if let Some(first) = line.first() {
                match first.as_str() {
                    "mtllib" => {}
                    "o" => {}
                    "v" => {}
                    "usemtl" => {}
                    "s" => {}
                    "f" => {}
                    _ => {}
                }
                println!("{first}");
            }
        }
        Ok(object)
    }
}

impl Object {
    #[allow(dead_code)]
    pub fn new(name: String, vertices: Vec<Vertex>, faces: Vec<Face>) -> Self {
        Self {
            name,
            vertices,
            faces,
        }
    }

    pub fn new_default() -> Self {
        Self {
            name: "scop".to_string(),
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn insert_vertex(&self) {
        // self.vertices.append();
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
