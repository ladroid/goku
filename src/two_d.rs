extern crate sdl2;
// extern  crate gl;

use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use std::path::Path;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use sdl2::EventPump;
use sdl2::controller::GameController;

use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, Channel};

// use gl::types::*;
use nalgebra::Vector2;

use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;
use serde::ser::SerializeTuple;

use rand::Rng;

pub struct SpriteSheet<'a> {
    pub texture: Texture<'a>,
    pub frame_width: u32,
    pub frame_height: u32,
}

impl<'a> SpriteSheet<'a> {
    pub fn new(texture: Texture<'a>, frame_width: u32, frame_height: u32) -> Self {
        Self {
            texture,
            frame_width,
            frame_height,
        }
    }

    pub fn get_frame(&self, index: u32) -> Rect {
        let x = (index * self.frame_width) as i32;
        Rect::new(x, 0, self.frame_width, self.frame_height)
    }
}

pub struct AnimatedTexture<'a> {
    pub sprite_sheet: SpriteSheet<'a>,
    pub frame_delay: u32,
    pub current_frame: u32,
    pub last_frame_time: u32,
}

impl<'a> AnimatedTexture<'a> {
    pub fn new(sprite_sheet: SpriteSheet<'a>, frame_delay: u32) -> Self {
        Self {
            sprite_sheet,
            frame_delay,
            current_frame: 0,
            last_frame_time: 0,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>, dest: Rect) -> Result<(), String> {
        let now = unsafe {
            sdl2_sys::SDL_GetTicks()
        };
        let delta_time = now - self.last_frame_time;
        if delta_time >= self.frame_delay {
            self.current_frame = (self.current_frame + 1) % (self.sprite_sheet.texture.query().width / self.sprite_sheet.frame_width) as u32;
            self.last_frame_time = now;
        }

        let src = self.sprite_sheet.get_frame(self.current_frame);
        canvas.copy(&self.sprite_sheet.texture, src, dest)?;

        Ok(())
    }
}

pub struct TextureManager<'a> {
    pub texture: Option<Texture<'a>>,
    pub texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

#[allow(dead_code)]
impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        Self {
            texture: None,
            texture_creator,
        }
    }

    pub fn load_texture(&mut self, path: &Path) -> Result<(), String> {
        let texture = self.texture_creator.load_texture(path)?;

        //  // Update collider if the texture has a collider
        //  if self.has_collider {
        //     let query = texture.query();
        //     self.collider.set_width(query.width);
        //     self.collider.set_height(query.height);
        // }

        self.texture = Some(texture);
        Ok(())
    }

    pub fn render_texture(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, dest: sdl2::rect::Rect) -> Result<(), String> {
        if let Some(texture) = &self.texture {
            canvas.copy(texture, None, dest)?;
            Ok(())
        } else {
            Err("Texture not loaded".to_owned())
        }
    }

    // pub fn update_position(&mut self, position: Vector2<i32>) {
    //     self.collider.set_x(position.x);
    //     self.collider.set_y(position.y);
    // }
}

pub struct TextureManagerAnim<'a> {
    pub texture: Option<AnimatedTexture<'a>>,
    pub texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

#[allow(dead_code)]
impl<'a> TextureManagerAnim<'a> {
    pub fn new(texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        Self {
            texture: None,
            texture_creator,
        }
    }

    pub fn load_texture(&mut self, path: &Path, frame_width: u32, frame_height: u32, frame_delay: u32) -> Result<(), String> {
        let texture = self.texture_creator.load_texture(path)?;
        let sprite_sheet = SpriteSheet::new(texture, frame_width, frame_height);

        let animated_texture = AnimatedTexture::new(sprite_sheet, frame_delay);
        self.texture = Some(animated_texture);

        Ok(())
    }

