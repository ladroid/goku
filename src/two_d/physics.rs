extern crate sdl2;
// extern  crate gl;

use nalgebra::Vector2;

// physics
    // Collider  +
    // Rigidbody +
    // Kinematic body -
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

#[allow(dead_code)]
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

    pub fn apply_gravity(&mut self, gravity: Vector2<f32>) {
        self.acceleration += gravity;
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