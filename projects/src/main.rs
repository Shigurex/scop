use anyhow::Result;
use args::{get_args, parse_args};
use object::{parse_obj, Object};
use sdl2::{event::Event, keyboard::Keycode};
use window::WindowSdl;

mod args;
mod object;
mod window;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<()> {
    let args = get_args();
    let settings = parse_args(args)?;
    let objs = parse_obj(&settings.obj_path())?;
    let mut window_sdl = WindowSdl::new(&objs.name(), 800, 640)?;

    'main_loop: loop {
        for event in window_sdl.get_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }
        window_sdl.swap_window();
    }
    Ok(())
}
