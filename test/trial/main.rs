use std::ffi::c_void;

use anyhow::Result;
use args::{get_args, parse_args};
use model::Model;
use object::Object;
use parser::Parser;
use sdl2::{event::Event, keyboard::Keycode};
use shader::{make_shader_program, ShaderProgram};
use window::WindowSdl;

mod args;
mod model;
mod object;
mod parser;
mod shader;
mod window;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<()> {
    let args = get_args();
    let settings = parse_args(args)?;
    let objs = Object::parse(&settings.obj_path())?;
    let mut window_sdl = WindowSdl::new(&objs.name(), 800, 640)?;
    let shader_program = make_shader_program(&settings.vertex_path(), &settings.fragement_path())?;
    
    let model = Model::new()?;

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
        portrait(&shader_program, &model);
        window_sdl.swap_window();
    }
    Ok(())
}

fn portrait(shader_program: &ShaderProgram, model: &Model) {
    unsafe {
        gl::ClearColor(0.5, 0.5, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        shader_program.apply();

        // int vertexColorLocation = glGetUniformLocation(shaderProgram, "ourColor");
        // glUniform4f(vertexColorLocation, 0.0f, greenValue, 0.0f, 1.0f);
        // gl::GetUniformLocation(shader_program.id(), "Color");

        gl::BindVertexArray(model.vao());
        // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        gl::DrawElements(
            gl::TRIANGLES,
            6,
            gl::UNSIGNED_INT,
            std::ptr::null::<c_void>(),
        );
        gl::BindVertexArray(0);
    }
}
