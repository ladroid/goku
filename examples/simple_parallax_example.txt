mod two_d;

pub fn test_parallax() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    
    // initialize your texture managers
    let mut texture_manager_1 = TextureManager::new(&texture_creator);
    texture_manager_1.load_texture(Path::new("E:\\Projects\\RustProj\\GameEngine\\Tall Forest Files\\Layers\\back.png"))?;
    let mut texture_manager_2 = TextureManager::new(&texture_creator);
    texture_manager_2.load_texture(Path::new("E:\\Projects\\RustProj\\GameEngine\\Tall Forest Files\\Layers\\far.png"))?;
    
    // initialize your parallax layers
    let parallax_layer_1 = ParallaxLayer::new(texture_manager_1, 200.0);
    let parallax_layer_2 = ParallaxLayer::new(texture_manager_2, 200.0);
    
    // initialize your parallax background
    let mut parallax_background = ParallaxBackground::new(vec![parallax_layer_1, parallax_layer_2]);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut camera = Camera { 
        position: Vector2::new(0, 0), 
        size: Vector2::new(800, 600)
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        i = (i + 1) % 255;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(i, 64, 255 - i));
        canvas.clear();

        // update the parallax background
        let delta_time = 1.0 / 60.0; // assuming a constant 60 FPS for simplicity
        parallax_background.update(delta_time);

        // render the parallax background
        parallax_background.render(&mut canvas, &camera).unwrap();

        canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}