    pub fn render_texture(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, dest: sdl2::rect::Rect) -> Result<(), String> {
        if let Some(texture) = &mut self.texture {
            texture.render(canvas, dest)?;
            Ok(())
        } else {
            Err("Texture not loaded".to_owned())
        }
    }
}

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

    pub fn load_texture(&mut self, path: &Path, frame_width: u32, frame_height: u32, frame_delay: u32) -> Result<(), String> {
        self.texture_manager_anim.load_texture(path, frame_width, frame_height, frame_delay)
    }

    pub fn render_texture(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, scale: u32) -> Result<(), String> {
        let dest = sdl2::rect::Rect::new(self.position.x, self.position.y, self.texture_manager_anim.texture.as_ref().unwrap().sprite_sheet.frame_width * scale, self.texture_manager_anim.texture.as_ref().unwrap().sprite_sheet.frame_height * scale);
        self.texture_manager_anim.render_texture(canvas, dest)
    }

    pub fn update_position(&mut self, event: sdl2::event::Event, colliders: &Vec<Rect>, delta_time: f32) {
        let mut new_position = self.position;

        match event {
            sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                match keycode {
                    sdl2::keyboard::Keycode::Left => { new_position.x -= 1; self.rigid_body.apply_force(Vector2::new(-new_position.x as f32, 0.0)); },
                    sdl2::keyboard::Keycode::Right => { new_position.x += 1; self.rigid_body.apply_force(Vector2::new(new_position.x as f32, 0.0)); },
                    sdl2::keyboard::Keycode::Up => { new_position.y -= 1; self.rigid_body.apply_force(Vector2::new(0.0, -new_position.y as f32)); },
                    sdl2::keyboard::Keycode::Down => { new_position.y += 1; self.rigid_body.apply_force(Vector2::new(0.0, new_position.y as f32)); },
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

pub type TextureGrid<'a> = Vec<Vec<TextureManager<'a>>>;

pub struct Tile<'a> {
    pub textures: Vec<&'a TextureManager<'a>>,
    pub tile_map: Vec<Vec<u32>>,
    pub colliders: Vec<Rect>,
    pub texture_grid: Option<TextureGrid<'a>>,
}

#[allow(dead_code)]
impl<'a> Tile<'a> {
    pub fn new(tile_map_path: &Path, textures: Vec<& 'a TextureManager<'a>>, texture_grid: Option<TextureGrid<'a>>) -> Result<Self, Box<dyn std::error::Error>> {
        let tile_map_string = std::fs::read_to_string(tile_map_path).map_err(|e| e.to_string())?;
        let mut tile_map: Vec<Vec<u32>> = Vec::new();
        let mut colliders: Vec<Rect> = Vec::new();

        for (y, line) in tile_map_string.lines().enumerate() {
            let row: Vec<u32> = line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            for (x, &tile_type) in row.iter().enumerate() {
                if tile_type == 2 {
                    let collider = Rect::new((x * 82) as i32, (y * 82) as i32, 82, 82);
                    colliders.push(collider);
                }
            }
            tile_map.push(row);
        }
        Ok(Self { textures, tile_map, colliders, texture_grid })
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, tile_size: (u32, u32)) -> Result<(), String> {
        let (tile_width, tile_height) = tile_size;
        let rows = self.tile_map.len();
        let cols = self.tile_map[0].len();
        for y in 0..rows {
            for x in 0..cols {
                let tile_index = self.tile_map[y][x] as usize;
                let texture_manager = &self.textures[tile_index];
                let dest = sdl2::rect::Rect::new(
                    (x * tile_width as usize) as i32,
                    (y * tile_height as usize) as i32,
                    tile_width,
                    tile_height,
                );
                texture_manager.render_texture(canvas, dest)?;
            }
        }
        Ok(())
    }

    pub fn set_texture_grid(&mut self, texture_grid: TextureGrid<'a>) {
        self.texture_grid = Some(texture_grid);
    }
}

// camera
pub struct Camera {
    pub position: Vector2<i32>,
    pub size: Vector2<u32>,
}

impl Serialize for Camera {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Camera", 2)?;
        state.serialize_field("position", &[self.position.x, self.position.y])?;
        state.serialize_field("size", &[self.size.x, self.size.y])?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Position, Size }

        struct CameraVisitor;

        impl<'de> serde::de::Visitor<'de> for CameraVisitor {
            type Value = Camera;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Camera")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Camera, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut position = None;
                let mut size = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Position => {
                            let coords: [i32; 2] = map.next_value()?;
                            position = Some(Vector2::new(coords[0], coords[1]));
                        }
                        Field::Size => {
                            let coords: [i32; 2] = map.next_value()?;
                            size = Some(Vector2::new(coords[0] as u32, coords[1] as u32));
                        }
                    }
                }
                let position = position.ok_or_else(|| serde::de::Error::missing_field("position"))?;
                let size = size.ok_or_else(|| serde::de::Error::missing_field("size"))?;
                Ok(Camera { position, size })
            }
        }

        const FIELDS: &'static [&'static str] = &["position", "size"];
        deserializer.deserialize_struct("Camera", FIELDS, CameraVisitor)
    }
}

