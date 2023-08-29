pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let last_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
    let mut current_frame_time;
    let mut delta_time;

    let texture_creator = canvas.texture_creator();

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

    // Create PointLight
    let mut point_light = PointLight::new(TextureManager::new(&texture_creator), Vector2::new(200, 200));
    point_light.load_texture(Path::new("point_light.png"))?;
    // Create a texture for the light map
    let mut light_map_texture = texture_creator
        .create_texture_target(sdl2::pixels::PixelFormatEnum::RGBA8888, 800, 600)
        .map_err(|e| e.to_string())?;

    'mainloop: loop {
        current_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
        delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;

        for event in sdl_context.event_pump()?.poll_iter() {
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

        // Clear the light map
        canvas.with_texture_canvas(&mut light_map_texture, |texture_canvas| {
            texture_canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 128));
            texture_canvas.clear();
        }).map_err(|e| e.to_string())?;

        // Render the point light on the light map
        canvas.set_blend_mode(sdl2::render::BlendMode::Add);
        canvas.with_texture_canvas(&mut light_map_texture, |texture_canvas| {
            point_light.render_texture(texture_canvas).unwrap();
        }).map_err(|e| e.to_string())?;

        // Render the scene
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

        canvas.clear();

        tile_map.render(&mut canvas, (82, 82))?;
        player.render_texture(&mut canvas, 2)?;
        enemy.render_texture(&mut canvas, 2)?;

        // Render the light map on top of the scene
        canvas.set_blend_mode(sdl2::render::BlendMode::Mod);
        canvas.copy(&light_map_texture, None, None)?;
        
        canvas.present();
    }

    Ok(())
}