extern crate sdl2;
// extern  crate gl;

use crate::two_d::texture_manager_anim::TextureManagerAnim;
use crate::two_d::physics::RigidBody;
use crate::two_d::ai_system::BehaviourTreeNode;
use crate::two_d::event::GEvent;
use crate::two_d::event::KeyEvent;

use std::path::Path;
use sdl2::rect::Rect;

// use gl::types::*;
use nalgebra::Vector2;

pub struct GameObject<'a> {
    pub texture_manager_anim: TextureManagerAnim<'a>,
    pub position: Vector2<i32>,
    pub collider: Rect,
    pub rigid_body: RigidBody,
    pub behaviour_tree_node: Option<BehaviourTreeNode<'a>>,
}

#[allow(dead_code)]
impl<'a> GameObject<'a> {
    pub fn new(texture_manager_anim: TextureManagerAnim<'a>, position: Vector2<i32>) -> Self {
        let collider = sdl2::rect::Rect::new(position.x, position.y, 30, 30);
        let rigid_body = RigidBody::new(1.0);
        Self {
            texture_manager_anim,
            position,
            collider,
            rigid_body,
            behaviour_tree_node: None,
        }
    }

    pub fn load_texture(&mut self, tag:&str, path: &Path, frame_width: u32, frame_height: u32, frame_delay: u32, row: u32) -> Result<(), String> {
        self.texture_manager_anim.load_animation(tag, path, frame_width, frame_height, frame_delay, row)
    }

    pub fn render_texture(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, scale: u32, flip_horizontal: bool) -> Result<(), String> {
        if let Some(tag) = &self.texture_manager_anim.current_animation {
            if let Some(animated_texture) = self.texture_manager_anim.animations.get(tag) {
                let sprite_sheet = &animated_texture.sprite_sheet;
                let dest = sdl2::rect::Rect::new(self.position.x, self.position.y, sprite_sheet.frame_width * scale, sprite_sheet.frame_height * scale);
                
                let flip = if flip_horizontal {
                    sdl2_sys::SDL_RendererFlip::SDL_FLIP_HORIZONTAL as u32
                } else {
                    sdl2_sys::SDL_RendererFlip::SDL_FLIP_NONE as u32
                };
                self.texture_manager_anim.render_texture(canvas, dest, flip)
            } else {
                Err("Texture not loaded for the current animation tag".to_owned())
            }
        } else {
            Err("No animation set".to_owned())
        }
    }
    

    pub fn update_position(&mut self, event: GEvent, colliders: &Vec<Rect>, delta_time: f32) {
        let mut new_position = self.position;

        match event {
            GEvent::KeyDown(key_event) => {
                match key_event {
                    KeyEvent::Left => { 
                        new_position.x -= 1; 
                        self.rigid_body.apply_force(Vector2::new(-new_position.x as f32, 0.0));
                    },
                    KeyEvent::Right => { 
                        new_position.x += 1; 
                        self.rigid_body.apply_force(Vector2::new(new_position.x as f32, 0.0));
                    },
                    KeyEvent::Up => { 
                        new_position.y -= 1; 
                        self.rigid_body.apply_force(Vector2::new(0.0, -new_position.y as f32));
                    },
                    KeyEvent::Down => { 
                        new_position.y += 1; 
                        self.rigid_body.apply_force(Vector2::new(0.0, new_position.y as f32));
                    },
                    _ => {},
                }
            },
            _ => {},
        }        

        self.rigid_body.update(delta_time);
        new_position += Vector2::new(self.rigid_body.velocity.x as i32, self.rigid_body.velocity.y as i32);

        let new_collider = sdl2::rect::Rect::new(new_position.x, new_position.y, self.collider.width(), self.collider.height());

        // Check for collisions with other colliders
        let mut collision = false;
        for collider in colliders.iter() {
            if new_collider.has_intersection(*collider) {
                collision = true;
                break;
            }
        }

        if !collision {
            self.position = new_position;
            self.collider = new_collider;
        }

        // Reset acceleration after updating velocity
        self.rigid_body.acceleration = Vector2::zeros();
    }

    pub fn get_position(&mut self) -> Vector2<i32> {
        self.position
    }
}