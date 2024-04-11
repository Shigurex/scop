#[allow(dead_code)]
pub struct Face {
    vertices: Vec<usize>,
    material: String,
    smooth_shading: bool,
}

impl Face {
    #[allow(dead_code)]
    pub fn new_default() -> Self {
        Self {
            vertices: Vec::new(),
            material: String::from("face"),
            smooth_shading: false,
        }
    }
}
