mod two_d;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
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
    t1.load_texture(&Path::new("grass.png"))?;
    let mut t2 = TextureManager::new(&texture_creator);
    t2.load_texture(&Path::new("dirt.png"))?;

    let tile_map = Tile::new(Path::new("map.txt"), vec![
        t1, t2,
        // Add more TextureManager objects for each tile type you want to render
    ])?;

    let vec_coll = vec![enemy.collider];

    // Create a camera object
    let mut camera = Camera::new(Vector2::new(0, 0), Vector2::new(800, 600));

    // layer + button
    let mut ui_layer = Layer::new();
    //let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    // Load a font:
    let font_path = Path::new("arial.ttf");
    let font_size = 24;
    let font = std::sync::Arc::new(TTF_CONTEXT.load_font(font_path, font_size)?);
    let text_box = std::rc::Rc::new(TextBox::new("Hello, world".to_lowercase(), font, sdl2::rect::Rect::new(50, 20, 100, 50)));
    let button = std::rc::Rc::new(Button::new(text_box.clone(), sdl2::pixels::Color::RGB(123, 23, 56), sdl2::rect::Rect::new(50, 20, 100, 50), Box::new(|| {
        println!("Button pressed!");
    }),));
    ui_layer.add_button(button.clone());

    // Create a slider
    let background_rect = sdl2::rect::Rect::new(300, 20, 200, 20);
    let slider_rect = sdl2::rect::Rect::new(300, 10, 40, 40);
    let mut slider = Slider::new(
        background_rect,
        slider_rect,
        sdl2::pixels::Color::RGB(255, 255, 255),
        sdl2::pixels::Color::RGB(200, 200, 200),
        Box::new(|value| {
            println!("Slider value: {}", value);
        }),
    );

    
    'mainloop: loop {
        current_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
        delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;

        for event in window.sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::MouseButtonDown { x, y, .. } => {
                    ui_layer.handle_mouse_click(x, y);
                    slider.handle_mouse_click(x, y);
                }
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
        let player_rect = Rect::new(player.position.x, player.position.y, player.texture_manager.texture.as_ref().unwrap().sprite_sheet.frame_width * 2, player.texture_manager.texture.as_ref().unwrap().sprite_sheet.frame_height * 2);
        let transformed_player_rect = camera.transform_rect(player_rect);
        player.texture_manager.render_texture(&mut window.canvas, transformed_player_rect)?;

        // Render the enemy
        let enemy_rect = Rect::new(enemy.position.x, enemy.position.y, enemy.texture_manager.texture.as_ref().unwrap().sprite_sheet.frame_width * 2, enemy.texture_manager.texture.as_ref().unwrap().sprite_sheet.frame_height * 2);
        let transformed_enemy_rect = camera.transform_rect(enemy_rect);
        enemy.texture_manager.render_texture(&mut window.canvas, transformed_enemy_rect)?;

        ui_layer.render(&mut window.canvas)?;
        slider.render(&mut window.canvas)?;

        window.canvas.present();
    }

    Ok(())
}