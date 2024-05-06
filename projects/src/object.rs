use self::{face::Face, vertex::Vertex};

mod vertex;
mod face;

pub struct Object {
    name: String,
    vertice: Vec<Vertex>,
    faces: Vec<Face>
}

impl Object {
    pub fn new(name: String, vertice: Vec<Vertex>, faces: Vec<Face>) -> Self {
        Self {
            name,
            vertice,
            faces
        }
    }

    pub fn new_default() -> Self {
        Self {
            name: String::from("scop"),
            vertice: Vec::<Vertex>::new(),
            faces: Vec::<Face>::new()
        }
    }
}

pub fn parse_object(_obj_path: String) -> Result<Object, String> {
    Ok(Object::new_default())
}
