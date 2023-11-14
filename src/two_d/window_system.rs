extern crate sdl2;
// extern  crate gl;

use sdl2::render::Canvas;

// Window system
pub struct Window {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: Canvas<sdl2::video::Window>,
}

#[allow(dead_code)]
impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .opengl()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self { sdl_context, video_subsystem, canvas })
    }
}