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

    let mut light_texture = texture_creator.create_texture_streaming(None, 800, 600)?;
    light_texture.set_blend_mode(sdl2::render::BlendMode::Add);
    let light = two_d::AmbientFilter::new(0.6);  // 1.0 means full intensity

    'mainloop: loop {
        current_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
        delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;

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

        window.canvas.clear();

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
        // Clear the light texture with an ambient color (e.g., dark blue)
        light_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..600 {
                for x in 0..800 {
                    let offset = y * pitch + x * 4;
                    buffer[offset] = 60;       // Blue
                    buffer[offset + 1] = 0;   // Green
                    buffer[offset + 2] = 100;   // Red
                    buffer[offset + 3] = 255;             // Alpha
                }
            }
        }).unwrap();

        // Render each light onto the light texture
        light.render(&mut window.canvas, &mut light_texture);

        // Set blend mode to Mod for blending the light texture onto the main scene
        window.canvas.set_blend_mode(sdl2::render::BlendMode::Mod);
        window.canvas.copy(&light_texture, None, None)?;
        window.canvas.set_blend_mode(sdl2::render::BlendMode::None);

        window.canvas.present();
    }
    Ok(())
}