#[allow(dead_code)]
impl Camera {
    pub fn new(position: Vector2<i32>, size: Vector2<u32>) -> Self {
        Self { position, size }
    }

    pub fn update(&mut self, target_position: Vector2<i32>) {
        self.position.x = target_position.x - self.size.x as i32 / 2;
        self.position.y = target_position.y - self.size.y as i32 / 2;
    }

    pub fn transform_rect(&self, rect: Rect) -> Rect {
        Rect::new(rect.x() - self.position.x, rect.y() - self.position.y, rect.width(), rect.height())
    }
}


// light & shadow
pub struct PointLight<'a> {
    texture_manager: TextureManager<'a>,
    position: Vector2<i32>,
}

#[allow(dead_code)]
impl<'a> PointLight<'a> {
    pub fn new(texture_manager: TextureManager<'a>, position: Vector2<i32>) -> Self {
        Self {
            texture_manager,
            position,
        }
    }

    pub fn load_texture(&mut self, path: &Path) -> Result<(), String> {
        self.texture_manager.load_texture(path)
    }

    pub fn render_texture(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let dest = sdl2::rect::Rect::new(self.position.x, self.position.y, 128, 128);
        self.texture_manager.render_texture(canvas, dest)
    }
}


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
    fn new(x: f32, y: f32, x_vel: f32, y_vel: f32, life: f32, color: sdl2::pixels::Color) -> Self {
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

// profiler

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

// UI -> Customizable
    // Button +
    // Radio button
    // Checkbox +
    // Slider +
    // Text box +
// UI Layer
pub struct Layer<'a> {
    pub buttons: Vec<std::rc::Rc<Button<'a>>>,
    pub checkboxes: Vec<&'a mut Checkbox<'a>>,
    // Add other UI elements as needed
}

#[allow(dead_code)]
impl<'a> Layer<'a> {
    pub fn new() -> Self {
        Self { buttons: Vec::new(), checkboxes: Vec::new() }
    }

    pub fn add_button(&mut self, button: std::rc::Rc<Button<'a>>) {
        self.buttons.push(button);
    }

    pub fn add_checkbox(&mut self, checkbox: &'a mut Checkbox<'a>) {
        self.checkboxes.push(checkbox);
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        for button in &self.buttons {
            button.render(canvas)?;
        }
        Ok(())
    }

    pub fn handle_mouse_click(&mut self, x: i32, y: i32) {
        for button in &self.buttons {
            if button.is_pressed(x, y) {
                button.on_click();
            }
        }

        for checkbox in &mut self.checkboxes {
            if checkbox.is_pressed(x, y) {
                checkbox.toggle();
            }
        }
    }
}

#[allow(dead_code)]
pub struct Button<'a> {
    text_box: std::rc::Rc<TextBox<'a>>,
    color: sdl2::pixels::Color,
    bg_rect: sdl2::rect::Rect, // Add this field
    on_click_callback: Box<dyn Fn() + 'a>, // Add this field
}

struct RcTextBoxWrapper<'a>(std::rc::Rc<TextBox<'a>>);

impl<'a> Serialize for RcTextBoxWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        // Here we just dereference `Rc<TextBox>` to `TextBox`
        // and then call `serialize` on it.
        TextBox::serialize(&*self.0, serializer)
    }
}

#[derive(Debug)]
struct ColorWrapper(sdl2::pixels::Color);

impl Serialize for ColorWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (r, g, b) = (self.0.r, self.0.g, self.0.b);
        let mut state = serializer.serialize_struct("ColorWrapper", 3)?;
        state.serialize_field("r", &r)?;
        state.serialize_field("g", &g)?;
        state.serialize_field("b", &b)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ColorWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ColorFields {
            r: u8,
            g: u8,
            b: u8,
        }

        let fields = ColorFields::deserialize(deserializer)?;
        Ok(ColorWrapper(sdl2::pixels::Color::RGB(fields.r, fields.g, fields.b)))
    }
}

