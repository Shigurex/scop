use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use shader::*;
use window::WindowSdl;

use crate::vertex::Vertex;

// mod args;
mod shader;
// mod parse;
mod vertex;
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
    let shader = Shader::new(
        "./resource/shaders/shader.vert",
        "./resource/shaders/shader.frag",
    )?;

    let vertex = Vertex::new();

    'main_loop: loop {
        for event in window_sdl.get_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                // Event::KeyDown {
                //     keycode:
                //         Some(Keycode::Up) | Some(Keycode::Down) | Some(Keycode::Right)
                //         | Some(Keycode::Left),
                //     ..
                // } => {}
                // Event::MouseButtonDown { x, y, .. } | Event::MouseButtonUp { x, y, .. } => {}
                _ => {}
            }
        }

        // render();

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.use_shader();
            gl::BindVertexArray(vertex.vao());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        window_sdl.swap_window();
    }

    Ok(())
}
