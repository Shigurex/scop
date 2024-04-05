pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn new_default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}
