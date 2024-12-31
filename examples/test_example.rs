use goku::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set current directory to the root of the project
    std::env::set_current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("Failed to set project root as current directory");

    let mut window = two_d::Window::new("My Game", 800, 600, false)?;

    let last_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
    let mut current_frame_time;
    let mut delta_time;

    // Initialize InputHandler
    let mut input_handler = two_d::InputHandler::new(&window.sdl_context)?;

    let texture_creator = window.canvas.texture_creator();

    let texture_manager = two_d::TextureManagerAnim::new(&texture_creator);

    let texture_manager2 = two_d::TextureManagerAnim::new(&texture_creator);

    let mut player = two_d::GameObject::new(texture_manager, nalgebra::Vector2::new(50, 50));
    player.load_texture("idle", std::path::Path::new("test_assets/player_anim.png"), 30, 30, 150, 0)?;

    let mut enemy = two_d::GameObject::new(texture_manager2, nalgebra::Vector2::new(70, 70));
    enemy.load_texture("idle", std::path::Path::new("test_assets/player_anim.png"), 30, 30, 150, 0)?;

    let mut t1 = two_d::TextureManager::new(&texture_creator);
    t1.load_texture(&std::path::Path::new("test_assets/grass.png"))?;
    let mut t2 = two_d::TextureManager::new(&texture_creator);
    t2.load_texture(&std::path::Path::new("test_assets/dirt.png"))?;

    let tile_map = two_d::Tile::new(std::path::Path::new("test_assets/map.txt"), vec![
        &t1, &t2,
        // Add more TextureManager objects for each tile type you want to render
    ], None)?;

    let vec_coll = vec![enemy.collider];

    // Create a camera object
    let mut camera = two_d::Camera::new(nalgebra::Vector2::new(0, 0), nalgebra::Vector2::new(800, 600));

    // layer + button
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut ui_layer = two_d::Layer::new();
    // Load a font:
    let font_path = std::path::Path::new("test_assets/ARIALUNI.TTF");
    let font_size = 24;
    let font = std::sync::Arc::new(sdl2::ttf::Sdl2TtfContext::load_font(&ttf_context, font_path, font_size)?);
    let text_box = std::rc::Rc::new(two_d::TextBox::new("Hello, world".to_lowercase(), font, sdl2::rect::Rect::new(50, 20, 120, 50)));
    let button = std::rc::Rc::new(two_d::Button::new(text_box.clone(), 
    sdl2::pixels::Color::RGB(123, 23, 56), 
    sdl2::rect::Rect::new(50, 20, 120, 50), (0, 0), 0, 
    Box::new(|| {
        println!("Button pressed!");
    }),));
    ui_layer.add_button(button.clone());

    // Create a slider
    let background_rect = sdl2::rect::Rect::new(300, 20, 200, 20);
    let slider_rect = sdl2::rect::Rect::new(300, 10, 40, 40);
    let mut slider = two_d::Slider::new(
        background_rect,
        slider_rect,
        sdl2::pixels::Color::RGB(255, 255, 255),
        sdl2::pixels::Color::RGB(200, 200, 200),
        Box::new(|value| {
            println!("Slider value: {}", value);
        }),
    );
    let flip_horizontal = false;
    
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
                                player.texture_manager_anim.set_animation("idle");
                            },
                            two_d::KeyEvent::Right => {
                                player.texture_manager_anim.set_animation("idle");
                            },
                            two_d::KeyEvent::Up => {
                                player.texture_manager_anim.set_animation("idle");
                            }, 
                            two_d::KeyEvent::Down => {
                                player.texture_manager_anim.set_animation("idle");
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

        // Render the enemy
        if let Some(current_animation_tag) = &enemy.texture_manager_anim.current_animation {
            if let Some(animated_texture) = enemy.texture_manager_anim.animations.get(current_animation_tag) {
                let enemy_rect = two_d::Rect::new(
                    enemy.position.x, 
                    enemy.position.y, 
                    animated_texture.sprite_sheet.frame_width * 2, 
                    animated_texture.sprite_sheet.frame_height * 2
                );
                let transformed_enemy_rect = camera.transform_rect(&enemy_rect);
                enemy.texture_manager_anim.render_texture(&mut window.canvas, transformed_enemy_rect.unwrap(), flip_horizontal as u32)?;
            }
        }

        ui_layer.render(&mut window.canvas, two_d::Color::new(0, 0, 0).sdl_color())?;
        slider.render(&mut window.canvas)?;

        window.canvas.present();
    }

    Ok(())
}