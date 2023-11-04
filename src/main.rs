// mod gui;
// mod two_d;
// use crate::gui::launcher;
// 
// fn main() {
//     launcher();
// }

mod two_d;
use rand::Rng;
use sdl2::image::LoadTexture;

fn generate_level(width: usize, height: usize, player_pos: (usize, usize), door_pos: (usize, usize)) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut grid = vec![vec![0; width]; height];  // Initialize grid with empty spaces
    
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

    grid
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

    player.load_texture("idle", std::path::Path::new("E:\\Projects\\RustProj\\GameEngine\\10chars\\character_idle_anim.png"), 16, 18, 150, 0)?;
    player.load_texture("walk_down", std::path::Path::new("E:\\Projects\\RustProj\\GameEngine\\10chars\\character_walk_anim.png"), 16, 18, 150, 0)?;
    player.load_texture("walk_up", std::path::Path::new("E:\\Projects\\RustProj\\GameEngine\\10chars\\character_walk_anim.png"), 16, 17, 150, 1)?;
    player.load_texture("walk_right", std::path::Path::new("E:\\Projects\\RustProj\\GameEngine\\10chars\\character_walk_anim.png"), 16, 17, 150, 2)?;

    let mut floor = two_d::TextureManager::new(&texture_creator);
    floor.load_texture(&std::path::Path::new("E:\\Projects\\RustProj\\GameEngine\\sprites\\world\\ground.png"))?;
    let mut wall = two_d::TextureManager::new(&texture_creator);
    wall.load_texture(&std::path::Path::new("E:\\Projects\\RustProj\\GameEngine\\sprites\\world\\door.png"))?;
    let mut obstacle = two_d::TextureManager::new(&texture_creator);
    obstacle.load_texture(&std::path::Path::new("E:\\Projects\\RustProj\\GameEngine\\sprites\\world\\stone.png"))?;

    let generated_map = generate_level(10, 10, player_grid_position, (1, 8));

    let tile_map = two_d::Tile::from_generated_map(
        generated_map,
        vec![&floor, &wall, &obstacle],
        None,
    )?;

    let mut camera = two_d::Camera::new(nalgebra::Vector2::new(0, 0), nalgebra::Vector2::new(800, 600));

    let mut flip_horizontal = false;

    let mut light_spot_texture = texture_creator.load_texture("E:\\Projects\\RustProj\\GameEngine\\goku\\test_assets\\point_light.png")?;
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
                            two_d::KeyEvent::Left if !left_key_pressed => {
                                if player_grid_position.0 > 0 && tile_map.tile_map[player_grid_position.1][player_grid_position.0 - 1] == 0 {
                                    left_key_pressed = true;
                                    player_grid_position.0 -= 1;
                                    flip_horizontal = true;
                                    player.texture_manager_anim.set_animation("walk_right");
                                }
                            },
                            two_d::KeyEvent::Right if !right_key_pressed => {
                                if player_grid_position.0 < tile_map.tile_map[0].len() - 1 && tile_map.tile_map[player_grid_position.1][player_grid_position.0 + 1] == 0 {
                                    right_key_pressed = true;
                                    player_grid_position.0 += 1;
                                    flip_horizontal = false;
                                    player.texture_manager_anim.set_animation("walk_right");
                                }
                            },
                            two_d::KeyEvent::Up if !up_key_pressed => {
                                if player_grid_position.1 > 0 && tile_map.tile_map[player_grid_position.1 - 1][player_grid_position.0] == 0 {
                                    up_key_pressed = true;
                                    player_grid_position.1 -= 1;
                                    flip_horizontal = false;
                                    player.texture_manager_anim.set_animation("walk_up");
                                }
                            },
                            two_d::KeyEvent::Down if !down_key_pressed => {
                                if player_grid_position.1 < tile_map.tile_map.len() - 1 && tile_map.tile_map[player_grid_position.1 + 1][player_grid_position.0] == 0 {
                                    down_key_pressed = true;
                                    player_grid_position.1 += 1;
                                    flip_horizontal = false;
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
