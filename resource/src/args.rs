use anyhow::{anyhow, Result};

use std::env;

pub struct Settings {
    vertex_path: String,
    fragment_path: String
}

impl Settings {
    pub fn new_default() -> Self {
        Settings {
            vertex_path: "../shaders/shader.vert".to_string(),
            fragment_path: "../shaders/shader.frag".to_string()
        }
    }
}

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn parse_args(args: Vec<String>) -> Result<Settings> {
    let len_args: usize = args.len();
    let mut settings: Settings = Settings::new_default();

    if len_args == 1 {
        println!(
            "usage: {} (file | size) [-a algorithm] [-h heuristic] [-t timeout] [--verbose]",
            args[0]
        );
        return Ok(settings);
    }

    // let mut i = 1;
    // while i < len_args {
    //     let arg = args[i].as_str();
    //     match arg {
    //         "-a" | "--algorithm" => {
    //             i += 1;
    //             if i == len_args {
    //                 return Err(anyhow!(
    //                     "Need an algorithm: Use astar, uniform_cost, or greedy"
    //                 ));
    //             }
    //             settings.set_algorithm(args[i].as_str())?
    //         }
    //         "-h" | "--heuristic" => {
    //             i += 1;
    //             if i == len_args {
    //                 return Err(anyhow!(
    //                     "Need a heuristic: Use manhattan, hamming, linear_conflict or inversion_distance"
    //                 ));
    //             }
    //             settings.set_heuristic(args[i].as_str())?
    //         }
    //         "-t" | "--timeout" => {
    //             i += 1;
    //             if i == len_args {
    //                 return Err(anyhow!("Need a time: Use numerical numbers"));
    //             }
    //             settings.set_timeout(args[i].as_str())?
    //         }
    //         "--verbose" => settings.verbose = true,
    //         _ => match arg.trim().parse::<usize>() {
    //             Ok(_) => settings.set_size(arg)?,
    //             Err(_) => settings.set_text_path(arg)?,
    //         },
    //     }
    //     i += 1;
    // }

    // settings.apply_default_setting()?;
    Ok(settings)
}
