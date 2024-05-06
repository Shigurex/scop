use crate::define::Vec3f;

pub struct Vertex {
    coordinate: Vec3f,
    color: Vec3f
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new(coordinate: Vec3f, color: Vec3f) -> Self {
        Self {
            coordinate,
            color
        }
    }

    #[allow(dead_code)]
    pub fn new_default() -> Self {
        Self {
            coordinate: Vec3f::new_default(),
            color: Vec3f::new_default(),
        }
    }

    pub fn coordinate(&self) -> Vec3f {
        self.coordinate
    }

    pub fn color(&self) -> Vec3f {
        self.color
    }
}
