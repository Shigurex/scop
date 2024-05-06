#[derive(Clone, Copy)]
pub struct Vec3f {
    src: [f32; 3],
}

impl Vec3f {
    #[allow(dead_code)]
    pub fn new(src: [f32; 3]) -> Self {
        Self { src }
    }

    #[allow(dead_code)]
    pub fn new_default() -> Self {
        Self { src: [0., 0., 0.] }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3d {
    src: [u32; 3],
}

impl Vec3d {
    #[allow(dead_code)]
    pub fn new(src: [u32; 3]) -> Self {
        Self { src }
    }

    #[allow(dead_code)]
    pub fn new_default() -> Self {
        Self { src: [0, 0, 0] }
    }
}

#[derive(Clone, Copy)]
pub struct Mat4f {
    src: [[f32; 4]; 4],
}

impl Mat4f {
    #[allow(dead_code)]
    pub fn new(src: [[f32; 4]; 4]) -> Self {
        Self { src }
    }
}
