mod two_d;
use rand::Rng;
use rand::seq::SliceRandom; // For random selection from slices
#[allow(unused_imports)]
use sdl2::image::LoadTexture;

struct Enemy<'a> {
    position: nalgebra::Vector2<i32>,
    grid_position: (i32, i32), // Add a grid_position to keep track of the enemy's position in grid terms
    texture_manager: two_d::TextureManagerAnim<'a>,
    speed: i32, // Speed of the enemy
    last_move: std::time::Instant, // Timestamp of the last move
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

// Helper function to check if the new position of an enemy would collide with any other enemy
fn enemy_collision_check(enemies: &[Enemy], current_index: usize, new_position: (i32, i32)) -> bool {
    for (index, enemy) in enemies.iter().enumerate() {
        if index != current_index && enemy.grid_position == new_position {
            return true;
        }
    }
    false
}

fn generate_level(width: usize, height: usize, player_pos: (usize, usize), door_pos: (usize, usize)) -> (Vec<Vec<u32>>, Vec<(usize, usize)>, (usize, usize)) {
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

    // Determine a random position for the ladder
    let ladder_pos = {
        let mut pos;
        loop {
            pos = (rng.gen_range(1..width - 1), rng.gen_range(1..height - 1));
            if pos != player_pos && pos != door_pos && grid[pos.1][pos.0] == 0 {
                break;
            }
        }
        pos
    };
    grid[ladder_pos.1][ladder_pos.0] = 3;

    // grid
    (grid, spawn_points, ladder_pos)
}

// Add this function to calculate the next move for an enemy to move towards the player
fn move_towards_player(
    player_pos: (i32, i32),
    enemy_pos: (i32, i32),
    enemies: &[Enemy], // Pass the slice of all enemies
    current_index: usize, // The index of the current enemy being moved
    tile_map: &two_d::Tile,
    speed: i32,
    last_move: &mut std::time::Instant,
    player_moved: bool,
) -> (i32, i32) {
    let mut direction = (0, 0);

    if player_moved {
        // Only move if enough time has passed since the last move
        if last_move.elapsed() >= core::time::Duration::from_millis((1000 / speed) as u64) {
            // Calculate direction in the X-axis
            if player_pos.0 < enemy_pos.0 && tile_map.tile_map[enemy_pos.1 as usize][(enemy_pos.0 - 1) as usize] == 0 {
                direction.0 = -1; // Move left
            } else if player_pos.0 > enemy_pos.0 && tile_map.tile_map[enemy_pos.1 as usize][(enemy_pos.0 + 1) as usize] == 0 {
                direction.0 = 1; // Move right
            }

            // Calculate direction in the Y-axis
            if player_pos.1 < enemy_pos.1 && tile_map.tile_map[(enemy_pos.1 - 1) as usize][enemy_pos.0 as usize] == 0 {
                direction.1 = -1; // Move up
            } else if player_pos.1 > enemy_pos.1 && tile_map.tile_map[(enemy_pos.1 + 1) as usize][enemy_pos.0 as usize] == 0 {
                direction.1 = 1; // Move down
            }

            *last_move = std::time::Instant::now(); // Reset the last move timer
        }
    }

    // Only update the direction if the new position does not result in a collision with another enemy
    if player_moved && !enemy_collision_check(enemies, current_index, (enemy_pos.0 + direction.0, enemy_pos.1 + direction.1)) {
        *last_move = std::time::Instant::now(); // Reset the last move timer
        direction
    } else {
        (0, 0) // No movement if collision would occur
    }
}

enum GameState {
    Menu,
    Playing,
}

fn render_menu(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, title: &two_d::TextBox, play_button: &two_d::Button, quit_button: &two_d::Button, color: sdl2::pixels::Color) -> Result<(), String> {
    // Clear the screen with a dark background
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();

    title.render(canvas, color)?;

    // Render buttons
    play_button.render(canvas, color)?;
    quit_button.render(canvas, color)?;

    Ok(())
}

