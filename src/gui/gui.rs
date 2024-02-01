use imgui::Context;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use rfd::FileDialog;
use copypasta::ClipboardProvider;
use crate::gui::component::Component;
use crate::gui::texture_component::TextureComponent;
use crate::gui::ambient_filter_component::AmbientFilterComponent;
use crate::gui::audio_player_component::AudioPlayerComponent;
use crate::gui::light_type::LightType;
use crate::gui::state::State;
use crate::gui::about_info::AboutInfo;
use crate::gui::display_component_tree::DisplayComponentTree;
use crate::gui::main_functionality::build_code;
use crate::gui::main_functionality::execute_code;
use crate::gui::main_functionality::execute_code_web;
use crate::gui::main_functionality::open_state;
use crate::gui::main_functionality::save_project;
use crate::gui::main_functionality::save_project_to_path;
use crate::gui::main_functionality::execute_command;
use crate::gui::main_functionality::handle_two_d_module;
use crate::deepl::deepl::call_python_add_function;

fn sdl_surface_to_gl_texture(surface: sdl2::surface::Surface) -> Result<u32, String> {
    let mut texture_id: u32 = 0;

    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        // Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let format = match surface.pixel_format_enum() {
            sdl2::pixels::PixelFormatEnum::RGB24 => gl::RGB,
            sdl2::pixels::PixelFormatEnum::RGBA32 => gl::RGBA,
            _ => return Err("Unsupported pixel format".to_string()),
        };

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            surface.width() as i32,
            surface.height() as i32,
             0,
            format,
            gl::UNSIGNED_BYTE,
            surface.without_lock().unwrap().as_ptr() as *const _,
        );
    }

    Ok(texture_id)
}

