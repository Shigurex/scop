use std::env;

#[allow(dead_code)]
pub struct Setting {
    obj_path: String,
    vertex_path: String,
    fragement_path: String,
}

impl Setting {
    #[allow(dead_code)]
    pub fn new(obj_path: &str, vertex_path: &str, fragement_path: &str) -> Self {
        Self {
            obj_path: String::from(obj_path),
            vertex_path: String::from(vertex_path),
            fragement_path: String::from(fragement_path),
        }
    }

    #[allow(dead_code)]
    pub fn new_default() -> Self {
        Self {
            obj_path: String::from("./objects/42.obj"),
            vertex_path: String::from("./projects/shaders/shader.vert"),
            fragement_path: String::from("./projects/shaders/shader.frag"),
        }
    }

    #[allow(dead_code)]
    pub fn obj_path(&self) -> String {
        self.obj_path.clone()
    }

    pub fn vertex_path(&self) -> String {
        self.vertex_path.clone()
    }

    pub fn fragement_path(&self) -> String {
        self.fragement_path.clone()
    }
}

#[allow(dead_code)]
pub fn parse_args(_args: Vec<String>) -> Setting {
    Setting::new_default()
}

#[allow(dead_code)]
pub fn get_args() -> Vec<String> {
    env::args().collect()
}