fn handle_menu_event(event: sdl2::event::Event, play_button: &two_d::Button, quit_button: &two_d::Button, game_state: &mut GameState) {
    match event {
        sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
            *game_state = GameState::Menu;
        },
        sdl2::event::Event::MouseButtonDown { x, y, .. } => {
            if play_button.is_pressed(x, y) {
                *game_state = GameState::Playing;
            } else if quit_button.is_pressed(x, y) {
                // Exit the game
                std::process::exit(0);
            }
        },
        _ => {}
    }
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
    let mut ladder = two_d::TextureManager::new(&texture_creator);
    ladder.load_texture(&std::path::Path::new("ladder.png"))?;

    let (generated_map, mut spawn_points, mut ladder_position) = generate_level(10, 10, player_grid_position, (1, 8));
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
                grid_position: (spawn_point.0 as i32, spawn_point.1 as i32), // Initialize grid_position
                texture_manager: enemy_texture_manager,
                speed: 3, // The speed at which the enemy moves, you can adjust this as needed
                last_move: std::time::Instant::now(), // Set the last move to the current time
            };
            enemies.push(enemy);
            spawn_points.retain(|&p| p != spawn_point); // Remove the used spawn point
        }
    }

    let mut tile_map = two_d::Tile::from_generated_map(
        generated_map,
        vec![&floor, &wall, &obstacle, &ladder],
        None,
    )?;

    let mut camera = two_d::Camera::new(nalgebra::Vector2::new(0, 0), nalgebra::Vector2::new(800, 600));

    let mut flip_horizontal = false;

    // Key press state tracking
    let mut left_key_pressed = false;
    let mut right_key_pressed = false;
    let mut up_key_pressed = false;
    let mut down_key_pressed = false;
    let mut player_moved = false;

    // Key press state tracking for attacking
    let mut attack_key_pressed = false;

    let mut game_state = GameState::Menu;
    // layer + button
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut ui_layer = two_d::Layer::new();
    // Load a font:
    let font_path = std::path::Path::new("ARIALUNI.TTF");
    let font_size = 24;
    let font = std::sync::Arc::new(sdl2::ttf::Sdl2TtfContext::load_font(&ttf_context, font_path, font_size)?);
    let text_box = std::rc::Rc::new(two_d::TextBox::new("Play".to_lowercase(), font, sdl2::rect::Rect::new(340, 340, 80, 50)));
    let play_button = std::rc::Rc::new(two_d::Button::new(text_box.clone(), 
    sdl2::pixels::Color::RGB(0, 0, 0), 
    sdl2::rect::Rect::new(340, 340, 120, 50), (0, 0), 0, 
    Box::new(|| {
        println!("Button pressed!");
    }),));

    let font2 = std::sync::Arc::new(sdl2::ttf::Sdl2TtfContext::load_font(&ttf_context, font_path, font_size)?);
    let text_box2 = std::rc::Rc::new(two_d::TextBox::new("Quit".to_lowercase(), font2, sdl2::rect::Rect::new(340, 420, 80, 50)));
    let quit_button = std::rc::Rc::new(two_d::Button::new(text_box2.clone(), 
    sdl2::pixels::Color::RGB(0, 0, 0), 
    sdl2::rect::Rect::new(340, 420, 120, 50), (0, 0), 0, 
    Box::new(|| {
        println!("Button pressed!");
    }),));

    let title_font_size = 36; // Larger font for the title
    let title_font = std::sync::Arc::new(sdl2::ttf::Sdl2TtfContext::load_font(&ttf_context, font_path, title_font_size)?);
    let title_text = "Rusty Dungeon";
    let title_width = 300; // Width of the title box, adjust as needed
    let title_height = 50;  // Height of the title box, adjust as needed
    let title_x = (800 - title_width) / 2; // Center the title
    let title_y = 50; // Position the title at 50 pixels from the top

    let text_box_title = std::rc::Rc::new(two_d::TextBox::new(
        title_text.to_string(),
        title_font,
        sdl2::rect::Rect::new(title_x, title_y, title_width as u32, title_height),
    ));

    ui_layer.add_button(play_button.clone());
    ui_layer.add_button(quit_button.clone());

    'mainloop: loop {

        match game_state {
            GameState::Menu => {
                // Render menu and handle menu events
                for event in input_handler.poll_events() {
                    handle_menu_event(event, &play_button, &quit_button, &mut game_state);
                }
                render_menu(&mut window.canvas, &text_box_title, &play_button, &quit_button, sdl2::pixels::Color::RGB(255, 255, 255))?;
            },
            GameState::Playing => {
                // Existing game logic goes here
                player_moved = false;

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
                                            && (tile_map.tile_map[new_position.1][new_position.0] == 0 || tile_map.tile_map[new_position.1][new_position.0] == 3)
                                            && !check_collision_with_enemies(&enemies, new_position)
                                        {
                                            left_key_pressed = true;
                                            player_grid_position = new_position;
                                            flip_horizontal = true;
                                            player.texture_manager_anim.set_animation("walk_right");
                                            player_moved = true;
                                        }
                                    },

                                // Check collisions for right movement
                                two_d::KeyEvent::Right if !right_key_pressed => {
                                    let new_position = (player_grid_position.0 + 1, player_grid_position.1);
                                    if player_grid_position.0 < tile_map.tile_map[0].len() - 1
                                        && (tile_map.tile_map[new_position.1][new_position.0] == 0 || tile_map.tile_map[new_position.1][new_position.0] == 3) // Allow movement if tile is empty or ladder
                                        && !check_collision_with_enemies(&enemies, new_position)
                                    {
                                        right_key_pressed = true;
                                        player_grid_position = new_position;
                                        flip_horizontal = false;
                                        player.texture_manager_anim.set_animation("walk_right");
                                        player_moved = true;
                                    }
                                },

                                // Check collisions for up movement
                                two_d::KeyEvent::Up if !up_key_pressed => {
                                    let new_position = (player_grid_position.0, player_grid_position.1 - 1);
                                    if player_grid_position.1 > 0
                                        && (tile_map.tile_map[new_position.1][new_position.0] == 0 || tile_map.tile_map[new_position.1][new_position.0] == 3)
                                        && !check_collision_with_enemies(&enemies, new_position)
                                    {
                                        up_key_pressed = true;
                                        player_grid_position = new_position;
                                        player.texture_manager_anim.set_animation("walk_up");
                                        player_moved = true;
                                    }
                                },

                                // Check collisions for down movement
                                two_d::KeyEvent::Down if !down_key_pressed => {
                                    let new_position = (player_grid_position.0, player_grid_position.1 + 1);
                                    if player_grid_position.1 < tile_map.tile_map.len() - 1
                                        && (tile_map.tile_map[new_position.1][new_position.0] == 0 || tile_map.tile_map[new_position.1][new_position.0] == 3)
                                        && !check_collision_with_enemies(&enemies, new_position)
                                    {
                                        down_key_pressed = true;
                                        player_grid_position = new_position;
                                        player.texture_manager_anim.set_animation("walk_down");
                                        player_moved = true;
                                    }
                                },
                                // Handle attack key event
                                two_d::KeyEvent::Other(sdl2::keyboard::Keycode::Space) if !attack_key_pressed => {
                                    attack_key_pressed = true;
                                },
                                _ => {},
                                }
                            },
                            two_d::GEvent::KeyUp(ref key_event) => {
                                match key_event {
                                    two_d::KeyEvent::Left | two_d::KeyEvent::Right | two_d::KeyEvent::Up | two_d::KeyEvent::Down | two_d::KeyEvent::Other(sdl2::keyboard::Keycode::Space) => {
                                        left_key_pressed = false;
                                        right_key_pressed = false;
                                        up_key_pressed = false;
                                        down_key_pressed = false;
                                        player.texture_manager_anim.set_animation("idle");
                                        player_moved = false;
                                        attack_key_pressed = false;
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

                if player_grid_position == ladder_position {
                    let (new_map, new_spawn_points, new_ladder_position) = generate_level(10, 10, player_grid_position, (1, 8));
                    ladder_position = new_ladder_position;

                    // Update tile_map with the new level
                    tile_map = two_d::Tile::from_generated_map(
                        new_map,
                        vec![&floor, &wall, &obstacle, &ladder],
                        None,
                    )?;

                    // Clear and repopulate enemies for the new level
                    enemies.clear();
                    for _ in 0..5 {
                        if let Some(spawn_point) = new_spawn_points.choose(&mut rng).cloned() {
                            let mut enemy_texture_manager = two_d::TextureManagerAnim::new(&texture_creator);
                            enemy_texture_manager.load_animation("enemy_idle", std::path::Path::new("player_anim.png"), 16, 18, 150, 0)?;

                            let enemy = Enemy {
                                position: nalgebra::Vector2::new(spawn_point.0 as i32 * TILE_SIZE, spawn_point.1 as i32 * TILE_SIZE),
                                grid_position: (spawn_point.0 as i32, spawn_point.1 as i32),
                                texture_manager: enemy_texture_manager,
                                speed: 3,
                                last_move: std::time::Instant::now(),
                            };
                            enemies.push(enemy);
                        }
                    }
                    // Reset player animation state
                    player.texture_manager_anim.set_animation("idle");
                }

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

                // Assuming you're using a similar rendering method for the ladder as for other tiles
                let ladder_rect = sdl2::rect::Rect::new(
                    (ladder_position.0 as i32 * TILE_SIZE as i32) as i32, 
                    (ladder_position.1 as i32 * TILE_SIZE as i32) as i32, 
                    TILE_SIZE as u32, TILE_SIZE as u32
                );
                let transformed_ladder_rect = camera.transform_rect(ladder_rect);
                ladder.render_texture(&mut window.canvas, transformed_ladder_rect)?;

                // Call the move_towards_player function for each enemy
                if player_moved {
                    // In the game loop, when updating enemy positions
                    for index in 0..enemies.len() {
                        let player_position = (player_grid_position.0 as i32, player_grid_position.1 as i32);
                        let enemy_position = (enemies[index].grid_position.0, enemies[index].grid_position.1);
                        
                        // Temporarily clone the enemy's last_move to pass it to the function
                        let mut last_move_clone = enemies[index].last_move.clone();

                        let direction = move_towards_player(
                            player_position,
                            enemy_position,
                            &enemies, // This is safe because we are not mutating enemies here
                            index,
                            &tile_map,
                            enemies[index].speed,
                            &mut last_move_clone, // Use the clone instead of the original
                            player_moved
                        );
                        
                        // If move_towards_player determined a valid direction, update the enemy
                        if direction != (0, 0) {
                            enemies[index].grid_position.0 += direction.0;
                            enemies[index].grid_position.1 += direction.1;
                            enemies[index].position = nalgebra::Vector2::new(enemies[index].grid_position.0 * TILE_SIZE, enemies[index].grid_position.1 * TILE_SIZE);
                            enemies[index].last_move = last_move_clone; // Update the last_move with the clone
                        }
                    }

                }

                // Game loop
                if attack_key_pressed {
                    // Define the attack range
                    let attack_range = 1; // One tile around the player

                    // Use retain to keep only the enemies that are not hit
                    enemies.retain(|enemy| {
                        let distance_x = (player_grid_position.0 as i32 - enemy.grid_position.0).abs();
                        let distance_y = (player_grid_position.1 as i32 - enemy.grid_position.1).abs();

                        // If the enemy is outside the attack range, keep it
                        distance_x > attack_range || distance_y > attack_range
                    });

                    attack_key_pressed = false; // Reset the attack key after handling the attack
                }
            },
        }
 
        window.canvas.present();
    }
    Ok(())
}