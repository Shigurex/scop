pub struct Face {
    vertices: Vec<usize>,
    material: String,
    smooth_shading: bool,
}

impl Face {
    pub fn new_default() -> Self {
        Self {
            vertices: Vec::new(),
            material: "face".to_string(),
            smooth_shading: false,
        }
    }
}
