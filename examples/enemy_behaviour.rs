/*
* Assets were take from here -> https://pixel-boy.itch.io/ninja-adventure-asset-pack

Your distribution folder might look like this:

MyGame/
├── assets/
│   ├── images/
│   ├── sounds/
│   └── ...
├── src/
└── Cargo.toml
*/

mod two_d;

pub fn test_top_down() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("My Game", 800, 600)?;

    let last_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
    let mut current_frame_time;
    let mut delta_time;

    let texture_creator = window.canvas.texture_creator();

    let texture_manager = TextureManagerAnim::new(&texture_creator);

    let texture_manager2 = TextureManagerAnim::new(&texture_creator);

    let mut player = GameObject::new(texture_manager, Vector2::new(50, 50));
    player.load_texture(Path::new("player_anim.png"), 30, 30, 150)?;

    let mut enemy = GameObject::new(texture_manager2, Vector2::new(70, 70));
    enemy.load_texture(Path::new("player_anim.png"), 30, 30, 150)?;

    let mut t1 = TextureManager::new(&texture_creator);
    t1.load_texture(&Path::new("NinjaAdventure/Backgrounds/Tilesets/TilesetField_1.png"))?;
    let mut t2 = TextureManager::new(&texture_creator);
    t2.load_texture(&Path::new("NinjaAdventure/Backgrounds/Tilesets/TilesetField_2.png"))?;
    let mut t3 = TextureManager::new(&texture_creator);
    t3.load_texture(&Path::new("NinjaAdventure/Backgrounds/Tilesets/TilesetHouse_1.png"))?;

    let tile_map = Tile::new(Path::new("test_assets/map.txt"), vec![
        &t1, &t2, &t3
        // Add more TextureManager objects for each tile type you want to render
    ])?;

    let mut vec_coll = vec![enemy.collider];
    vec_coll.extend(tile_map.colliders.iter().clone());

    // Create a camera object
    let mut camera = Camera::new(Vector2::new(0, 0), Vector2::new(800, 600));

    let mut enemy_speed = 1.0;  // This is now the actual current speed of your enemy
    let target_speed = 1.0;  // This is the speed your enemy wants to reach
    let acceleration = 0.1;  // This is how quickly your enemy can change speed
    let flee_distance = 100.0;
    let chase_distance = 200.0;

    let mut enemy_chasing = false;

    'mainloop: loop {
        current_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
        delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;

        for event in window.sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {
                    player.update_position(event, &vec_coll, delta_time);
                }
            }
        }  

        // Update the camera's position to follow the player
        camera.update(player.get_position());

        window.canvas.clear();

        let direction_to_player_x = player.get_position()[0] - enemy.get_position()[0];
        let direction_to_player_y = player.get_position()[1] - enemy.get_position()[1];
        let distance_to_player = ((direction_to_player_x.pow(2) + direction_to_player_y.pow(2)) as f32).sqrt();

        let direction_to_player_x = direction_to_player_x as f32 / distance_to_player;
        let direction_to_player_y = direction_to_player_y as f32 / distance_to_player;

        let mut target_velocity_x = 0.0;
        let mut target_velocity_y = 0.0;

        if distance_to_player < flee_distance {
            // Enemy should move in the opposite direction
            target_velocity_x = -direction_to_player_x * target_speed;
            target_velocity_y = -direction_to_player_y * target_speed;
            enemy_chasing = false;
        } else if distance_to_player > chase_distance {
            // Enemy should move towards the player
            target_velocity_x = direction_to_player_x * target_speed;
            target_velocity_y = direction_to_player_y * target_speed;
            enemy_chasing = true;
        }

        // Gradually adjust the enemy's speed towards the target speed
        if enemy_chasing {
            enemy_speed += acceleration * delta_time;
            if enemy_speed > target_speed {
                enemy_speed = target_speed;
            }
        } else {
            enemy_speed -= acceleration * delta_time;
            if enemy_speed < -target_speed {
                enemy_speed = -target_speed;
            }
        }

        // Update the enemy's position
        let enemy_position = enemy.get_position();
        enemy.position[0] = (enemy_position[0] as f32 + target_velocity_x * delta_time) as i32;
        enemy.position[1] = (enemy_position[1] as f32 + target_velocity_y * delta_time) as i32;


        // Render the tile map
        for y in 0..tile_map.tile_map.len() {
            for x in 0..tile_map.tile_map[0].len() {
                let tile_index = tile_map.tile_map[y][x] as usize;
                let texture_manager = &tile_map.textures[tile_index];
                let rect = Rect::new((x * 82) as i32, (y * 82) as i32, 82, 82);
                let transformed_rect = camera.transform_rect(rect);
                texture_manager.render_texture(&mut window.canvas, transformed_rect)?;
            }
        }

        // Render the player
        let player_rect = Rect::new(player.position.x, player.position.y, player.texture_manager_anim.texture.as_ref().unwrap().sprite_sheet.frame_width * 2, player.texture_manager_anim.texture.as_ref().unwrap().sprite_sheet.frame_height * 2);
        let transformed_player_rect = camera.transform_rect(player_rect);
        player.texture_manager_anim.render_texture(&mut window.canvas, transformed_player_rect)?;

        // Render the enemy
        let enemy_rect = Rect::new(enemy.position.x, enemy.position.y, enemy.texture_manager_anim.texture.as_ref().unwrap().sprite_sheet.frame_width * 2, enemy.texture_manager_anim.texture.as_ref().unwrap().sprite_sheet.frame_height * 2);
        let transformed_enemy_rect = camera.transform_rect(enemy_rect);
        enemy.texture_manager_anim.render_texture(&mut window.canvas, transformed_enemy_rect)?;

        window.canvas.present();
    }
    Ok(())
}