mod two_d;
extern crate sdl2;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

#[allow(dead_code)]
pub fn spawn_particles_glowing_orbs(particles: &mut Vec<two_d::Particle>, x: i32, y: i32, count: usize, shape: two_d::ParticleShape) {
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        // Randomize the direction and speed for a floating effect
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(10.0..30.0); // slower speed for a floating effect
        let x_vel = angle.cos() * speed;
        let y_vel = angle.sin() * speed;

        // Randomize life, size, and color
        let life = rng.gen_range(3.0..6.0);
        let size = rng.gen_range(4..10); // larger size for visible glow
        let color = sdl2::pixels::Color::RGBA(
            rng.gen_range(100..256) as u8,
            rng.gen_range(100..256) as u8,
            rng.gen_range(100..256) as u8,
            128, // semi-transparent for a glowing effect
        );

        particles.push(two_d::Particle::new(x as f32, y as f32, x_vel, y_vel, life, color, shape));
        particles.last_mut().unwrap().size = size; // setting the size of the particle
    }
}

#[allow(dead_code)]
pub fn spawn_particles_stardust(particles: &mut Vec<two_d::Particle>, screen_width: u32, screen_height: u32, count: usize, shape: two_d::ParticleShape) {
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let x = rng.gen_range(0..screen_width) as f32;
        let y = rng.gen_range(0..screen_height) as f32;
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(1.0..5.0);
        let x_vel = angle.cos() * speed;
        let y_vel = angle.sin() * speed;
        let life = rng.gen_range(3.0..6.0);
        let size = rng.gen_range(2..5);
        let color = if rng.gen_bool(0.5) {
            sdl2::pixels::Color::RGB(255, 255, 255) // White color
        } else {
            sdl2::pixels::Color::RGB(255, 255, 224) // Light yellow color
        };

        particles.push(two_d::Particle::new(x, y, x_vel, y_vel, life, color, shape));
        particles.last_mut().unwrap().size = size;
    }
}

#[allow(dead_code)]
pub fn spawn_particles_swirling_leaves(particles: &mut Vec<two_d::Particle>, screen_width: u32, screen_height: u32, count: usize, shape: two_d::ParticleShape) {
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let x = rng.gen_range(0..screen_width) as f32;
        let y = 0.0; // start at the top of the screen
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(1.0..3.0);
        let x_vel = angle.cos() * speed;
        let y_vel = angle.sin() * speed;
        let life = rng.gen_range(5.0..10.0);
        let size = rng.gen_range(8..15); // larger size for leaf shape
        let color = match rng.gen_range(0..4) {
            0 => sdl2::pixels::Color::RGB(34, 139, 34), // Forest Green
            1 => sdl2::pixels::Color::RGB(255, 165, 0), // Orange
            2 => sdl2::pixels::Color::RGB(255, 215, 0), // Gold
            _ => sdl2::pixels::Color::RGB(160, 82, 45),  // Sienna
        };

        particles.push(two_d::Particle::new(x, y, x_vel, y_vel, life, color, shape));
        particles.last_mut().unwrap().size = size;
    }
}

// Ensure you integrate the circle update logic and the conditional bubble spawning into your main loop or relevant update function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = two_d::Window::new("My Game", 800, 600)?;

    let mut event_pump = window.sdl_context.event_pump().unwrap();

    let mut particles: Vec<two_d::Particle> = Vec::new();
    let mut last_update = std::time::Instant::now();
    let mut last_spawn_time = std::time::Instant::now();
    let spawn_interval = std::time::Duration::from_secs(3); // Change interval as needed
    let spawn_count = 20; // Number of particles to spawn each time

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        let now = std::time::Instant::now();
        let delta = now.duration_since(last_update);
        let delta_time = delta.as_secs_f32();
        last_update = now;

        // Spawn Stardust particles at intervals
        if last_spawn_time.elapsed() >= spawn_interval {
            //spawn_particles_stardust(&mut particles, 800, 600, spawn_count);
            spawn_particles_swirling_leaves(&mut particles, 800, 600, spawn_count, two_d::ParticleShape::Circle);
            last_spawn_time = std::time::Instant::now();
        }

        // Update and render particles
        particles.retain_mut(|p| {
            p.update(delta_time, 600);
            p.life > 0.0
        });

        window.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        window.canvas.clear();

        for particle in &particles {
            particle.render(&mut window.canvas);
        }

        window.canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}