use std::fs;

use anyhow::{anyhow, Result};

use self::{face::Face, vertex::Vertex};

mod face;
mod vertex;

enum ObjectContents {
    MTLLIB(String),
    NAME(String),
    VERTEX(f32, f32, f32),
    USEMTL(String),
    SHADER(bool),
    FACE(Vec<usize>),
}

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
    let contents = fs::read_to_string(path)?;
    let contents_without_comments: Vec<String> = contents
        .lines()
        .map(|line| match line.find('#') {
            Some(index) => String::from(&line[0..index]),
            None => String::from(line),
        })
        .collect();

    let contents_vectored: Vec<Vec<&str>> = contents_without_comments
        .iter()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .filter(|line| !line.is_empty())
        .collect();

    // for elem in elements {
    //     for e in elem {
    //         println!("{e}");
    //     }
    // }

    println!("{:?}", contents_vectored);
    Ok(Object::new_default())
}