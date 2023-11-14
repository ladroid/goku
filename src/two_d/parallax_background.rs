extern crate sdl2;

use crate::two_d::texture_manager::TextureManager;
use crate::two_d::camera::Camera;

// parallax background
pub struct ParallaxLayer<'a> {
    pub texture_manager: TextureManager<'a>,
    pub speed: f32,
    pub offset: f32,
}

#[allow(dead_code)]
impl<'a> ParallaxLayer<'a> {
    pub fn new(texture_manager: TextureManager<'a>, speed: f32) -> Self {
        Self {
            texture_manager,
            speed,
            offset: 0.0,
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, camera: &Camera) -> Result<(), String> {
        let mut dest = sdl2::rect::Rect::new(0, 0, self.texture_manager.texture.as_ref().unwrap().query().width, self.texture_manager.texture.as_ref().unwrap().query().height);
        dest.x = (self.offset - camera.position.x as f32 * self.speed) as i32;
        while dest.x < camera.position.x + camera.size.x as i32 {
            self.texture_manager.render_texture(canvas, dest)?;
            dest.x += dest.width() as i32;
        }
        Ok(())
    }

    pub fn update(&mut self, delta_time: f32) {
        self.offset += self.speed * delta_time;
        if self.offset > self.texture_manager.texture.as_ref().unwrap().query().width as f32 {
            self.offset -= self.texture_manager.texture.as_ref().unwrap().query().width as f32;
        }
    }
}

pub struct ParallaxBackground<'a> {
    pub layers: Vec<ParallaxLayer<'a>>,
}

#[allow(dead_code)]
impl<'a> ParallaxBackground<'a> {
    pub fn new(layers: Vec<ParallaxLayer<'a>>) -> Self {
        Self {
            layers,
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, camera: &Camera) -> Result<(), String> {
        for layer in &self.layers {
            layer.render(canvas, camera)?;
        }
        Ok(())
    }

    pub fn update(&mut self, delta_time: f32) {
        for layer in &mut self.layers {
            layer.update(delta_time);
        }
    }
}