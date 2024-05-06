use std::env;

use anyhow::{anyhow, Result};

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

    pub fn new_default() -> Self {
        Self {
            obj_path: String::from("./objects/42.obj"),
            vertex_path: String::from("./projects/shaders/shader.vert"),
            fragement_path: String::from("./projects/shaders/shader.frag"),
        }
    }

    pub fn obj_path(&self) -> String {
        self.obj_path.clone()
    }

    #[allow(dead_code)]
    pub fn vertex_path(&self) -> String {
        self.vertex_path.clone()
    }

    #[allow(dead_code)]
    pub fn fragement_path(&self) -> String {
        self.fragement_path.clone()
    }
}

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn parse_args(args: Vec<String>) -> Result<Setting> {
    let mut args_iter = args.iter().peekable();

    match args_iter.next() {
        Some(path) => {
            if args_iter.peek().is_none() {
                return Err(anyhow!(
                    "usage: {path} (obj_path) [-v vertex_path] [-f fragment_path]"
                ));
            }
        }
        None => return Err(anyhow!("failed to execute")),
    };

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-v" | "--vertex" => {
                if let Some(path) = args_iter.next() {
                    println!("vertex path is set to {path}")
                } else {
                    return Err(anyhow!("vertex path is undefined"));
                }
            }
            "-f" | "--fragment" => {
                if let Some(path) = args_iter.next() {
                    println!("fragment path is set to {path}")
                } else {
                    return Err(anyhow!("fragment path is undefined"));
                }
            }
            _ => {
                println!("object path is set to {arg}")
            }
        }
    }
    Ok(Setting::new_default())
}
