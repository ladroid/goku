pub struct GameViewport {
    game_object: GameObject<'static>,
    tile: Tile<'static>,
    camera: Camera,
    viewport_size: Vector2<u32>,
}

impl GameViewport {
    pub fn new(mut game_object: GameObject<'static>, tile: Tile<'static>, viewport_size: Vector2<u32>) -> Self {
        let camera = Camera::new(game_object.get_position(), viewport_size);
        Self {
            game_object,
            tile,
            camera,
            viewport_size,
        }
    }

    pub fn update(&mut self, delta_time: f32, event: Option<sdl2::event::Event>) {
        let colliders = vec![]; // You can add your colliders here
        if let Some(event) = event {
            self.game_object.update_position(event, &colliders, delta_time);
        }
        self.camera.update(self.game_object.get_position());
    }

    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        self.tile.render(canvas, (32, 32))?; // Tile size should be adjusted accordingly
        let dest = self.camera.transform_rect(self.game_object.collider);
        self.game_object.render_texture(canvas, dest.width() / 32)?;
        Ok(())
    }
}

// Game viewport window
egui::Window::new("Game Viewport")
    .default_size([game_viewport.viewport_size.x as f32, game_viewport.viewport_size.y as f32])
    .resizable(false)
    .show(ctx, |ui| {
        // You can draw your game viewport here using SDL2 canvas
        // For example:
        // game_viewport.draw(canvas);

        // You can also update your game viewport based on user input, e.g.:
        // game_viewport.update(delta_time, event);
    });