impl<'a> Serialize for Button<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Button", 3)?;
        state.serialize_field("text_box", &RcTextBoxWrapper(self.text_box.clone()))?;
        state.serialize_field("color", &ColorWrapper(self.color))?;
        state.serialize_field("bg_rect", &RectWrapper(self.bg_rect))?;
        // We cannot serialize the on_click_callback, so we will skip it.
        state.end()
    }
}

impl<'a> Button<'a> {
    pub fn new(text_box: std::rc::Rc<TextBox<'a>>, color: sdl2::pixels::Color, bg_rect: sdl2::rect::Rect, on_click_callback: Box<dyn Fn() + 'a>,) -> Self {
        Self { text_box, color, bg_rect, on_click_callback, }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.bg_rect)?;
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 0)); // Reset the draw color
        self.text_box.render(canvas)
    }

    pub fn is_pressed(&self, x: i32, y: i32) -> bool {
        self.bg_rect.contains_point(sdl2::rect::Point::new(x, y))
    }

    pub fn on_click(&self) {
        (self.on_click_callback)();
    }
}

#[derive(Debug)]
struct RectWrapper(sdl2::rect::Rect);

impl Serialize for RectWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (x, y, w, h) = (self.0.x(), self.0.y(), self.0.width(), self.0.height());
        let mut tuple = serializer.serialize_tuple(4)?;
        tuple.serialize_element(&x)?;
        tuple.serialize_element(&y)?;
        tuple.serialize_element(&w)?;
        tuple.serialize_element(&h)?;
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for RectWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RectVisitor;

        impl<'de> serde::de::Visitor<'de> for RectVisitor {
            type Value = RectWrapper;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a tuple with 4 elements representing the SDL2 Rect")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<RectWrapper, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let x = seq.next_element()?.unwrap();
                let y = seq.next_element()?.unwrap();
                let w = seq.next_element()?.unwrap();
                let h = seq.next_element()?.unwrap();
                Ok(RectWrapper(sdl2::rect::Rect::new(x, y, w, h)))
            }
        }

        deserializer.deserialize_tuple(4, RectVisitor)
    }
}

pub struct TextBox<'a> {
    pub text: String,
    pub font: std::sync::Arc<sdl2::ttf::Font<'a, 'static>>,
    pub rect: sdl2::rect::Rect,
}

impl<'a> Serialize for TextBox<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("TextBox", 3)?;
        state.serialize_field("text", &self.text)?;
        state.serialize_field("rect", &RectWrapper(self.rect))?;
        // We cannot serialize the font, so we will skip it.
        state.end()
    }
}

#[allow(dead_code)]
impl<'a> TextBox<'a> {
    pub fn new(text: String, font: std::sync::Arc<sdl2::ttf::Font<'a, 'static>>, rect: sdl2::rect::Rect) -> Self {
        Self { text, font, rect }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let surface = self.font.render(&self.text).blended(sdl2::pixels::Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, self.rect)?;
        Ok(())
    }
}

#[allow(dead_code)]
pub struct Checkbox<'a> {
    pub button: Button<'a>,
    pub checked: bool,
}

#[allow(dead_code)]
impl<'a> Checkbox<'a> {
    pub fn new(
        text_box: std::rc::Rc<TextBox<'a>>,
        color: sdl2::pixels::Color,
        bg_rect: sdl2::rect::Rect,
    ) -> Self {
        let button = Button::new(
            text_box,
            color,
            bg_rect,
            Box::new(|| {
                println!("Checkbox clicked!");
            }),
        );
        Self { button, checked: false }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        self.button.render(canvas)?;

        if self.checked {
            let checkmark_rect = sdl2::rect::Rect::new(
                self.button.bg_rect.x() + self.button.bg_rect.width() as i32 / 4,
                self.button.bg_rect.y() + self.button.bg_rect.height() as i32 / 4,
                self.button.bg_rect.width() / 2,
                self.button.bg_rect.height() / 2,
            );
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            canvas.fill_rect(checkmark_rect)?;
        }

        Ok(())
    }

    pub fn is_pressed(&self, x: i32, y: i32) -> bool {
        self.button.is_pressed(x, y)
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
        (self.button.on_click_callback)();
    }
}

