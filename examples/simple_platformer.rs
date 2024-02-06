extern crate sdl2;
mod two_d;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const PLAYER_MOVEMENT_SPEED: i32 = 5;
const GRAVITY: i32 = 2;
const JUMP_FORCE: i32 = -25;

struct Player {
    rect: Rect,
    y_velocity: i32,
    on_ground: bool,
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        Player {
            rect: Rect::new(x, y, 50, 50),
            y_velocity: 0,
            on_ground: false,
        }
    }

    fn update(&mut self, platforms: &[Platform]) {
        // Apply gravity if not on ground
        if !self.on_ground {
            self.y_velocity += GRAVITY;
        }

        // Predict next Y position
        let next_y = self.rect.y() + self.y_velocity;

        // Reset on_ground status to check again
        self.on_ground = false;

        // Check for collisions with platforms
        for platform in platforms {
            // Check if the player is above the platform and moving down
            if self.rect.x() < platform.rect.x() + platform.rect.width() as i32 &&
               self.rect.x() + self.rect.width() as i32 > platform.rect.x() &&
               self.rect.y() + self.rect.height() as i32 <= platform.rect.y() &&
               next_y + self.rect.height() as i32 > platform.rect.y() {
                // Adjust Y position to top of the platform, simulate landing
                self.rect.set_y(platform.rect.y() - self.rect.height() as i32);
                self.y_velocity = 0;
                self.on_ground = true;
                return; // Early return to avoid further adjustments
            }
        }

        // Update Y position if no collision with the top of a platform
        self.rect.set_y(next_y.max(0).min((SCREEN_HEIGHT - self.rect.height()) as i32));
    }

    // Ensure jump is only allowed if on_ground is true
    fn jump(&mut self) {
        if self.on_ground {
            self.y_velocity = JUMP_FORCE;
            self.on_ground = false; // Player leaves the ground
        }
    }

    fn move_left(&mut self) {
        self.rect.set_x(self.rect.x() - PLAYER_MOVEMENT_SPEED);
    }

    fn move_right(&mut self) {
        self.rect.set_x(self.rect.x() + PLAYER_MOVEMENT_SPEED);
    }
}

struct Platform {
    rect: Rect,
}

impl Platform {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Platform {
        Platform {
            rect: Rect::new(x, y, w, h),
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust SDL2 Demo: Platformer", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player = Player::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 - 100);

    let mut camera = two_d::Camera::new(
        nalgebra::Vector2::new(player.rect.x(), player.rect.y()), 
        nalgebra::Vector2::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    );
    
    let platforms = vec![
        Platform::new(0, SCREEN_HEIGHT as i32 - 50, SCREEN_WIDTH, 50),
        Platform::new(200, SCREEN_HEIGHT as i32 - 150, 150, 20),
        Platform::new(400, SCREEN_HEIGHT as i32 - 250, 200, 20),
    ];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            player.move_left();
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            player.move_right();
        }
        if keyboard_state.is_scancode_pressed(Scancode::Space) {
            player.jump();
        }

        player.update(&platforms);
        // Update the camera to follow the player
        camera.update(nalgebra::Vector2::new(player.rect.x(), player.rect.y()));

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the player using camera transformation (the real player in white)
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(camera.transform_rect(player.rect)).unwrap();

        // Draw platforms using camera transformation
        canvas.set_draw_color(Color::RGB(120, 120, 120));
        for platform in &platforms {
            canvas.fill_rect(camera.transform_rect(platform.rect)).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}