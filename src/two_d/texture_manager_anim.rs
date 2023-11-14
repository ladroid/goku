extern crate sdl2;
// extern  crate gl;

use crate::two_d::sprite_sheet::SpriteSheet;
use crate::two_d::animated_texture::AnimatedTexture;

use std::collections::HashMap;
use sdl2::image::LoadTexture;
use std::path::Path;

pub struct TextureManagerAnim<'a> {
    pub animations: HashMap<String, AnimatedTexture<'a>>,
    pub texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub current_animation: Option<String>,
}

#[allow(dead_code)]
impl<'a> TextureManagerAnim<'a> {
    pub fn new(texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        Self {
            animations: HashMap::new(),
            texture_creator,
            current_animation: None,
        }
    }

    pub fn load_animation(&mut self, tag: &str, path: &Path, frame_width: u32, frame_height: u32, frame_delay: u32, row: u32) -> Result<(), String> {
        let texture = self.texture_creator.load_texture(path)?;
        let sprite_sheet = SpriteSheet::new(texture, frame_width, frame_height, row);
        let animated_texture = AnimatedTexture::new(sprite_sheet, frame_delay);
        self.animations.insert(tag.to_string(), animated_texture);

        if self.current_animation.is_none() {
            self.current_animation = Some(tag.to_string());
        }

        Ok(())
    }

    pub fn set_animation(&mut self, tag: &str) {
        if self.animations.contains_key(tag) {
            self.current_animation = Some(tag.to_string());
        }
    }

    pub fn render_texture(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, dest: sdl2::rect::Rect, flip: u32) -> Result<(), String> {
        if let Some(tag) = &self.current_animation {
            if let Some(texture) = self.animations.get_mut(tag) {
                texture.render(canvas, dest, flip)?;
                Ok(())
            } else {
                Err("Texture not loaded for the current animation tag".to_owned())
            }
        } else {
            Err("No animation set".to_owned())
        }
    }
}