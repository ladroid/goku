extern crate sdl2;
mod two_d;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const PLAYER_MOVEMENT_SPEED: i32 = 5;
const GRAVITY: i32 = 2;
const JUMP_FORCE: i32 = -25;

const PLATFORM_SPACING: i32 = 50; // Horizontal spacing between platforms
const PLATFORM_MIN_WIDTH: u32 = 100;
const PLATFORM_MAX_WIDTH: u32 = 300;
const PLATFORM_HEIGHT: u32 = 20;

struct Player {
    rect: two_d::Rect,
    y_velocity: i32,
    on_ground: bool,
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        Player {
            rect: two_d::Rect::new(x, y, 50, 50),
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
    rect: two_d::Rect,
}

impl Platform {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Platform {
        Platform {
            rect: two_d::Rect::new(x, y, w, h),
        }
    }
}

fn main() {
    let mut window = two_d::Window::new("Rust SDL2 Demo: Platformer", SCREEN_WIDTH, SCREEN_HEIGHT, false).unwrap();
    let mut input_handler = two_d::InputHandler::new(&window.sdl_context).unwrap();

    let mut player = Player::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 - 100);

    let mut camera = two_d::Camera::new(
        nalgebra::Vector2::new(player.rect.x(), player.rect.y()), 
        nalgebra::Vector2::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    );
    
    let mut platforms = vec![
        Platform::new(0, SCREEN_HEIGHT as i32 - 50, SCREEN_WIDTH, 50), // Initial ground platform
    ];

    let mut last_platform_x = 0; // Track the last platform's X position for generation

    'running: loop {
        input_handler.poll_events(); // Poll events using InputHandler

        if input_handler.is_key_pressed(Keycode::Escape) {
            break 'running;
        }

        if input_handler.is_key_pressed(Keycode::Left) {
            player.move_left();
        }
        if input_handler.is_key_pressed(Keycode::Right) {
            player.move_right();
        }
        if input_handler.is_key_pressed(Keycode::Space) {
            player.jump();
        }

        player.update(&platforms);

        // Dynamically generate platforms as the player moves horizontally
        let screen_right_edge = camera.position.x + SCREEN_WIDTH as i32;
        while last_platform_x < screen_right_edge + PLATFORM_SPACING {
            // Generate platform to the right
            let platform_width = rand::thread_rng().gen_range(PLATFORM_MIN_WIDTH..=PLATFORM_MAX_WIDTH);
            let platform_x = last_platform_x + rand::thread_rng().gen_range(10..PLATFORM_SPACING);
            let platform_y = SCREEN_HEIGHT as i32 - PLATFORM_HEIGHT as i32 - rand::thread_rng().gen_range(0..SCREEN_HEIGHT/3) as i32;

            platforms.push(Platform::new(platform_x, platform_y, platform_width, PLATFORM_HEIGHT));

            last_platform_x = platform_x + platform_width as i32;
        }

        // Optional: Remove platforms that are no longer visible to the left
        let screen_left_edge = camera.position.x;
        platforms.retain(|p| p.rect.x() + p.rect.width() as i32 > screen_left_edge);

        // Update the camera to follow the player
        camera.update(nalgebra::Vector2::new(player.rect.x(), player.rect.y()));

        window.canvas.set_draw_color(two_d::Color::new(0, 0, 0).sdl_color());
        window.canvas.clear();

        // During rendering of the player and platforms:
        window.canvas.set_draw_color(two_d::Color::new(255, 255, 255).sdl_color()); // Set color for the player
        window.canvas.fill_rect(camera.transform_rect(&player.rect)).unwrap(); // Render player

        // Set color for platforms and render them
        window.canvas.set_draw_color(two_d::Color::new(120, 120, 120).sdl_color());
        for platform in &platforms {
            window.canvas.fill_rect(camera.transform_rect(&platform.rect)).unwrap();
        }

        window.canvas.present();
        ::std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}