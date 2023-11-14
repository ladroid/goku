extern crate sdl2;
// extern  crate gl;
use crate::two_d::sprite_sheet::SpriteSheet;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

pub struct AnimatedTexture<'a> {
    pub sprite_sheet: SpriteSheet<'a>,
    pub frame_delay: u32,
    pub current_frame: u32,
    pub last_frame_time: u32,
    pub flip: sdl2_sys::SDL_RendererFlip,
}

impl<'a> AnimatedTexture<'a> {
    pub fn new(sprite_sheet: SpriteSheet<'a>, frame_delay: u32) -> Self {
        Self {
            sprite_sheet,
            frame_delay,
            current_frame: 0,
            last_frame_time: 0,
            flip: sdl2_sys::SDL_RendererFlip::SDL_FLIP_NONE, // Default to no flip
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>, dest: Rect, flip: u32) -> Result<(), String> {
        let now = unsafe {
            sdl2_sys::SDL_GetTicks()
        };
        let delta_time = now - self.last_frame_time;
        if delta_time >= self.frame_delay {
            self.current_frame = (self.current_frame + 1) % (self.sprite_sheet.texture.query().width / self.sprite_sheet.frame_width) as u32;
            self.last_frame_time = now;
        }

        let src = self.sprite_sheet.get_frame(self.current_frame);
        canvas.copy_ex(&self.sprite_sheet.texture, Some(src), Some(dest), 0.0, None, flip == sdl2_sys::SDL_RendererFlip::SDL_FLIP_HORIZONTAL as u32, false)?;
        // canvas.copy(&self.sprite_sheet.texture, src, dest)?;

        Ok(())
    }
}