pub struct Slider<'a> {
    pub background_rect: sdl2::rect::Rect,
    pub slider_rect: sdl2::rect::Rect,
    pub slider_color: sdl2::pixels::Color,
    pub background_color: sdl2::pixels::Color,
    pub on_value_changed_callback: Box<dyn Fn(f32) + 'a>,
    pub value: f32,
}

#[allow(dead_code)]
impl<'a> Slider<'a> {
    pub fn new(
        background_rect: sdl2::rect::Rect,
        slider_rect: sdl2::rect::Rect,
        slider_color: sdl2::pixels::Color,
        background_color: sdl2::pixels::Color,
        on_value_changed_callback: Box<dyn Fn(f32) + 'a>,
    ) -> Self {
        Self {
            background_rect,
            slider_rect,
            slider_color,
            background_color,
            on_value_changed_callback,
            value: 0.0,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(self.background_color);
        canvas.fill_rect(self.background_rect)?;
        canvas.set_draw_color(self.slider_color);
        canvas.fill_rect(self.slider_rect)?;
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 0)); // Reset the draw color
        Ok(())
    }

    pub fn handle_mouse_click(&mut self, x: i32, y: i32) {
        if self.background_rect.contains_point(sdl2::rect::Point::new(x, y)) {
            let new_x = x - self.slider_rect.width() as i32 / 2;
            let max_x = self.background_rect.x() + self.background_rect.width() as i32 - self.slider_rect.width() as i32;
            let min_x = self.background_rect.x();
            self.slider_rect.set_x(new_x.clamp(min_x, max_x));
            self.update_value();
        }
    }

    pub fn update_value(&mut self) {
        let relative_x = self.slider_rect.x() - self.background_rect.x();
        let width_diff = self.background_rect.width() - self.slider_rect.width();
        self.value = relative_x as f32 / width_diff as f32;
        (self.on_value_changed_callback)(self.value);
    }
}


// Audio -> play .wav, .ogg
    // Volume
    // Play
    // Stop
    // Loop
const CHUNK_SIZE: i32 = 1024;
const FREQUENCY: i32 = 44_100;

pub struct AudioPlayer {
    pub mixer_context: sdl2::mixer::Sdl2MixerContext,
}

#[allow(dead_code)]
impl AudioPlayer {
    pub fn new(numchans: i32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let _audio = sdl_context.audio().unwrap();
        let mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3 | sdl2::mixer::InitFlag::FLAC | sdl2::mixer::InitFlag::MOD | sdl2::mixer::InitFlag::OGG).unwrap();

        sdl2::mixer::open_audio(FREQUENCY, AUDIO_S16LSB, DEFAULT_CHANNELS, CHUNK_SIZE).unwrap();
        sdl2::mixer::allocate_channels(numchans);

        AudioPlayer { mixer_context }
    }

    pub fn play(&self, file_path: &Path, loops: i32, volume: i32) -> Channel {
        let audio_chunk = sdl2::mixer::Chunk::from_file(file_path).unwrap();
        let channel = sdl2::mixer::Channel::all().play(&audio_chunk, loops).unwrap();
        channel.set_volume(volume);
        channel
    }

    pub fn pause(&self, channel: Channel) {
        channel.pause();
    }

    pub fn resume(&self, channel: Channel) {
        channel.resume();
    }

    pub fn stop(&self, channel: Channel) {
        channel.halt();
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        sdl2::mixer::Music::hook_finished(|| println!("Music is ended"))
    }
}

// AI system +
    // Behaviour Tree (i.e. follow -> hide -> attack -> hide -> ...) +
        // Component system where we define behaviour and put into Behaviour Tree