pub fn launcher() -> Result<(), String> {
    /* initialize SDL and its video subsystem */
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

    /* hint SDL to initialize an OpenGL 3.3 core profile context */
    /* create a new OpenGL context and make it current */
    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }

    /* create a new window, be sure to call opengl method on the builder when using glow! */
    let window = video_subsystem
        .window("goku engine", 1280, 720)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let mut canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();

    /* enable vsync to cap framerate */
    canvas.window().subsystem().gl_set_swap_interval(1).unwrap();
    
    /* create context */
    let mut imgui = Context::create();

    /* disable creation of files on disc */
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    // Enable docking
    let io = imgui.io_mut();
    io.config_flags |= imgui::ConfigFlags::DOCKING_ENABLE; // Enable Docking
    io.config_flags |= imgui::ConfigFlags::VIEWPORTS_ENABLE; // Enable Multi-Viewport / Platform Windows

    let mut state = State::new();
    if let Err(e) = state.load_translations("src/gui/translation.json") {
        state.terminal.log_error(format!("Failed to load translations: {}", e));
        // handle error appropriately
    }

    /* setup platform and renderer, and fonts to imgui */
    let fonts = imgui.fonts();
    let font_path = format!("src/gui/fonts/{}.ttf", state.general_settings.font_name);
    let font_data = std::fs::read(&font_path).unwrap();
    fonts.add_font(&[imgui::FontSource::TtfData {
        data: &font_data,
        size_pixels: state.general_settings.font_size,
        config: Some(imgui::FontConfig {
            size_pixels: state.general_settings.font_size,
            oversample_h: 3,
            oversample_v: 1,
            pixel_snap_h: false,
            glyph_extra_spacing: [0.0, 0.0],
            glyph_offset: [0.0, 0.0],
            glyph_ranges: imgui::FontGlyphRanges::japanese(),
            glyph_min_advance_x: 0.0,
            glyph_max_advance_x: f32::MAX,
            font_builder_flags: 0,
            rasterizer_multiply: 1.0,
            ellipsis_char: None,
            name: None,
        }),
    }]);
    
    /* create platform and renderer */
    let window_ref = canvas.window();
    let mut platform = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window_ref);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video_subsystem.gl_get_proc_address(s) as _);

    /* load texture from PNG file */
    let texture_creator = canvas.texture_creator();
    let mut textures = Vec::new();

    /* Initialize texture position variables */
    let mut texture_pos: Vec<(f32, f32)> = vec![(301.676, 22.346)];  // One position for each texture
    let mut texture_scale: f32 = 1.0;

    /* start main loop */
    let mut event_pump = sdl.event_pump().unwrap();

    let mut is_light_theme = false; // Boolean to track the current theme (You can manage this in your state if required)

    let about_info = AboutInfo {
        version: String::from("0.1.2"),
        date: String::from("01-06-2023"),
        rust_version: String::from("1.68"),
        games_api: String::from("SDL2"),
        os: std::env::consts::OS.to_string(),
    };

    loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platform */
            platform.handle_event(&mut imgui, &event);

            match event {
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
                    texture_pos[0].1 -= 5.0; // Move up
                }
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => {
                    texture_pos[0].1 += 5.0; // Move down
                }
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => {
                    texture_pos[0].0 -= 5.0; // Move left
                }
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => {
                    texture_pos[0].0 += 5.0; // Move right
                }
                Event::Quit { .. } => {
                    state.canvas_present = true;
                    return Ok(());
                }
                _ => {}
            }
            if state.exit_requested {
                return Ok(());
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        platform.prepare_frame(imgui.io_mut(), &canvas.window(), &event_pump.mouse_state());

        let ui = imgui.new_frame();

        if state.open_about {
            ui.open_popup("goku game engine");
            state.open_about = false;
        }

        /* create a top menu bar */
        ui.main_menu_bar(|| {
            ui.menu(state.translate("File"), || {
                ui.menu_item(state.translate("New"));
                if ui.menu_item(state.translate("Open")) {
                    if let Ok(Some(new_state)) = open_state() {
                        state = new_state; // Replace the current state with the newly loaded one
                        println!("Open ok");
                        state.terminal.log("Open ok");
                    } else {
                        println!("Open error");
                        state.terminal.log("Open error");
                    }
                }                
                if ui.menu_item(state.translate("Save")) {
                    if let Some((path, _)) = save_project() {
                        if save_project_to_path(&path, &state).is_ok() {
                            println!("Project saved to {:?}", path);
                            state.terminal.log("Project saved");
                        } else {
                            eprintln!("Failed to save project to {:?}", path);
                            state.terminal.log_error(format!("Failed to save project to {:?}", path));
                        }
                    }
                }
                if ui.menu_item(state.translate("Save As...")) {
                    save_project();
                }
                if ui.menu_item(state.translate("Preferences")) {
                    state.open_preferences = true;
                    println!("Open preferenes");
                    state.terminal.log("Open preferenes");
                }
                ui.separator();
                if ui.menu_item(state.translate("Exit")) {
                    state.exit_requested = true;
                }
            });
            ui.menu(state.translate("Edit"), || {
                if ui.menu_item(state.translate("Undo")) {
                    if let Some(previous_content) = state.undo_stack.pop() {
                        state.redo_stack.push(state.text_editor_content.clone());
                        state.text_editor_content = previous_content;
                    }
                }
                if ui.menu_item(state.translate("Redo")) {
                    if let Some(next_content) = state.redo_stack.pop() {
                        state.undo_stack.push(state.text_editor_content.clone());
                        state.text_editor_content = next_content;
                    }
                }
                ui.separator();
                if ui.menu_item(state.translate("Cut")) {
                    if let Some(clipboard) = &mut state.clipboard {
                        clipboard.set_contents(state.text_editor_content.clone()).ok();
                        state.text_editor_content.clear();
                    }
                }
            
                if ui.menu_item(state.translate("Copy")) {
                    if let Some(clipboard) = &mut state.clipboard {
                        clipboard.set_contents(state.text_editor_content.clone()).ok();
                    }
                }
            
                if ui.menu_item(state.translate("Paste")) {
                    if let Some(clipboard) = &mut state.clipboard {
                        if let Ok(contents) = clipboard.get_contents() {
                            state.text_editor_content.push_str(&contents);
                        }
                    }
                }
            });
            ui.menu(state.translate("View"), || {
                ui.menu(state.translate("Appearance"), || { // Appearance submenu
                    ui.menu(state.translate("Themes"), || { // Submenu for themes
                        if ui.radio_button(state.translate("Light Theme"), &mut is_light_theme, true) {
                            let mut style = unsafe { ui.style() }.clone();
                            style.use_light_colors();
                            // Implement logic for switching to light theme
                        }
                        if ui.radio_button(state.translate("Dark Theme"), &mut is_light_theme, false) {
                            let mut style = unsafe { ui.style() }.clone();
                            style.use_dark_colors();
                            // Implement logic for switching to dark theme
                        }
                    });
                });
                ui.input_text("Search", &mut state.search_query).build();
                if ui.button("Find Next") {
                    state.search_for_next();
                }
                ui.same_line();
                if ui.button("Find Previous") {
                    state.search_for_previous();
                }

                // Print the line containing the search result
                if !state.search_results.is_empty() && state.search_result_index < state.search_results.len() {
                    let start_index = state.search_results[state.search_result_index];
                    if let Some(line) = state.get_line_containing_index(start_index) {
                        ui.text_colored([1.0, 0.5, 0.2, 1.0], &line);  // Using a colored text to highlight the result
                    }
                }

                if ui.menu_item(state.translate("Text Editor")) {  // New menu item
                    state.open_text_editor = !state.open_text_editor;  // Toggle text editor open/close
                }
                if ui.menu_item("Character Generator") {
                    match call_python_add_function() {
                        Ok(image_path) => {
                            // Assuming `sdl_surface_to_gl_texture` is a function that takes an SDL2 surface and returns an OpenGL texture ID
                            // Load the image into an SDL2 surface
                            match sdl2::image::LoadSurface::from_file(&image_path) {
                                Ok(surface) => {
                                    match sdl_surface_to_gl_texture(surface) {
                                        Ok(tex_id) => {
                                            // Store this `tex_id` somewhere accessible for rendering
                                            state.dynamic_texture_id = Some(tex_id);
                                            println!("Image loaded successfully: {}", image_path);
                                            state.terminal.log(format!("Image loaded successfully: {}", image_path));
                                        },
                                        Err(e) => {
                                            println!("Failed to convert SDL2 surface to OpenGL texture: {}", e);
                                            state.terminal.log_error(format!("Failed to convert SDL2 surface to OpenGL texture: {}", e));
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to load image: {}", e);
                                    state.terminal.log_error(format!("Failed to load image: {}", e));
                                }
                            }
                        },
                        Err(e) => {
                            println!("Python function execution failed: {}", e);
                            state.terminal.log_error(format!("Python function execution failed: {}", e));
                        },
                    }
                    state.open_image_view = !state.open_image_view;
                }
            });
            ui.menu(state.translate("Tools"), || {
                ui.menu(state.translate("Build"), || {
                    if ui.menu_item("Web") {
                        match execute_code_web(&state.text_editor_content) {
                            Ok(_) => {
                                println!("Execution successful!");
                                state.terminal.log("Execution successful!");
                            },
                            Err(e) => { 
                                println!("Execution failed: {}", e);
                                state.terminal.log_error(format!("Execution failed: {}", e));
                            },
                        }
                    }
                    if ui.menu_item("Windows/Linux/MacOS") {
                        match build_code(&mut state) {
                            Ok(_) => {
                                println!("Build successful!");
                                state.terminal.log("Build successful!");
                            },
                            Err(e) => {
                                println!("Build failed: {}", e);
                                state.terminal.log_error(format!("Build failed: {}", e));
                            }
                        }
                    }
                });
                if ui.menu_item(state.translate("Run")) {
                    match execute_code(&mut state) {
                        Ok(_) => {
                            println!("Execution successful!");
                            state.terminal.log("Execution successful!");
                        },
                        Err(e) => {
                            println!("Execution failed: {}", e);
                            state.terminal.log_error(format!("Execution failed: {}", e));
                        },
                    }
                }
            });
            ui.menu(state.translate("Help"), || {
                if ui.menu_item(state.translate("Documentation")) {
                    // Open the link in a web browser
                    let link = "https://lados-organization.gitbook.io/goku/";
                    webbrowser::open(link).unwrap();
                }
                if ui.menu_item(state.translate("About")) {
                    state.open_about = true;
                }
            });
        });

        ui.modal_popup("goku game engine", || {
            ui.text(format!("Version: {}", about_info.version));
            ui.text(format!("Date: {}", about_info.date));
            ui.text(format!("Rust: {}", about_info.rust_version));
            ui.text(format!("Games API: {}", about_info.games_api));
            ui.text(format!("OS: {}", std::env::consts::OS));
            if ui.button("OK") {
                ui.close_current_popup();
            }
        });

        let (window_width, window_height) = canvas.window().size();
        let control_panel_width = 300.0;
        let control_panel_position: [f32; 2] = [window_width as f32 - control_panel_width, 24.0];
        /* create imgui UI here */
        ui.window("Inspector")
        .flags(imgui::WindowFlags::NO_COLLAPSE | imgui::WindowFlags::NO_RESIZE)
        .size([300.0, window_height as f32 - 223.0], imgui::Condition::Always)
        .position(control_panel_position, imgui::Condition::Always)
        .build(|| {
            match &state.selected_component {
                Some(component) if component == "Scene" => {
                    ui.input_text("Window Name", &mut state.window_name).build();
                    ui.input_int("Width", &mut state.window_width).build();
                    ui.input_int("Height", &mut state.window_height).build();
                    if ui.button("OK") {
                        println!("Window name set to: {}", state.window_name);
                        state.terminal.log(format!("Window name set to: {}", state.window_name));
                        // You can add additional logic if needed
                    }
                },
                Some(component) if component == "GameObject" => {
                    for (idx, pos) in texture_pos.iter_mut().enumerate() {
                        ui.text(format!("Texture {} position X:", idx + 1));
                        ui.slider(&format!("X{}", idx + 1), 0.0, 1000.0, &mut pos.0);
                        ui.text(format!("Texture {} position Y:", idx + 1));
                        ui.slider(&format!("Y{}", idx + 1), 0.0, 1000.0, &mut pos.1);
                    }
                    state.gameobject_position = Some(texture_pos[0]);
                    ui.text("Texture scale:");
                    ui.slider("Scale", 0.1, 10.0, &mut texture_scale);
                    // Your GUI here
                    if ui.button("Generate Template") {
                        // Set the flag to show the save dialog
                        state.show_save_dialog = true;
                        generate_template(&mut state);
                    }
                },
                Some(component) if component == "Texture" => {
                    if ui.button("Add Texture...") {
                        let file = FileDialog::new()
                            .add_filter("PNG Image", &["png"])
                            .pick_file();

                        if let Some(file_path) = file {
                            state.textures.push(TextureComponent {
                                path: file_path,
                                tag_name: String::new(),  // default empty string
                                width: 0,                 // default value, e.g., 0
                                height: 0,                // default value, e.g., 0
                            });
                        } else {
                            println!("No file chosen");
                            state.terminal.log_error("No file chosen");
                        }
                    }
                    
                    for (idx, texture) in state.textures.iter_mut().enumerate() {
                        ui.text(format!("Texture {} path: {:?}", idx + 1, texture.path));

                        // Temporary variables for ImGui input
                        let mut temp_width = texture.width as i32;
                        let mut temp_height = texture.height as i32;

                        ui.input_text("Tag Name", &mut texture.tag_name).build();
                        ui.input_int("Width", &mut temp_width).build();
                        ui.input_int("Height", &mut temp_height).build();

                        // Clamp negative values to zero (or handle as needed)
                        texture.width = temp_width.max(0) as u32;
                        texture.height = temp_height.max(0) as u32;

                        let p = texture.path.clone();
                        if ui.button(&format!("Load Texture {}", idx + 1)) {
                            let tex = texture_creator.load_texture(&p).unwrap();
                            textures.push(tex);
                            texture_pos.push((301.676, 22.346)); // Add a new position for the new texture
                            state.terminal.log(format!("Texture {:?} loaded", texture.path.to_str()));
                        }
                        state.texture_path = Some(texture.path.to_str().unwrap().to_string());
                    }                    
                },  
                Some(component) if component == "Ambient Filter" => {
                    if state.ambient_filters.is_empty() {
                        // Add a new AmbientFilterComponent with a default intensity
                        state.ambient_filters.push(AmbientFilterComponent { 
                            intensity: 0.5, 
                            color: [1.0, 1.0, 1.0, 1.0], // White color 
                        });
                    } else {
                        // If an AmbientFilterComponent already exists, update its intensity
                        if let Some(filter) = state.ambient_filters.last_mut() {
                            ui.input_float("Intensity", &mut filter.intensity)
                                .step(0.01)
                                .step_fast(0.1)
                                .build();
                            ui.color_edit4("Color", &mut filter.color);
                        }
                    }
                },
                Some(component) if component == "Audio Player" => {
                    if state.audio_player.is_none() {
                        state.audio_player = Some(AudioPlayerComponent {
                            volume: 35,
                            track_path: String::new(),
                            loop_count: -1,
                        });
                    }
                
                    if let Some(audio_player) = state.audio_player.as_mut() {
                        if ui.button("Select Track...") {
                            let file_dialog = rfd::FileDialog::new()
                                .add_filter("audio", &["mp3", "wav", "ogg"])
                                .pick_file();
                
                            if let Some(file_path) = file_dialog {
                                audio_player.track_path = file_path.to_string_lossy().to_string();
                            }
                        }
                
                        ui.text(format!("Track Path: {}", audio_player.track_path));
                        ui.input_int("Volume", &mut audio_player.volume).build();
                        ui.input_int("Loop Count", &mut audio_player.loop_count).build();
                    }
                },
                Some(component) if component == "Light" => {
                    let mut light_type = match state.light_type {
                        LightType::None => 0,
                        LightType::Point => 1,
                        LightType::Spotlight => 2,
                    };
                    let light_type_names = ["None", "Point", "Spotlight"];
                    if ui.combo("Type", &mut light_type, &light_type_names, |&x| std::borrow::Cow::Borrowed(x)) {
                        state.light_type = match light_type {
                            0 => LightType::None,
                            1 => LightType::Point,
                            2 => LightType::Spotlight,
                            _ => unreachable!(),
                        };
                    }
    
                    ui.color_edit3("Color", &mut state.light_color);
    
                    if ui.button("Select PNG...") {
                        let file = FileDialog::new()
                            .add_filter("PNG Image", &["png"])
                            .pick_file();
    
                        if let Some(file_path) = file {
                            state.light_png_path = file_path.to_str().unwrap_or_default().to_string();
                        } else {
                            println!("No file chosen");
                            state.terminal.log_error("No file chosen");
                        }
                    }
    
                    ui.text(format!("PNG Path: {}", state.light_png_path));
                },                         
                Some(component) => ui.text(component),
                None => ui.text("No component selected"),
            }
        });       

        let (_, window_height) = canvas.window().size();
        ui.window("Components")
        .flags(imgui::WindowFlags::NO_COLLAPSE | imgui::WindowFlags::NO_RESIZE)
        .size([300.0, window_height as f32 - 223.0], imgui::Condition::Always)
        .position([0.0, 24.0], imgui::Condition::Always)
        .build(|| {
            ui.text("Add component:");
            if ui.small_button("+") {
                ui.open_popup("Add Component");
            }

            ui.popup("Add Component", || {
                let mut component_types = ["Scene", "Texture", "GameObject", "Ambient Filter", "Audio Player", "Light"];
                component_types.sort();
                for component_type in &component_types {
                    if ui.selectable(component_type) {     
                        let new_component = Component {
                            name: component_type.to_string(),
                            children: Vec::new(),
                        };

                        match state.selected_component.as_ref() {
                            None => state.components.push(new_component),
                            Some(selected) => {
                                if let Some(parent) = state.components.iter_mut().find(|component| &component.name == selected) {
                                    parent.children.push(new_component);
                                }
                            }
                        }
                    }
                }
            });

            let components_copy = state.components.clone();
            let mut display_component_tree = DisplayComponentTree { ui, state: &mut state };
            display_component_tree.display(&components_copy, 0);
        });

        // Terminal
        let (_, window_height) = canvas.window().size();
        let terminal_height = 200.0; // Adjust as needed
        let terminal_position: [f32; 2] = [0.0, window_height as f32 - terminal_height];
        ui.window("Console")
          .flags(imgui::WindowFlags::NO_RESIZE)
          .size([window_width as f32, window_height as f32], imgui::Condition::Always)
          .position(terminal_position, imgui::Condition::Always)
          .build(|| {
              if let Some(tab_bar) = ui.tab_bar("##tabbar") {
                  if ui.tab_item("Log").is_some() {
                    // Terminal UI code
                    state.terminal.display(&ui, false);
                    // Add more logic as needed for the terminal content
                  }
                  if ui.tab_item("Problems").is_some() {
                    // Terminal UI code
                    state.terminal.display(&ui, true);
                    // Add more logic as needed for the terminal content
                }
                  tab_bar.end();
              }
            });

        if state.open_preferences {
            // Assume we have an open_preferences_flag that indicates whether the preferences window should be opened.
            let mut open_preferences_flag = state.open_preferences;

            ui.window(state.translate("Preferences"))
                .size([400.0, 300.0], imgui::Condition::FirstUseEver)
                .flags(imgui::WindowFlags::NO_RESIZE)
                .opened(&mut open_preferences_flag) // Use a temporary flag instead of borrowing state directly
                .build(|| {
                    ui.text(state.translate("Preferences Categories:"));
                    ui.separator();
                    ui.menu("General", || {
                        ui.text(state.translate("General settings"));
                        ui.separator();
                        // You can add sliders, checkboxes, and other controls for various general settings
                        ui.checkbox(state.translate("Enable Fullscreen"), &mut state.general_settings.enable_fullscreen);
                        ui.checkbox(state.translate("Enable canvas present"), &mut state.canvas_present);
                        ui.checkbox(state.translate("Enable VSync"), &mut state.general_settings.enable_vsync);
                        // Add controls for general settings here
                        ui.separator();
                        if ui.input_text(state.translate("Font Name"), &mut state.general_settings.font_name).build() {
                            state.general_settings.font_change_requested = true;
                        }
                        if ui.slider(state.translate("Font Size"), 10.0, 24.0, &mut state.general_settings.font_size) {
                            state.general_settings.font_change_requested = true;
                        }
                    });
                    ui.menu(state.translate("Input"), || {
                        ui.checkbox(state.translate("Enable Input Handler"), &mut state.general_settings.enable_input_handler);
                        // Add controls for input settings here
                    });
                    let languages = ["English", "Deutsch", "Español", "Français", "日本語"];
                    
                    let mut current_item = languages
                        .iter()
                        .position(|language| language == &state.general_settings.language)
                        .unwrap_or(0);

                    if ui.combo(
                        state.translate("Language"),
                        &mut current_item,
                        &languages,
                        |language| language.to_string().into(),
                    ) {
                        state.general_settings.language = languages[current_item].to_string();
                    }
                    // Add more categories as needed
                });
            // Update state with the flag value after the closure
            state.open_preferences = open_preferences_flag;
        }

        if state.general_settings.font_change_requested {
            if let Err(e) = state.save_settings() {
                eprintln!("Error saving settings: {}", e);
                // Handle the error appropriately
            }
            state.general_settings.font_change_requested = false;
        }
        
        if state.open_image_view {
            ui.window("Image Window")
            .size([300.0, 300.0], imgui::Condition::FirstUseEver)
            .opened(&mut state.open_image_view)
            .build(|| {
                // Check if we have a valid texture ID and display it
                if let Some(tex_id) = state.dynamic_texture_id {
                    let image = imgui::Image::new(imgui::TextureId::from(tex_id as usize), [300.0, 300.0]);
                    image.build(&ui);
                } else {
                    ui.text("No image loaded.");
                }
            });
        }

        if state.open_text_editor {
            ui.window(state.translate("Text Editor"))
                .size([600.0, 600.0], imgui::Condition::FirstUseEver)
                .position([350.0, 10.0], imgui::Condition::FirstUseEver)
                .opened(&mut state.open_text_editor)
                .build(|| {
                    if ui.input_text_multiline("##texteditor", &mut state.text_editor_content, [550.0, 550.0])
                        .flags(imgui::InputTextFlags::ALLOW_TAB_INPUT | imgui::InputTextFlags::CTRL_ENTER_FOR_NEW_LINE)
                        .build()
                    {
                        state.undo_stack.push(state.text_editor_content.clone());
                        state.redo_stack.clear();
                        // state.text_editor_content is now updated with the changes
                    }
            });
        }

        if state.show_save_dialog {
            if let Some(want) = user_wants_to_save(&ui, &mut state.show_save_dialog) {
                if want {
                    state.show_save_dialog_file = true;
                    if state.show_save_dialog_file {
                        if let Some((file_path_str, sc_path_str)) = save_project() {
                            state.project_dir = std::path::PathBuf::from(file_path_str);
                            let package_name = state.project_dir.file_name().unwrap().to_str().unwrap().replace(".", "_");
                            // Create a new cargo project if it doesn't exist
                            if !state.project_dir.exists() {
                                match std::process::Command::new("cargo")
                                        .arg("new")
                                        .arg("--bin")
                                        .arg("--name")
                                        .arg(&package_name)
                                        .arg(&state.project_dir)
                                        .status() {
                                    Ok(status) => {
                                        if !status.success() {
                                            eprintln!("Failed to create a new cargo project");
                                            state.terminal.log_error("Failed to create a new cargo project");
                                            // Handle error appropriately
                                        }
                                    },
                                    Err(e) => {
                                        eprintln!("Cargo command failed: {}", e);
                                        state.terminal.log_error(format!("Cargo command failed: {}", e));
                                        // Handle error appropriately
                                    }
                                }
                            }
        
                            // Append the required dependencies to Cargo.toml
                            let cargo_toml_path = state.project_dir.join("Cargo.toml");
                            let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
                            cargo_toml.push_str("\nnalgebra = \"0.32.2\"\nsdl2-sys = \"0.35.2\"\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"\nserde_derive = \"1.0.163\"\nrand = \"0.8.5\"\n");
                            cargo_toml.push_str("sdl2 = { version = \"0.35\", default-features = false, features = [\"image\", \"ttf\", \"mixer\"] }\n");
                            std::fs::write(&cargo_toml_path, cargo_toml).expect("Failed to write to Cargo.toml");
        
                            // Write the content to the main.rs file
                            let main_rs_path = state.project_dir.join("src").join("main.rs");
                            std::fs::write(&main_rs_path, state.text_editor_content.as_str()).expect("Failed to write to main.rs");
                            
                            // Handle two_d module copying
                            match handle_two_d_module(&state.project_dir) {
                                Ok(_) => {
                                    println!("two_d module was placed");
                                    state.terminal.log("two_d module was placed");
                                },
                                Err(e) => {
                                    eprintln!("Failed to place two_d module: {}", e);
                                    state.terminal.log_error(format!("Failed to place two_d module: {}", e));
                                }
                            }

                            if is_vscode_installed() {
                                match execute_command("code.cmd", &main_rs_path) {
                                    Ok(_) => {
                                        println!("VSCode opened with the project");
                                        state.terminal.log("VSCode was found and opened with the project");
                                    },
                                    Err(e) => {
                                        eprintln!("Failed to open project with VSCode: {}", e);
                                        state.terminal.log_error(format!("Failed to open project with VSCode: {}", e));
                                    },
                                }
                            } else {
                                eprintln!("Visual Studio Code is not installed or not in PATH.");
                                state.terminal.log_error("Visual Studio Code is not installed or not in PATH");
                            }

                            // Save the .sc file
                            if save_project_to_path(&sc_path_str, &state).is_err() {
                                eprintln!("Failed to save .sc file to {:?}", sc_path_str);
                                state.terminal.log_error(format!("Failed to save .sc file to {:?}", sc_path_str));
                            }
        
                            println!("Project saved to {:?}", state.project_dir);
                            state.terminal.log(format!("Project saved to {:?}", state.project_dir));
                        } else {
                            eprintln!("Failed to save project");
                            state.terminal.log_error("Failed to save project");
                        }
                    }
                } else {
                    state.terminal.log("Save canceled by user");
                }
            }
        }        
                
        /* render texture at the position specified by the slider */
        canvas.clear();
        let texture_width = (50.0 * texture_scale) as u32;
        let texture_height = (50.0 * texture_scale) as u32;
        for (idx, texture) in textures.iter().enumerate() { 
            let rect = sdl2::rect::Rect::new(texture_pos[idx].0 as i32, texture_pos[idx].1 as i32, texture_width, texture_height);
            canvas.copy(texture, None, rect).unwrap();
        }

        /* render */
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        platform.prepare_render(&ui, &canvas.window());
        renderer.render(&mut imgui);
        canvas.window().gl_swap_window();

        if state.canvas_present {
            canvas.present();
        }
        canvas.clear();
    }
}

fn is_vscode_installed() -> bool {
    if cfg!(target_os = "windows") {
        // Windows-specific logic
        std::process::Command::new("cmd")
            .args(["/C", "code --version"])
            .output()
            .is_ok()
    } else {
        // Unix-like OS logic
        std::process::Command::new("sh")
            .arg("-c")
            .arg("code --version")
            .output()
            .is_ok()
    }
}

fn generate_template(state: &mut State) {
    let window_title = &state.window_name;
    let window_width = state.window_width;
    let window_height = state.window_height;
    let mut content = format!(r#"
mod two_d;
use nalgebra::Vector2;
use std::path::Path;
use crate::two_d::{{Window, TextureManagerAnim, GameObject, Camera, InputHandler, AmbientFilter, PointLight, SpotLight}};

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let mut window = Window::new("{}", {}, {})?;

    let mut last_frame_time = unsafe {{ sdl2::sys::SDL_GetTicks() }};
    let mut current_frame_time;
    let mut delta_time;
    
    // Create a camera object
    let mut camera = Camera::new(Vector2::new(0, 0), Vector2::new({}, {}));

    let texture_creator = window.canvas.texture_creator();
    "#, window_title, window_width, window_height, window_width, window_height);

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

fn user_wants_to_save(ui: &imgui::Ui, show_save_dialog: &mut bool) -> Option<bool> {
    let mut result = None;

    if *show_save_dialog {
        ui.open_popup("Save Scene?");
    }

    ui.modal_popup("Save Scene?", || {
        ui.text("Do you want to save the scene?");

        if ui.button("Yes") {
            result = Some(true);
            *show_save_dialog = false;
            ui.close_current_popup();
        }

        ui.same_line();

        if ui.button("No") {
            result = Some(false);
            *show_save_dialog = false;
            ui.close_current_popup();
        }
    });

    result
}