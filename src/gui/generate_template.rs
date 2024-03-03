use crate::gui::state::State;
use crate::gui::light_type::LightType;

pub fn generate_template(state: &mut State) {
    let window_title = &state.window_name;
    let window_width = state.window_width;
    let window_height = state.window_height;
    // Assume enable_fullscreen is a boolean reflecting the fullscreen state
    let enable_fullscreen = state.general_settings.enable_fullscreen;
    let mut content = format!(r#"
mod two_d;
use nalgebra::Vector2;
use std::path::Path;
use crate::two_d::{{Window, TextureManagerAnim, GameObject, Camera, InputHandler, AmbientFilter, PointLight, SpotLight}};

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let mut window = Window::new("{}", {}, {}, {})?;

    let mut last_frame_time = unsafe {{ sdl2::sys::SDL_GetTicks() }};
    let mut current_frame_time;
    let mut delta_time;
    
    // Create a camera object
    let mut camera = Camera::new(Vector2::new(0, 0), Vector2::new({}, {}));

    let texture_creator = window.canvas.texture_creator();
    "#, window_title, window_width, window_height, enable_fullscreen, window_width, window_height);

    // Create a vector to store game object variable names
    let mut game_objects = Vec::new();

    for component in &state.components {
        if component.name == "GameObject" {
            let game_object_var_name = format!("game_object{}", game_objects.len() + 1);
            game_objects.push(game_object_var_name.clone());
            content.push_str(&format!(r#"
    let mut texture_manager{} = TextureManagerAnim::new(&texture_creator);
    let mut {} = GameObject::new(texture_manager{}, Vector2::new(50, 50));
    "#, game_objects.len(), game_object_var_name, game_objects.len()));
    
            for texture in &state.textures {
                content.push_str(&format!(r#"
    {}.load_texture("{}", Path::new("{}"), {}, {}, 150, 0)?;
    "#, game_object_var_name, texture.tag_name, texture.path.display(), texture.width, texture.height));
            }
        }
    }

    // Additional code for AmbientFilter if it exists in state
    if state.components.iter().any(|c| c.name == "Ambient Filter") {
        for (index, filter) in state.ambient_filters.iter().enumerate() {
            content.push_str(&format!(r#"
    // Initialize AmbientFilter {}
    let mut light_texture{} = texture_creator.create_texture_streaming(None, 800, 600)?;
    light_texture{}.set_blend_mode(sdl2::render::BlendMode::Add);
    let light{} = AmbientFilter::new({});  // Intensity from the state
    "#, index, index, index, index, filter.intensity));
        }
    }

    if state.general_settings.enable_input_handler {
        content.push_str(r#"
    // Initialize InputHandler
    let mut input_handler = InputHandler::new(&window.sdl_context)?;
    "#);
    }

    if let Some(audio_player) = &state.audio_player {
        content.push_str(&format!(r#"
    // Initialize Audio Player
    let mut audio = two_d::audio::AudioPlayer::new(4);
    let _music = audio.play(std::path::Path::new("{}"), {}, {});
    "#, audio_player.track_path, audio_player.loop_count, audio_player.volume));
    }

    // Check if Light component is present and generate light code
    let mut add_light_setup = false;
    if let Some(_light) = state.components.iter().find(|c| c.name == "Light") {
        match state.light_type {
            LightType::Point => {
                add_light_setup = true;
                content.push_str(&format!(r#"
    // Initialize Point Light
    let light_texture = texture_creator.load_texture("{}")?;
    let light = two_d::PointLight::new(
        nalgebra::Vector2::new(400.0, 300.0), // You may want to replace these with dynamic values
        100.0,  // Light radius
        0.6,    // Light intensity
        sdl2::pixels::Color::RGB({}, {}, {}) // Light color
    );
    let mut darkness_texture = texture_creator.create_texture_target(None, 800, 600)?;
    darkness_texture.set_blend_mode(sdl2::render::BlendMode::Mod);
    "#, state.light_png_path, 
       (state.light_color[0] * 255.0) as u8,
       (state.light_color[1] * 255.0) as u8,
       (state.light_color[2] * 255.0) as u8));
            },
            LightType::Spotlight => {
                // Add code generation for Spotlight if applicable
                add_light_setup = true;
                content.push_str(&format!(r#"
    // Initialize Spot Light
    let mut spotlight_texture = texture_creator.load_texture("{}")?;
    let spotlight = two_d::SpotLight::new(
        nalgebra::Vector2::new(400.0, 300.0),
        nalgebra::Vector2::new(0.0, -1.0),   // Pointing upwards
        45.0,                                // 45-degree cone
        200.0,
        0.6,
        sdl2::pixels::Color::RGB({}, {}, {})
    );
    let mut darkness_texture = texture_creator.create_texture_target(None, 800, 600)?;
    darkness_texture.set_blend_mode(sdl2::render::BlendMode::Mod);
    "#, state.light_png_path, 
        (state.light_color[0] * 255.0) as u8,
        (state.light_color[1] * 255.0) as u8,
        (state.light_color[2] * 255.0) as u8));
            },
            _ => {} // Handle other light types or no light
        }
    }

    // Inserting the main loop
    content.push_str(r#"
    'mainloop: loop {
        current_frame_time = unsafe { sdl2::sys::SDL_GetTicks() };
        delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;
    "#);

    if state.general_settings.enable_input_handler {
        content.push_str(r#"
        for event in input_handler.poll_events() {
            // Process input events here
            // ...
        }
        "#);
    }

    for game_object_var_name in &game_objects {
        content.push_str(&format!(r#"
        // Update and render each game object: {}
        // Update camera position to follow this game object (if needed)
        camera.update({}.get_position());

        window.canvas.clear();
        // Render the game object
        if let Some(current_animation_tag) = &{}.texture_manager_anim.current_animation {{
            if let Some(animated_texture) = {}.texture_manager_anim.animations.get(current_animation_tag) {{
                let rect = sdl2::rect::Rect::new(
                    {}.position.x, 
                    {}.position.y, 
                    animated_texture.sprite_sheet.frame_width * 2, 
                    animated_texture.sprite_sheet.frame_height * 2
                );
                let transformed_rect = camera.transform_rect(rect);
                {}.texture_manager_anim.render_texture(&mut window.canvas, transformed_rect, 0)?;
            }}
        }}
        "#, game_object_var_name, game_object_var_name, game_object_var_name, game_object_var_name, game_object_var_name, game_object_var_name, game_object_var_name));
    }

    // Additional rendering code for AmbientFilter
    if state.components.iter().any(|c| c.name == "Ambient Filter") {
        // Loop through each AmbientFilter component for rendering
        for (index, filter) in state.ambient_filters.iter().enumerate() {
            // Convert each color component to an integer (0-255 range)
            let red = (filter.color[0] * 255.0) as u8;
            let green = (filter.color[1] * 255.0) as u8;
            let blue = (filter.color[2] * 255.0) as u8;
            let alpha = (filter.color[3] * 255.0) as u8;
            content.push_str(&format!(r#"
        // Clear the light texture with an ambient color (e.g., dark blue)
        light_texture{}.with_lock(None, |buffer: &mut [u8], pitch: usize| {{
            for y in 0..600 {{
                for x in 0..800 {{
                    let offset = y * pitch + x * 4;
                    buffer[offset] = {};       // Blue
                    buffer[offset + 1] = {};   // Green
                    buffer[offset + 2] = {};   // Red
                    buffer[offset + 3] = {};             // Alpha
                }}
            }}
        }}).unwrap();
        // Render the ambient light from filter {}
        light{}.render(&mut window.canvas, &mut light_texture{});
        
        window.canvas.set_blend_mode(sdl2::render::BlendMode::Mod);
        window.canvas.copy(&light_texture{}, None, None)?;
        window.canvas.set_blend_mode(sdl2::render::BlendMode::None);
        "#, index, blue, green, red, alpha, index, index, index, index));
            }
    }

    // Insert the light rendering code inside the main loop
    if add_light_setup {
        content.push_str(r#"
        // Render the light
        window.canvas.with_texture_canvas(&mut darkness_texture, |canvas| {
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 150)); // Semi-transparent black
            canvas.clear();
            light.render(canvas, &light_texture);
        }).unwrap();
        window.canvas.set_blend_mode(sdl2::render::BlendMode::Mod);
        window.canvas.copy(&darkness_texture, None, None).unwrap();
        window.canvas.set_blend_mode(sdl2::render::BlendMode::None);
        "#);
    }

    content.push_str(r#"
        window.canvas.present();
    }
    Ok(())
}"#);

    state.text_editor_content = content;
}