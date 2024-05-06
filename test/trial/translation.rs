pub struct Matrix {
    src: [[f32; 4]; 4]
}

impl Matrix {
    pub fn new(src: [[f32; 4]; 4]) -> Self {
        Self { src }
    }

    pub fn new_default() -> Self {
        Self { src: [
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.]
        ] }
    }

    pub fn make_identity_matrix() -> Self {
        Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.]
        ])
    }
}