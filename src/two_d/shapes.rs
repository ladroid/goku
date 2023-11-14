extern crate sdl2;
// extern  crate gl;
use crate::two_d::RigidBody;

use sdl2::rect::Rect;

// shapes
const BLOCK_SIZE: u32 = 20;
pub struct Shape2D {
    pub blocks: Vec<Rect>,
    pub color: sdl2::pixels::Color,
    pub rigid_body: RigidBody,
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

#[allow(dead_code)]
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