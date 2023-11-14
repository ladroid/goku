extern crate sdl2;
// extern  crate gl;

use sdl2::image::LoadTexture;
use std::path::Path;
use sdl2::render::Texture;

pub struct TextureManager<'a> {
    pub texture: Option<Texture<'a>>,
    pub texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

#[allow(dead_code)]
impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        Self {
            texture: None,
            texture_creator,
        }
    }

    pub fn load_texture(&mut self, path: &Path) -> Result<(), String> {
        let texture = self.texture_creator.load_texture(path)?;

        //  // Update collider if the texture has a collider
        //  if self.has_collider {
        //     let query = texture.query();
        //     self.collider.set_width(query.width);
        //     self.collider.set_height(query.height);
        // }

        self.texture = Some(texture);
        Ok(())
    }

    pub fn render_texture(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, dest: sdl2::rect::Rect) -> Result<(), String> {
        if let Some(texture) = &self.texture {
            canvas.copy(texture, None, dest)?;
            Ok(())
        } else {
            Err("Texture not loaded".to_owned())
        }
    }

    // pub fn update_position(&mut self, position: Vector2<i32>) {
    //     self.collider.set_x(position.x);
    //     self.collider.set_y(position.y);
    // }
}