use crate::define::Vec3d;

pub struct Face {
    src: Vec<u32>,
}

impl Face {
    #[allow(dead_code)]
    pub fn convert_to_vec3d(&self) -> Vec<Vec3d> {
        Vec::<Vec3d>::new()
    }
}