#[allow(dead_code)]
pub enum BehaviourTreeNode<'a> {
    Action(Box<dyn Fn() -> BehaviourTreeResult + 'a>),
    Selector(Vec<BehaviourTreeNode<'a>>),
    Sequence(Vec<BehaviourTreeNode<'a>>),
}

#[allow(dead_code)]
pub enum BehaviourTreeResult {
    Success,
    Failure,
    Running,
}
#[allow(dead_code)]
impl<'a> BehaviourTreeNode<'a> {
    fn tick(&self) -> BehaviourTreeResult {
        match self {
            BehaviourTreeNode::Action(action) => action(),
            BehaviourTreeNode::Selector(nodes) => {
                for node in nodes {
                    match node.tick() {
                        BehaviourTreeResult::Success => return BehaviourTreeResult::Success,
                        BehaviourTreeResult::Running => return BehaviourTreeResult::Running,
                        BehaviourTreeResult::Failure => {}
                    }
                }
                BehaviourTreeResult::Failure
            }
            BehaviourTreeNode::Sequence(nodes) => {
                for node in nodes {
                    match node.tick() {
                        BehaviourTreeResult::Failure => return BehaviourTreeResult::Failure,
                        BehaviourTreeResult::Running => return BehaviourTreeResult::Running,
                        BehaviourTreeResult::Success => {}
                    }
                }
                BehaviourTreeResult::Success
            }
        }
    }
}        

// Window system
pub struct Window {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: Canvas<sdl2::video::Window>,
}

#[allow(dead_code)]
impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .opengl()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self { sdl_context, video_subsystem, canvas })
    }
}

// Input handler
pub struct InputHandler {
    event_pump: EventPump,
    controller: Option<GameController>,
    keys_pressed: HashSet<Keycode>,
    mouse_button_pressed: bool,
    mouse_position: (i32, i32),
}

#[allow(dead_code)]
impl InputHandler {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, Box<dyn std::error::Error>> {
        let event_pump = sdl_context.event_pump()?;
        
        let mut controller = None;
        let game_controller_subsystem = sdl_context.game_controller()?;
        for i in 0..game_controller_subsystem.num_joysticks()? {
            if game_controller_subsystem.is_game_controller(i) {
                controller = game_controller_subsystem.open(i).ok();
                break;
            }
        }
        
        Ok(Self {
            event_pump,
            controller,
            keys_pressed: HashSet::new(),
            mouse_button_pressed: false,
            mouse_position: (0, 0),
        })
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        self.mouse_button_pressed = false;
        
        let mut events = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.keys_pressed.insert(keycode);
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    self.keys_pressed.remove(&keycode);
                },
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        self.mouse_position = (x, y);
                        self.mouse_button_pressed = true;
                    }
                },
                _ => {}
            }
            events.push(event);
        }

        events
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.keys_pressed.contains(&keycode)
    }

    pub fn is_button_pressed(&self, button: sdl2::controller::Button) -> bool {
        if let Some(controller) = &self.controller {
            controller.button(button)
        } else {
            false
        }
    }

    pub fn is_mouse_button_pressed(&self) -> bool {
        self.mouse_button_pressed
    }

    pub fn get_mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }

    pub fn get_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }
}

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

// Timer
pub struct Timer {
    start_time: std::time::Instant,
    last_frame_time: std::time::Instant,
    delta_time: std::time::Duration,
    frames: Vec<std::time::Duration>,
}

#[allow(dead_code)]
impl Timer {
    pub fn new() -> Self {
        let now = std::time::Instant::now();
        Self {
            start_time: now,
            last_frame_time: now,
            delta_time: std::time::Duration::new(0, 0),
            frames: Vec::new(),
        }
    }

    // Measures the time between two frames.
    pub fn step(&mut self) {
        let now = std::time::Instant::now();
        self.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
        self.frames.push(self.delta_time);

        while self.frames.len() > 60 {
            self.frames.remove(0);
        }
    }

    // Returns the time between the last two frames.
    pub fn get_delta(&self) -> std::time::Duration {
        self.delta_time
    }

    // Returns the average delta time over the last second.
    pub fn get_average_delta(&self) -> std::time::Duration {
        let total: std::time::Duration = self.frames.iter().sum();
        total / self.frames.len() as u32
    }

    // Returns the precise amount of time since some time in the past.
    pub fn get_time(&self) -> std::time::Duration {
        std::time::Instant::now() - self.start_time
    }

    // Returns the value of a timer with microsecond precision.
    pub fn get_micro_time(&self) -> u128 {
        self.get_time().as_micros()
    }

    // Returns the current frames per second.
    pub fn get_fps(&self) -> f32 {
        1.0 / self.get_average_delta().as_secs_f32()
    }

    // Pauses the current thread for the specified amount of time.
    pub fn sleep(&self, duration: std::time::Duration) {
        std::thread::sleep(duration);
    }
}

#[allow(dead_code)]
struct DialogueOption {
    pub text: String,
    pub action: fn(),
}