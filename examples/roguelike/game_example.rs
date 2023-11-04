mod two_d;
use rand::Rng;
use rand::seq::SliceRandom; // For random selection from slices
#[allow(unused_imports)]
use sdl2::image::LoadTexture;

struct Enemy<'a> {
    position: nalgebra::Vector2<i32>,
    texture_manager: two_d::TextureManagerAnim<'a>,
}

// A function to check if the given position collides with any enemy.
fn check_collision_with_enemies(enemies: &[Enemy], new_position: (usize, usize)) -> bool {
    enemies.iter().any(|enemy| {
        let enemy_grid_position = (
            (enemy.position.x / 82) as usize,
            (enemy.position.y / 82) as usize,
        );
        new_position == enemy_grid_position
    })
}

fn generate_level(width: usize, height: usize, player_pos: (usize, usize), door_pos: (usize, usize)) -> (Vec<Vec<u32>>, Vec<(usize, usize)>) {
    let mut rng = rand::thread_rng();
    let mut grid = vec![vec![0; width]; height];  // Initialize grid with empty spaces
    let mut spawn_points = Vec::new();
    
    // Set the perimeter walls
    for x in 0..width {
        grid[0][x] = 2;
        grid[height-1][x] = 2;
    }
    for y in 0..height {
        grid[y][0] = 2;
        grid[y][width-1] = 2;
    }

    // Set the door position
    grid[door_pos.0][door_pos.1] = 0;
    
    // Fill the interior with random walls and obstacles, avoiding the player and door positions
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if (x, y) != player_pos && (x, y) != door_pos {
                grid[y][x] = if rng.gen_bool(0.2) { 2 } else { 0 };  // 20% chance of a wall, adjust as needed
            }
        }
    }

    // Collect valid spawn points
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if (x, y) != player_pos && (x, y) != door_pos && grid[y][x] == 0 {
                spawn_points.push((x, y));
            }
        }
    }

    // grid
    (grid, spawn_points)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = two_d::Window::new("My Game", 800, 600)?;

    let texture_creator = window.canvas.texture_creator();

    let texture_manager = two_d::TextureManagerAnim::new(&texture_creator);

    let mut input_handler = two_d::InputHandler::new(&window.sdl_context)?;

    // Assuming each tile is 32x32 pixels
    const TILE_SIZE: i32 = 82;
    let mut player_grid_position = (2, 2);  // Grid position (x, y)

    let mut player = two_d::GameObject::new(texture_manager, nalgebra::Vector2::new(player_grid_position.0 as i32 * TILE_SIZE as i32, player_grid_position.1 as i32 * TILE_SIZE as i32));

    player.load_texture("idle", std::path::Path::new("character_idle_anim.png"), 16, 18, 150, 0)?;
    player.load_texture("walk_down", std::path::Path::new("character_walk_anim.png"), 16, 18, 150, 0)?;
    player.load_texture("walk_up", std::path::Path::new("character_walk_anim.png"), 16, 17, 150, 1)?;
    player.load_texture("walk_right", std::path::Path::new("character_walk_anim.png"), 16, 17, 150, 2)?;

    let mut floor = two_d::TextureManager::new(&texture_creator);
    floor.load_texture(&std::path::Path::new("ground.png"))?;
    let mut wall = two_d::TextureManager::new(&texture_creator);
    wall.load_texture(&std::path::Path::new("door.png"))?;
    let mut obstacle = two_d::TextureManager::new(&texture_creator);
    obstacle.load_texture(&std::path::Path::new("stone.png"))?;

    let (generated_map, mut spawn_points) = generate_level(10, 10, player_grid_position, (1, 8));
    let mut rng = rand::thread_rng();
    let mut enemies = Vec::new();

    for _ in 0..5 {
        if let Some(spawn_point) = spawn_points.choose(&mut rng).cloned() {
            let mut enemy_texture_manager = two_d::TextureManagerAnim::new(&texture_creator);
            // Load enemy textures here. This should be adapted to your actual texture loading logic.
            // For example:
            enemy_texture_manager.load_animation("enemy_idle", std::path::Path::new("player_anim.png"), 16, 18, 150, 0)?;

            let enemy = Enemy {
                position: nalgebra::Vector2::new(spawn_point.0 as i32 * TILE_SIZE, spawn_point.1 as i32 * TILE_SIZE),
                texture_manager: enemy_texture_manager,
            };
            enemies.push(enemy);
            spawn_points.retain(|&p| p != spawn_point); // Remove the used spawn point
        }
    }

    let tile_map = two_d::Tile::from_generated_map(
        generated_map,
        vec![&floor, &wall, &obstacle],
        None,
    )?;

    let mut camera = two_d::Camera::new(nalgebra::Vector2::new(0, 0), nalgebra::Vector2::new(800, 600));

    let mut flip_horizontal = false;

    let mut light_spot_texture = texture_creator.load_texture("point_light.png")?;
    let light = two_d::PointLight::new(
        nalgebra::Vector2::new(400.0, 300.0),
        100.0,
        0.6,  // Intensity: 0.0 (off) to 1.0 (full intensity)
        sdl2::pixels::Color::RGB(255, 255, 255)  // White color for pure light. You can change this!
    );
    let mut darkness_texture = texture_creator.create_texture_target(None, 800, 600)?;
    darkness_texture.set_blend_mode(sdl2::render::BlendMode::Mod);

    // Key press state tracking
    let mut left_key_pressed = false;
    let mut right_key_pressed = false;
    let mut up_key_pressed = false;
    let mut down_key_pressed = false;

    'mainloop: loop {

        for event in input_handler.poll_events() {
            if let Some(event) = two_d::from_sdl_event(event) {
                match event {
                    two_d::GEvent::Quit | two_d::GEvent::KeyDown(two_d::KeyEvent::Escape) => break 'mainloop,
                    two_d::GEvent::KeyDown(ref key_event) => {
                        match key_event {
                            // Check collisions for left movement
                        two_d::KeyEvent::Left if !left_key_pressed => {
                            let new_position = (player_grid_position.0 - 1, player_grid_position.1);
                            if player_grid_position.0 > 0
                                && tile_map.tile_map[new_position.1][new_position.0] == 0
                                && !check_collision_with_enemies(&enemies, new_position)
                            {
                                left_key_pressed = true;
                                player_grid_position = new_position;
                                flip_horizontal = true;
                                player.texture_manager_anim.set_animation("walk_right");
                            }
                        },

                        // Check collisions for right movement
                        two_d::KeyEvent::Right if !right_key_pressed => {
                            let new_position = (player_grid_position.0 + 1, player_grid_position.1);
                            if player_grid_position.0 < tile_map.tile_map[0].len() - 1
                                && tile_map.tile_map[new_position.1][new_position.0] == 0
                                && !check_collision_with_enemies(&enemies, new_position)
                            {
                                right_key_pressed = true;
                                player_grid_position = new_position;
                                flip_horizontal = false;
                                player.texture_manager_anim.set_animation("walk_right");
                            }
                        },

                        // Check collisions for up movement
                        two_d::KeyEvent::Up if !up_key_pressed => {
                            let new_position = (player_grid_position.0, player_grid_position.1 - 1);
                            if player_grid_position.1 > 0
                                && tile_map.tile_map[new_position.1][new_position.0] == 0
                                && !check_collision_with_enemies(&enemies, new_position)
                            {
                                up_key_pressed = true;
                                player_grid_position = new_position;
                                player.texture_manager_anim.set_animation("walk_up");
                            }
                        },

                        // Check collisions for down movement
                        two_d::KeyEvent::Down if !down_key_pressed => {
                            let new_position = (player_grid_position.0, player_grid_position.1 + 1);
                            if player_grid_position.1 < tile_map.tile_map.len() - 1
                                && tile_map.tile_map[new_position.1][new_position.0] == 0
                                && !check_collision_with_enemies(&enemies, new_position)
                            {
                                down_key_pressed = true;
                                player_grid_position = new_position;
                                player.texture_manager_anim.set_animation("walk_down");
                            }
                        },
                        _ => {},
                        }
                    },
                    two_d::GEvent::KeyUp(ref key_event) => {
                        match key_event {
                            two_d::KeyEvent::Left | two_d::KeyEvent::Right | two_d::KeyEvent::Up | two_d::KeyEvent::Down => {
                                left_key_pressed = false;
                                right_key_pressed = false;
                                up_key_pressed = false;
                                down_key_pressed = false;
                                player.texture_manager_anim.set_animation("idle");
                            },
                            _ => {},
                        }
                    }
                }
            }
        }

        // Update player pixel position
        player.position = nalgebra::Vector2::new(player_grid_position.0 as i32 * TILE_SIZE as i32, player_grid_position.1 as i32 * TILE_SIZE as i32);

        // Update camera position to follow player
        camera.update(player.get_position());

        window.canvas.clear();

        // Render tile map
        for y in 0..tile_map.tile_map.len() {
            for x in 0..tile_map.tile_map[0].len() {
                let tile_index = tile_map.tile_map[y][x] as usize;
                let texture_manager = &tile_map.textures[tile_index];
                let rect = sdl2::rect::Rect::new((x as i32 * TILE_SIZE as i32) as i32, (y as i32 * TILE_SIZE as i32) as i32, TILE_SIZE as u32, TILE_SIZE as u32);
                let transformed_rect = camera.transform_rect(rect);
                texture_manager.render_texture(&mut window.canvas, transformed_rect)?;
            }
        }

        // Render player
        if let Some(current_animation_tag) = &player.texture_manager_anim.current_animation {
            if let Some(animated_texture) = player.texture_manager_anim.animations.get(current_animation_tag) {
                let player_rect = sdl2::rect::Rect::new(
                    player.position.x as i32, 
                    player.position.y as i32, 
                    animated_texture.sprite_sheet.frame_width * 2, 
                    animated_texture.sprite_sheet.frame_height * 2
                );
                let transformed_player_rect = camera.transform_rect(player_rect);
                player.texture_manager_anim.render_texture(&mut window.canvas, transformed_player_rect, flip_horizontal as u32)?;
            }
        }

        // Render enemies inside the game loop
        for enemy in &mut enemies {
            // Render the enemy using its texture and position
            // Adapt the below code to match how your engine handles enemy animations and rendering
            if let Some(current_animation_tag) = &enemy.texture_manager.current_animation {
                if let Some(animated_texture) = enemy.texture_manager.animations.get(current_animation_tag) {
                    let player_rect = sdl2::rect::Rect::new(
                        enemy.position.x as i32, 
                        enemy.position.y as i32, 
                        animated_texture.sprite_sheet.frame_width * 2, 
                        animated_texture.sprite_sheet.frame_height * 2
                    );
                    let transformed_player_rect = camera.transform_rect(player_rect);
                    enemy.texture_manager.render_texture(&mut window.canvas, transformed_player_rect, 0)?;
                }
            }
        }

        // Render each light onto the light texture
        window.canvas.with_texture_canvas(&mut darkness_texture, |canvas| {
            // Clear the texture with a semi-transparent black for darkness
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 150));
            canvas.clear();
            
            // Render each light onto this dark texture
            light.render(canvas, &mut light_spot_texture);
        })?;

        // Set blend mode to Mod for blending the light texture onto the main scene
        // Now, set the blend mode and render the darkness_texture over the main canvas to achieve the lighting effect
        window.canvas.set_blend_mode(sdl2::render::BlendMode::Mod);
        window.canvas.copy(&darkness_texture, None, None)?;
        window.canvas.set_blend_mode(sdl2::render::BlendMode::None);

        window.canvas.present();
    }
    Ok(())
}
