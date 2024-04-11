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
    smooth_shading: bool,
}

impl Parser<Self> for Object {
    fn format(contents: Vec<Vec<String>>) -> Result<Self> {
        let mut object = Self::new_default();
        for line in contents {
            let mut line_itr = line.iter();
            if let Some(first) = line_itr.next() {
                match first.as_str() {
                    "mtllib" => {
                        match line_itr.next() {
                            Some(_path) => {},
                            None => {}
                        }
                    },
                    "o" => {
                        match line_itr.next() {
                            Some(name) => object.name = name.clone(),
                            None => {}
                        }
                    },
                    "v" => {},
                    "usemtl" => {
                        match line_itr.next() {
                            Some(_name) => {},
                            None => {}
                        }
                    },
                    "s" => {
                        match line_itr.next() {
                            Some(status) => {
                                match status.as_str() {
                                    "on" => object.smooth_shading = true,
                                    "off" => object.smooth_shading = false,
                                    _ => {}
                                }
                            },
                            None => {}
                        }
                    },
                    "f" => {},
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
    pub fn new(name: String, vertices: Vec<Vertex>, faces: Vec<Face>, smooth_shading: bool) -> Self {
        Self {
            name,
            vertices,
            faces,
            smooth_shading,
        }
    }

    pub fn new_default() -> Self {
        Self {
            name: "scop".to_string(),
            vertices: Vec::new(),
            faces: Vec::new(),
            smooth_shading: false,
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
