use args::{get_args, parse_args};
use model::{make_model, Model};
use sdl2::{event::Event, keyboard::Keycode};
use shader::make_shader_program;
use window::WindowSdl;

mod args;
mod shader;
mod window;
mod model;
mod object;
mod define;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<(), String> {
    let args = get_args();
    let settings = parse_args(args);
    let mut window_sdl = WindowSdl::new("scop", 800, 640)?;
    let object = object::parse_object(settings.obj_path())?;
    let shader_program = make_shader_program(&settings.vertex_path(), &settings.fragement_path())?;
    let model = make_model(&object);

    'main_loop: loop {
        for event in window_sdl.get_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right),
                    ..
                } => {}
                Event::MouseWheel {
                    direction: _direction,
                    ..
                } => {}
                _ => {}
            }
        }
        window_sdl.swap_window();
    }
    Ok(())
}
