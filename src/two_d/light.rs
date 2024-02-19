extern crate sdl2;
use crate::two_d;
// extern  crate gl;


// light & shadow
pub struct PointLight {
    pub position: nalgebra::Vector2<f32>,
    pub radius: f32,
    pub intensity: f32,
    pub color: two_d::Color,
}

#[allow(dead_code)]
impl PointLight {
    pub fn new(position: nalgebra::Vector2<f32>, radius: f32, intensity: f32, color: two_d::Color) -> Self {
        PointLight {
            position,
            radius,
            intensity,
            color,
        }
    }

    // Render the point light onto a texture (you might need to adapt this)
    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, light_spot_texture: &mut sdl2::render::Texture) {
        let alpha_value = (self.intensity * 255.0).clamp(0.0, 255.0) as u8;
        
        // Set the alpha and color modulation of the light texture
        light_spot_texture.set_alpha_mod(alpha_value);
        light_spot_texture.set_color_mod(self.color.r(), self.color.g(), self.color.b());
        
        let dst_rect = sdl2::rect::Rect::new(
            self.position.x as i32 - self.radius as i32,
            self.position.y as i32 - self.radius as i32,
            (self.radius * 2.0) as u32,
            (self.radius * 2.0) as u32,
        );
        
        canvas.copy(light_spot_texture, None, Some(dst_rect)).unwrap();
    }
}

pub struct SpotLight {
    pub position: nalgebra::Vector2<f32>,
    pub direction: nalgebra::Vector2<f32>,  // Direction the light is facing.
    pub cutoff_angle: f32,  // Angle of the cone (in degrees).
    pub distance: f32,      // Maximum distance the light can reach.
    pub intensity: f32,
    pub color: sdl2::pixels::Color,
}

#[allow(dead_code)]
impl SpotLight {
    pub fn new(position: nalgebra::Vector2<f32>, direction: nalgebra::Vector2<f32>, cutoff_angle: f32, distance: f32, intensity: f32, color: sdl2::pixels::Color) -> Self {
        SpotLight {
            position,
            direction,
            cutoff_angle,
            distance,
            intensity,
            color,
        }
    }

    // Render the spotlight onto a texture (you might need to adapt this)
    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, spotlight_texture: &mut sdl2::render::Texture) {
        let alpha_value = (self.intensity * 255.0).clamp(0.0, 255.0) as u8;
        
        // Set the alpha and color modulation of the light texture
        spotlight_texture.set_alpha_mod(alpha_value);
        spotlight_texture.set_color_mod(self.color.r, self.color.g, self.color.b);
        
        let dst_rect = sdl2::rect::Rect::new(
            self.position.x as i32 - (self.distance / 2.0) as i32,
            self.position.y as i32 - (self.distance / 2.0) as i32,
            self.distance as u32,
            self.distance as u32,
        );
        
        // You might want to rotate the texture to match the direction of the spotlight
        let angle = self.direction.y.atan2(self.direction.x).to_degrees() - 90.0;  // Convert to degrees and adjust for the spotlight texture orientation.
        
        canvas.copy_ex(spotlight_texture, None, Some(dst_rect), angle as f64, None, false, false).unwrap();
    }
}

pub struct AmbientFilter {
    pub intensity: f32,
}
#[allow(dead_code)]
impl AmbientFilter {
    pub fn new(intensity: f32) -> Self {
        AmbientFilter { intensity }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, light_texture: &mut sdl2::render::Texture) {
        let alpha_value = (self.intensity * 255.0).clamp(0.0, 255.0) as u8;
        light_texture.set_alpha_mod(alpha_value);
        
        // Cover the entire scene with the ambient light
        canvas.copy(light_texture, None, None).unwrap();
    }
}