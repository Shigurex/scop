use sdl2::{
    event::EventPollIterator,
    video::{GLContext, SwapInterval, Window},
    EventPump, Sdl,
};

#[allow(dead_code)]
pub struct WindowSdl {
    sdl: Sdl,
    window: Window,
    gl_context: GLContext,
    gl: (),
    event_pump: EventPump,
}

impl WindowSdl {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        // setting up opengl version and profile (OpenGL version 3.3 is compatible with GLSL version 3.3)
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // making sdl2 window
        let window = video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .position_centered()
            .build()
            .map_err(|_| String::from("failed while building build"))?;

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)?;

        let event_pump = sdl.event_pump().unwrap();

        Ok(WindowSdl {
            sdl,
            window,
            gl_context,
            gl,
            event_pump,
        })
    }

    pub fn get_events(&mut self) -> EventPollIterator<'_> {
        self.event_pump.poll_iter()
    }

    pub fn swap_window(&self) {
        self.window.gl_swap_window();
    }
}
