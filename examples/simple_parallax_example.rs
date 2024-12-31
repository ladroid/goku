/*
* Assets were take from here -> https://ansimuz.itch.io/sunnyland-tall-forest

Your distribution folder might look like this:

MyGame/
├── assets/
│   ├── images/
│   ├── sounds/
│   └── ...
├── src/
└── Cargo.toml
*/

use goku::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set current directory to the root of the project
    std::env::set_current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("Failed to set project root as current directory");
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    
    // initialize your texture managers
    let mut texture_manager_1 = two_d::TextureManager::new(&texture_creator);
    texture_manager_1.load_texture(std::path::Path::new("test_assets/back.png"))?;
    let mut texture_manager_2 = two_d::TextureManager::new(&texture_creator);
    texture_manager_2.load_texture(std::path::Path::new("test_assets/far.png"))?;
    
    // initialize your parallax layers
    let parallax_layer_1 = two_d::ParallaxLayer::new(texture_manager_1, 200.0);
    let parallax_layer_2 = two_d::ParallaxLayer::new(texture_manager_2, 200.0);
    
    // initialize your parallax background
    let mut parallax_background = two_d::ParallaxBackground::new(vec![parallax_layer_1, parallax_layer_2]);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let camera = two_d::Camera { 
        position: nalgebra::Vector2::new(0, 0), 
        size: nalgebra::Vector2::new(800, 600)
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        i = (i + 1) % 255;
        canvas.set_draw_color(two_d::Color::new(i, 64, 255 - i).sdl_color());
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