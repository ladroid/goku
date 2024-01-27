extern crate sdl2;
// extern  crate gl;

use sdl2::rect::Rect;
use sdl2::render::Canvas;

use rand::Rng;

// particle system +
    // Shader +
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub x_vel: f32,
    pub y_vel: f32,
    pub life: f32,
    pub size: u32,
    pub color: sdl2::pixels::Color,
    pub alpha: u8,
}

#[allow(dead_code)]
impl Particle {
    pub fn new(x: f32, y: f32, x_vel: f32, y_vel: f32, life: f32, color: sdl2::pixels::Color) -> Self {
        Particle {
            x,
            y,
            x_vel,
            y_vel,
            life,
            size: 2,
            color,
            alpha: 255,
        }
    }

    pub fn update(&mut self, delta_time: f32, screen_height: u32) {
        self.x += self.x_vel * delta_time;
        self.y += self.y_vel * delta_time;
        self.life -= delta_time;
        self.alpha = (self.life * 255.0).max(0.0).min(255.0) as u8;

        // If particle reaches bottom of the screen, respawn it at the top
        if self.y as u32 > screen_height {
            self.y = 0.0;
            self.life = 5.0;
        }
    }

    pub fn render<T: sdl2::render::RenderTarget>(&self, canvas: &mut Canvas<T>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(self.color.r, self.color.g, self.color.b, self.alpha));
        canvas.fill_rect(Rect::new(self.x as i32, self.y as i32, self.size, self.size)).unwrap();
    }
}

#[allow(dead_code)]
pub fn spawn_particles_sparks(particles: &mut Vec<Particle>, x: i32, y: i32, count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..200.0);
        let x_vel = angle.cos() * speed;
        let y_vel = angle.sin() * speed;
        let life = rng.gen_range(0.5..2.5);
        let color = sdl2::pixels::Color::RGBA(123, 56, 89, 255);

        particles.push(Particle::new(x as f32, y as f32, x_vel, y_vel, life, color));
    }
}

#[allow(dead_code)]
pub fn spawn_particles_fires(particles: &mut Vec<Particle>, x: i32, y: i32, count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let angle = rng.gen_range(std::f32::consts::PI..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..200.0);
        let x_vel = angle.cos() * speed;
        let y_vel = angle.sin() * speed;
        let life = rng.gen_range(0.5..2.5);
        let color = match rng.gen_range(0..3) {
            0 => sdl2::pixels::Color::RGB(254, 95, 85),
            1 => sdl2::pixels::Color::RGB(254, 207, 92),
            _ => sdl2::pixels::Color::RGB(254, 253, 153),
        };

        particles.push(Particle::new(x as f32, y as f32, x_vel, y_vel, life, color));
    }
}

#[allow(dead_code)]
pub fn spawn_particles_rain(particles: &mut Vec<Particle>, screen_width: u32, count: usize) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..count {
        let x = rng.gen_range(0..screen_width) as f32; // random horizontal position
        let y = 0.0; // start at the top of the screen
        let x_vel = rng.gen_range(-5.0..5.0); // slight horizontal movement
        let y_vel = rng.gen_range(50.0..100.0); // vertical falling movement
        let life = rng.gen_range(2.0..5.0); // life of raindrop
        let color = sdl2::pixels::Color::RGBA(0, 0, 255, 255); // blue color for rain
        
        particles.push(Particle::new(x, y, x_vel, y_vel, life, color));
    }
}