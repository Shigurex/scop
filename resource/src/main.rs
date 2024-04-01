use anyhow::{Result};
use shader::*;
use render::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use window::WindowSdl;

// mod args;
mod shader;
// mod parse;
mod render;
mod window;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<()> {
    const WIN_WIDTH: u32 = 800;
    const WIN_HEIGHT: u32 = 640;

    let mut window_sdl = WindowSdl::new(WIN_WIDTH, WIN_HEIGHT)?;
    let shader = Shader::new("./resource/shaders/shader.vert", "./resource/shaders/shader.frag")?;

    'main_loop: loop {
        for event in window_sdl.get_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Up)
                    | Some(Keycode::Down)
                    | Some(Keycode::Right)
                    | Some(Keycode::Left),
                    ..
                } => {},
                Event::MouseButtonDown {
                    x,
                    y,
                    ..
                } | Event::MouseButtonUp {
                    x,
                    y,
                    ..
                } => {},
                _ => {}
            }
        }

        // render();

        unsafe {
            gl::ClearColor(0.9, 0.1, 0.9, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader.use_shader();
        }

        window_sdl.swap_window();
    }

    Ok(())
}
