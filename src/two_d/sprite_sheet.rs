extern crate sdl2;
// extern  crate gl;

use sdl2::render::Texture;
use sdl2::rect::Rect;

pub struct SpriteSheet<'a> {
    pub texture: Texture<'a>,
    pub frame_width: u32,
    pub frame_height: u32,
    pub row: u32,
}

impl<'a> SpriteSheet<'a> {
    pub fn new(texture: Texture<'a>, frame_width: u32, frame_height: u32, row: u32) -> Self {
        Self {
            texture,
            frame_width,
            frame_height,
            row,
        }
    }

    pub fn get_frame(&self, index: u32) -> Rect {
        let x = (index * self.frame_width) as i32;
        let y = (self.row * self.frame_height) as i32;
        Rect::new(x, y, self.frame_width, self.frame_height)
    }
}