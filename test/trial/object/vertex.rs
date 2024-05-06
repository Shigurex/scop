#[allow(dead_code)]

type Coordinate = [f32; 3];
type Color = [f32; 3];

pub struct Vertex {
    coordinate: Coordinate,
    color: Color,
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new(coordinate: Coordinate, color: Color) -> Self {
        Self { coordinate, color }
    }

    #[allow(dead_code)]
    pub fn new_default() -> Self {
        Self {
            coordinate: [0., 0., 0.],
            color: [0., 0., 0.],
        }
    }

    pub fn concat(&self) -> [f32; 6] {
        let mut combined: [f32; 6] = [0.; 6];
        combined[0..3].copy_from_slice(&self.coordinate);
        combined[3..6].copy_from_slice(&self.color);
        combined
    }
}
