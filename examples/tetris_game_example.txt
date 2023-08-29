pub struct RigidBody {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub mass: f32,
    pub speed: f32,
}

impl Clone for RigidBody {
    fn clone(&self) -> Self {
        Self {
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone(),
            mass: self.mass.clone(),
            speed: self.speed.clone(),
        }
    }
}


impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: Vector2::zeros(),
            acceleration: Vector2::zeros(),
            mass,
            speed: 10.0,
        }
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.acceleration += force / self.mass;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.velocity += self.acceleration * delta_time;

        // Normalize velocity
        let current_speed = self.velocity.norm();
        if current_speed != 0.0 {
            self.velocity /= current_speed;
        }

        // Apply speed limit
        self.velocity *= self.speed;
    }
    
    pub fn reset_acceleration(&mut self) {
        self.acceleration = Vector2::zeros();
    }
}

pub struct Shape2D {
    blocks: Vec<Rect>,
    color: sdl2::pixels::Color,
    rigid_body: RigidBody,
}

impl Clone for Shape2D {
    fn clone(&self) -> Self {
        Self {
            rigid_body: self.rigid_body.clone(),
            blocks: self.blocks.clone(),
            color: self.color,
        }
    }
}

impl Shape2D {
    pub fn new(blocks: Vec<Rect>, color: sdl2::pixels::Color, mass: f32) -> Self {
        Shape2D {
            blocks,
            color,
            rigid_body: RigidBody::new(mass),
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        for block in &self.blocks {
            canvas.fill_rect(*block)?;
        }

        Ok(())
    }

    pub fn collides_with(&self, other: &Shape2D) -> bool {
        for block_a in &self.blocks {
            for block_b in &other.blocks {
                if block_a.x() == block_b.x() && block_a.y() == block_b.y() {
                    return true;
                }
            }
        }
        false
    }    

    // Add this method inside the Shape2D impl block
    pub fn collision_with_placed_tetrominos(&self, placed_tetrominos: &[Shape2D]) -> bool {
        for placed_tetromino in placed_tetrominos {
            for placed_block in &placed_tetromino.blocks {
                for active_block in &self.blocks {
                    if active_block == placed_block {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        screen_width: i32,
        screen_height: i32,
        placed_tetrominos: &[Shape2D],
    ) -> bool {
        self.rigid_body.update(delta_time);
    
        let previous_blocks_positions: Vec<Rect> = self.blocks.clone();
    
        for block in &mut self.blocks {
            block.offset(self.rigid_body.velocity.x as i32, self.rigid_body.velocity.y as i32);
        }
    
        let dx = 0;
        let dy = 1;
    
        let mut collision = false;
    
        if !self.is_valid_position(dx, dy, screen_width, screen_height) {
            self.rigid_body.velocity.y = 0.0;
            self.rigid_body.reset_acceleration();
            collision = true;
        }
    
        if !self.is_valid_position(dx + 1, dy, screen_width, screen_height)
            || !self.is_valid_position(dx - 1, dy, screen_width, screen_height)
        {
            self.blocks = previous_blocks_positions.clone();
            self.rigid_body.velocity.x = 0.0;
            collision = true;
        }
    
        // Check for collisions with placed tetrominos
        for placed_tetromino in placed_tetrominos {
            if self.collides_with(placed_tetromino) {
                self.blocks = previous_blocks_positions.clone();
                self.rigid_body.velocity.y = 0.0;
                self.rigid_body.reset_acceleration();
                collision = true;
                break;
            }
        }
    
        collision
    }                    

    pub fn is_valid_position(&self, dx: i32, dy: i32, screen_width: i32, screen_height: i32) -> bool {
        for block in &self.blocks {
            let new_x = block.x() + dx;
            let new_y = block.y() + dy;

            if new_x < 0 || new_x + BLOCK_SIZE as i32 > screen_width || new_y < 0 || new_y + BLOCK_SIZE as i32 > screen_height {
                return false;
            }
        }
        true
    }

    pub fn rotate(&mut self) {
        let pivot = self.blocks[1]; // Pivot around the center of the tetromino
        for block in &mut self.blocks {
            // Perform the rotation around the pivot block
            let dx = block.x() - pivot.x();
            let dy = block.y() - pivot.y();

            block.set_x(pivot.x() - dy);
            block.set_y(pivot.y() + dx);
        }
    }
}

fn update_tetromino_position(tetromino: &mut Shape2D, dx: i32, dy: i32) {
    for block in &mut tetromino.blocks {
        block.set_x(block.x() + dx);
        block.set_y(block.y() + dy);
    }
}

const BLOCK_SIZE: u32 = 20;
fn create_tetromino(x: i32, y: i32, shape_type: char, color: sdl2::pixels::Color, mass: f32) -> Shape2D {
    let shape = match shape_type {
        'I' => vec![
            Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + 2 * BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + 3 * BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
        ],
        'O' => vec![
            Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
        ],
        'T' => vec![
            Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + 2 * BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
        ],
        'L' => vec![
            Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x, y + 2 * BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y + 2 * BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
        ],
        'J' => vec![
            Rect::new(x + BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y + 2 * BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x, y + 2 * BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
        ],
        'S' => vec![
            Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + 2 * BLOCK_SIZE as i32, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
        ],
        'Z' => vec![
            Rect::new(x + BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + 2 * BLOCK_SIZE as i32, y, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Rect::new(x + BLOCK_SIZE as i32, y + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
        ],
        // Add more shapes here
        _ => vec![],
    };

    Shape2D::new(shape, color, mass)
}

pub fn test_tetris() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Simple Tetris", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut tetromino_t = create_tetromino(60, 0, 'T', sdl2::pixels::Color::RGB(255, 0, 0), 1.0);
    let mut placed_tetrominos: Vec<Shape2D> = Vec::new();

    let gravity = Vector2::new(0.0, 9.8);

    let mut timer = std::time::Instant::now();
    let mut delta_time: f32;

    // Add these lines before the main loop
    let mut spawn_new_tetromino = false;
    let spawn_delay = std::time::Duration::from_secs(2);
    let mut spawn_time = std::time::Instant::now();

    'running: loop {
        delta_time = timer.elapsed().as_secs_f32();
        timer = std::time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    tetromino_t.rigid_body.velocity.x -= 10.0;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    tetromino_t.rigid_body.velocity.x += 10.0;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    // If the space key is pressed, rotate the tetromino
                    tetromino_t.rotate();
                },
                _ => {}
            }
        }

        // Add these lines inside the main loop
        if spawn_new_tetromino {
            if spawn_time.elapsed() >= spawn_delay {
                tetromino_t = create_tetromino(60, 0, 'T', sdl2::pixels::Color::RGB(255, 0, 0), 1.0);
                spawn_new_tetromino = false;
            }
        } else {
            tetromino_t.rigid_body.apply_force(gravity * delta_time);
            let collision = tetromino_t.update(delta_time, 800, 600, &placed_tetrominos);
            if collision {
                placed_tetrominos.push(tetromino_t.clone());
                spawn_new_tetromino = true;
                spawn_time = std::time::Instant::now();
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        tetromino_t.draw(&mut canvas).unwrap();

        // Add these lines to draw the placed tetrominos
        for placed_tetromino in &placed_tetrominos {
            placed_tetromino.draw(&mut canvas).unwrap();
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}