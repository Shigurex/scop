use crate::model::Model;

use self::{face::Face, vertex::Vertex};

mod face;
mod vertex;

pub struct Object {
    name: String,
    vertice: Vec<Vertex>,
    faces: Vec<Face>,
}

impl Object {
    pub fn new(name: String, vertice: Vec<Vertex>, faces: Vec<Face>) -> Self {
        Self {
            name,
            vertice,
            faces,
        }
    }

    pub fn new_default() -> Self {
        Self {
            name: String::from("scop"),
            vertice: Vec::<Vertex>::new(),
            faces: Vec::<Face>::new(),
        }
    }

    pub fn parse(_obj_path: String) -> Result<Self, String> {
        Ok(Self::new_default())
    }

    pub fn to_model(&self) -> Model {
        Model::new_default()
    }
}
