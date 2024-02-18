mod two_d;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = two_d::Window::new("My Game", 800, 600, false)?;

    let last_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
    let mut current_frame_time;
    let mut delta_time;

    let texture_creator = window.canvas.texture_creator();

    let texture_manager = two_d::TextureManagerAnim::new(&texture_creator);

    // Initialize InputHandler
    let mut input_handler = two_d::InputHandler::new(&window.sdl_context)?;

    let mut player = two_d::GameObject::new(texture_manager, nalgebra::Vector2::new(50, 50));
    player.load_texture("idle", std::path::Path::new("character_idle_anim.png"), 16, 18, 150, 0)?;
    player.load_texture("walk_down", std::path::Path::new("character_walk_anim.png"), 16, 18, 150, 0)?;
    player.load_texture("walk_up", std::path::Path::new("character_walk_anim.png"), 16, 17, 150, 1)?;
    player.load_texture("walk_right", std::path::Path::new("character_walk_anim.png"), 16, 17, 150, 2)?;

    let mut t1 = two_d::TextureManager::new(&texture_creator);
    t1.load_texture(&std::path::Path::new("NinjaAdventure\\Backgrounds\\Tilesets\\TilesetField_1.png"))?;
    let mut t2 = two_d::TextureManager::new(&texture_creator);
    t2.load_texture(&std::path::Path::new("NinjaAdventure\\Backgrounds\\Tilesets\\TilesetField_2.png"))?;
    let mut t3 = two_d::TextureManager::new(&texture_creator);
    t3.load_texture(&std::path::Path::new("NinjaAdventure\\Backgrounds\\Tilesets\\TilesetHouse_1.png"))?;

    let tile_map = two_d::Tile::new(std::path::Path::new("map.txt"), vec![
        &t1, &t2, &t3
        // Add more TextureManager objects for each tile type you want to render
    ], None)?;

    // Create a camera object
    let mut camera = two_d::Camera::new(nalgebra::Vector2::new(0, 0), nalgebra::Vector2::new(800, 600));

    let mut flip_horizontal = false; // Create a boolean flag to keep track of the flip state

    let mut profiler = two_d::Profiler::new();

    let mut particles: Vec<two_d::Particle> = Vec::new();

    'mainloop: loop {
        current_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
        delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;

        // Update the profiler
        profiler.update(current_frame_time);

        if input_handler.is_mouse_button_pressed() {
            let (x, y) = input_handler.get_mouse_position();
            for _ in 0..5 {
                two_d::spawn_particles_fires(&mut particles, x, y, 10, two_d::ParticleShape::Rect);
            }
        }   

        for event in input_handler.poll_events() {
            if let Some(event) = two_d::from_sdl_event(event) {
                match event {
                    two_d::GEvent::Quit | two_d::GEvent::KeyDown(two_d::KeyEvent::Escape) => break 'mainloop,
                    two_d::GEvent::KeyDown(ref key_event) => {
                        match key_event {
                            two_d::KeyEvent::Left => {
                                flip_horizontal = true;
                                player.texture_manager_anim.set_animation("walk_right");
                            },
                            two_d::KeyEvent::Right => {
                                flip_horizontal = false;
                                player.texture_manager_anim.set_animation("walk_right");
                            },
                            two_d::KeyEvent::Up => {
                                flip_horizontal = false;
                                player.texture_manager_anim.set_animation("walk_up");
                            }, 
                            two_d::KeyEvent::Down => {
                                flip_horizontal = false;
                                player.texture_manager_anim.set_animation("walk_down");
                            },
                            _ => {},
                        }
                        player.update_position(event, &tile_map.colliders, delta_time);

                        particles.retain(|particle| particle.life > 0.0);  // Remove dead particles.
                        for particle in &mut particles {
                            particle.update(delta_time, 600);  // Assuming 600 is the screen height.
                        }
                    },
                    two_d::GEvent::KeyUp(key_event) => {
                        match key_event {
                            two_d::KeyEvent::Left | two_d::KeyEvent::Right | two_d::KeyEvent::Up | two_d::KeyEvent::Down => {
                                player.texture_manager_anim.set_animation("idle");
                            },
                            _ => {},
                        }
                    }
                }
            }
        }

        // Update the camera's position to follow the player
        camera.update(player.get_position());

        window.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        window.canvas.clear();

        // Update and render particles
        //let delta_time = 0.016; // Assuming ~60 FPS
        particles.retain(|p| p.life > 0.0);
        for particle in &mut particles {
            particle.update(0.016, 600);
            particle.render(&mut window.canvas);
        }

        // Render the tile map
        for y in 0..tile_map.tile_map.len() {
            for x in 0..tile_map.tile_map[0].len() {
                let tile_index = tile_map.tile_map[y][x] as usize;
                let texture_manager = &tile_map.textures[tile_index];
                let rect = two_d::Rect::new((x * 82) as i32, (y * 82) as i32, 82, 82);
                let transformed_rect = camera.transform_rect(&rect);
                texture_manager.render_texture(&mut window.canvas, transformed_rect.unwrap())?;
            }
        }

        // Render the player
        if let Some(current_animation_tag) = &player.texture_manager_anim.current_animation {
            if let Some(animated_texture) = player.texture_manager_anim.animations.get(current_animation_tag) {
                let player_rect = two_d::Rect::new(
                    player.position.x, 
                    player.position.y, 
                    animated_texture.sprite_sheet.frame_width * 2, 
                    animated_texture.sprite_sheet.frame_height * 2
                );
                let transformed_player_rect = camera.transform_rect(&player_rect);
                player.texture_manager_anim.render_texture(&mut window.canvas, transformed_player_rect.unwrap(), flip_horizontal as u32)?;
            }
        }

        window.canvas.present();
    }
    Ok(())
}