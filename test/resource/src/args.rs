use anyhow::{anyhow, Result};

use std::env;

pub struct Settings {
    vertex_path: String,
    fragment_path: String,
    obj_path: String
}

impl Settings {
    pub fn new(vertex_path: &str, fragment_path: &str, obj_path: &str) -> Self {
        Settings {
            vertex_path: vertex_path.to_string(),
            fragment_path: fragment_path.to_string(),
            obj_path: obj_path.to_string(),
        }
    }

    pub fn new_default() -> Self {
        Settings {
            vertex_path: "./resource/shaders/shader.vert".to_string(),
            fragment_path: "./resource/shaders/shader.frag".to_string(),
            obj_path: "./objects/42.obj".to_string(),
        }
    }

    // fn vertex_path(&self) -> String {
    //     self.vertex_path
    // }

    // fn fragment_path(&self) -> String {
    //     self.fragment_path
    // }

    // fn obj_path(&self) -> String {
    //     self.obj_path
    // }

    fn set_vertex_path(&self, vertex_path: &str) {
        self.vertex_path = vertex_path.to_string();
    }

    fn set_fragment_path(&self, fragment_path: &str) {
        self.fragment_path = fragment_path.to_string();
    }

    fn set_obj_path(&self, obj_path: &str) {
        self.obj_path = obj_path.to_string();
    }
}

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn parse_args(args: Vec<String>) -> Result<Settings> {
    let len_args: usize = args.len();
    let settings: Settings = Settings::new_default();

    if len_args == 1 {
        println!(
            "usage: {} (file) [-v vertex_path] [-f fragment_path]",
            args[0]
        );
        return Ok(settings);
    }

    let mut i = 1;
    while i < len_args {
        let arg = args[i].as_str();
        match arg {
            "-v" | "--vertex" => {
                i += 1;
                if i == len_args {
                    return Err(anyhow!(
                        "Need a path to the vertex shader."
                    ));
                }
                settings.set_vertex_path(args[i].as_str())
            }
            "-f" | "--fragment" => {
                i += 1;
                if i == len_args {
                    return Err(anyhow!(
                        "Need a path to the fragment shader."
                    ));
                }
                settings.set_fragment_path(args[i].as_str())
            }
            _ => settings.set_obj_path(args[i].as_str()),
        }
        i += 1;
    }

    // settings.apply_default_setting()?;
    Ok(settings)
}
