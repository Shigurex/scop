use crate::parse::parse_file;
use objects::*;
use render::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use window::WindowSdl;

mod objects;
mod parse;
mod render;
mod window;

fn main() {
    let mut window_sdl = WindowSdl::new(800, 640).unwrap();

    'main_loop: loop {
        for event in window_sdl.get_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {}
                _ => {}
            }
        }

        render();

        window_sdl.swap_window();
    }
}
