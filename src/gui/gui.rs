use imgui::Context;
use rfd::FileDialog;
use copypasta::ClipboardProvider;
use sdl2::surface::Surface;
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
use crate::gui::main_functionality::user_wants_to_save;
use crate::gui::main_functionality::is_vscode_installed;
use crate::gui::generate_template::generate_template;
use crate::gui::shader::Image;
use crate::gui::shader::ViewportState;
use crate::gui::shader::compile_shader;
use crate::gui::shader::link_program;
use crate::gui::shader::sdl_surface_to_gl_texture;
use crate::gui::shader::VERTEX_SHADER_SOURCE;
use crate::gui::shader::FRAGMENT_SHADER_SOURCE;
use crate::gui::shader::load_texture_from_drop_event;
use crate::gui::shader::generate_grid_vertices;
use crate::gui::shader::create_grid_shader_program;
use crate::gui::grid_view::Grid;
use crate::deepl::deepl::call_python_add_function;
use sdl2::image::{LoadSurface, InitFlag};
use std::ffi::CString;
use std::ptr;

pub fn launcher() -> Result<(), String> {
    sdl2::image::init(InitFlag::PNG)?;
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
        .window("Goku Engine", 1280, 720)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();

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
    let mut image_path:Vec<String> = Vec::new();
    let mut textures:Vec<Image> = Vec::new();
    let mut current_pos_x = 301.676; // Starting position for the first image
    let pos_offset_x = 301.676; // Horizontal offset between images

    /* start main loop */
    let mut event_pump = sdl.event_pump().unwrap();

    let mut is_light_theme = false; // Boolean to track the current theme (You can manage this in your state if required)

    let about_info = AboutInfo {
        version: String::from("0.1.3"),
        date: String::from("02-10-2023"),
        rust_version: String::from("1.68"),
        games_api: String::from("SDL2"),
        os: std::env::consts::OS.to_string(),
    };

    // Set up shader program
    let vert_shader = compile_shader(gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE)?;
    let frag_shader = compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)?;
    let shader_program = link_program(vert_shader, frag_shader)?;

    // Set up vertex data (and buffer(s)) and configure vertex attributes
    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        let vertices: [f32; 20] = [
            // positions    // texture coords
            0.0,  0.0, 0.0,  1.0, 0.0,   // top right
            0.0, -0.5, 0.0,  1.0, 1.0,   // bottom right
            -0.5, -0.5, 0.0, 0.0, 1.0,   // bottom left
            -0.5,  0.0, 0.0, 0.0, 0.0    // top left 
        ];
        let indices: [u32; 6] = [
            0, 1, 3, // first triangle
            1, 2, 3  // second triangle
        ];
        let mut ebo = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    vertices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                    indices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW);

        let stride = 5 * std::mem::size_of::<f32>() as gl::types::GLint;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // texture coord attribute
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
        gl::EnableVertexAttribArray(1);
    }

    let mut viewport_state = ViewportState::new();
    let mut selected_image_index: Option<usize> = None;

    let grid = Grid::new(50.0, [0.8, 0.8, 0.8, 1.0]);
    let grid_vertices = generate_grid_vertices(grid.spacing, 800.0, 600.0);
    let grid_shader_program = create_grid_shader_program()?;
    let (mut grid_vao, mut grid_vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut grid_vao);
        gl::GenBuffers(1, &mut grid_vbo);

        gl::BindVertexArray(grid_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, grid_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (grid_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            grid_vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<f32>() as gl::types::GLint, ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platform */
            platform.handle_event(&mut imgui, &event);

            match event {
                sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                    // Update the viewport offset based on WASD keys
                    viewport_state.update_offset(keycode, 10.0); // Adjust the delta value as needed for sensitivity
                },
                sdl2::event::Event::MouseButtonDown { x, y, mouse_btn: sdl2::mouse::MouseButton::Left, .. } => {
                    // Check if the click is within any of the image boundaries
                    for image in &mut textures {
                        let x_float = x as f32;
                        let y_float = y as f32;
                        let texture_width = image.width as f32; // Width of the loaded image
                        let texture_height = image.height as f32; // Height of the loaded image
                        if x_float >= image.pos_x && x_float <= image.pos_x + texture_width &&
                            y_float >= image.pos_y && y_float <= image.pos_y + texture_height {
                                // Start dragging if clicked inside the image
                                image.is_dragging = true;
                                // Store initial click position relative to image position
                                let click_offset_x = x_float - image.pos_x;
                                let click_offset_y = y_float - image.pos_y;
                                // Store the offset for later use during dragging
                                image.offset_x = click_offset_x;
                                image.offset_y = click_offset_y;
                            }
                    }
                },
                // Mouse button up event
                sdl2::event::Event::MouseButtonUp { .. } => {
                    // Reset the dragging flag
                    for image in &mut textures {
                        image.is_dragging = false;
                    }
                },
                sdl2::event::Event::MouseMotion { x, y, mousestate, .. } if mousestate.left() => {
                    // Update image position only if dragging
                    for image in &mut textures {
                        if image.is_dragging {
                            // Update image position based on mouse movement
                            image.pos_x = x as f32 - image.offset_x;
                            image.pos_y = y as f32 - image.offset_y;
                        }
                    }
                },
                sdl2::event::Event::MouseWheel { y, .. } => {
                    // Zoom in or out when the mouse wheel is used
                    for image in &mut textures {
                        image.scale += y as f32 * 0.1; // Adjust the scale factor based on the wheel movement
                        image.scale = image.scale.max(0.1).min(10.0); // Constrain the scale factor to reasonable values
                    }
                },
                sdl2::event::Event::DropFile { ref filename, .. } => {
                    if let Ok((texture_id, width, height)) = load_texture_from_drop_event(&event) {
                        // Assuming a single frame and row for simplicity
                        textures.push(Image::new(texture_id, width, height, 4, 3, 50.0, 50.0, 0));
                        state.textures.push(TextureComponent {
                            path: std::path::PathBuf::from(filename),
                            tag_name: String::new(), // Default empty string
                            width, // Image width
                            height, // Image height
                            frames: 4, // Default single frame
                            rows: 3, // Default single row
                        });
                    }
                },
                sdl2::event::Event::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(new_width, new_height) => {
                            unsafe {
                                let (win_width, win_height) = (new_width as f32, new_height as f32);
                                // Adjust image position and scale
                                for image in &mut textures {
                                    image.pos_x *= new_width as f32 / win_width;
                                    image.pos_y *= new_height as f32 / win_height;
                                }

                                gl::Viewport(0, 0, new_width, new_height); // Update viewport
                            }
                            
                        },
                        _ => {}
                    }
                },
                sdl2::event::Event::Quit { .. } => {
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
            ui.open_popup("Goku Game Engine");
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
                                    match sdl_surface_to_gl_texture(&surface) {
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
                        match execute_code_web(&mut state) {
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
                // TODO: Tilemap
                if ui.menu_item("Tilemap") {
                    // TODO: Use function to draw a tilemap
                    println!("This is tilemap");
                    state.terminal.log("Tilemap");
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

        ui.modal_popup("Goku Game Engine", || {
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
                    for (idx, pos) in textures.iter_mut().enumerate() {
                        ui.text(format!("Texture {} position X:", idx + 1));
                        ui.slider(&format!("X{}", idx + 1), 0.0, 1000.0, &mut pos.pos_x);
                        ui.text(format!("Texture {} position Y:", idx + 1));
                        ui.slider(&format!("Y{}", idx + 1), 0.0, 1000.0, &mut pos.pos_y);
                        ui.text("Texture scale:");
                        ui.slider("Scale", 0.1, 10.0, &mut pos.scale);
                    }
                    
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
                                frames: 1, // Initialize with 1, will be updated later in UI
                                rows: 3,
                            });
                        } else {
                            println!("No file chosen");
                            state.terminal.log_error("No file chosen");
                        }
                    }
                
                    let mut width_changes = vec![0; state.textures.len()];
                    let mut height_changes = vec![0; state.textures.len()];
                
                    for (idx, texture) in state.textures.iter_mut().enumerate() {
                        ui.text(format!("Texture {} path: {:?}", idx + 1, texture.path));
                        
                        let original_width = texture.width;
                        let original_height = texture.height;
                
                        let mut temp_width = texture.width as i32;
                        let mut temp_height = texture.height as i32;
                
                        ui.input_text(format!("Tag Name {}", idx + 1), &mut texture.tag_name).build();
                        if ui.input_int(format!("Width {}", idx + 1), &mut temp_width).build() {
                            width_changes[idx] = temp_width as u32 - original_width;
                        }
                        if ui.input_int(format!("Height {}", idx + 1), &mut temp_height).build() {
                            height_changes[idx] = temp_height as u32 - original_height;
                        }
                
                        // Move frames input here to ensure it's visible and editable for each texture
                        let mut frames = texture.frames as i32;
                        if ui.input_int(format!("Frames {}", idx + 1), &mut frames).build() {
                            texture.frames = frames.max(1) as u32; // Ensure frames is at least 1
                        }
                
                        texture.width = (original_width as i32 + width_changes[idx] as i32).max(0) as u32;
                        texture.height = (original_height as i32 + height_changes[idx] as i32).max(0) as u32;
                
                        if ui.button(&format!("Load Texture {}", idx + 1)) {
                            let tex_path_str = texture.path.to_str().unwrap_or_default();
                            if !image_path.contains(&tex_path_str.to_string()) {
                                let tex = Surface::from_file(&tex_path_str).unwrap();
                                state.surf_texture_id = sdl_surface_to_gl_texture(&tex).unwrap();
                                textures.push(Image::new(state.surf_texture_id, tex.width(), tex.height(), texture.frames as usize, texture.rows, current_pos_x, 50.0, 0));
                                current_pos_x += pos_offset_x;
                                state.terminal.log(format!("Texture {:?} loaded", texture.path.to_str()));
                                image_path.push(tex_path_str.to_string());
                            }
                        }
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
                        ui.checkbox("Grid View", &mut state.enable_grid_view);
                        let selected_index = selected_image_index; // Cache selected_image_index to avoid borrowing issues
                        for (idx, tex) in textures.iter_mut().enumerate() {
                            let label = format!("Image {} - {}", idx, tex.texture_id);
                            if ui.selectable(&label) {
                                selected_image_index = Some(idx); // User has selected this image
                            }
                        }

                        if let Some(selected_index) = selected_index {
                            let mut layer = textures[selected_index].layer;
                            let mut selected_row_i32 = textures[selected_index].selected_row as i32; // Temporary i32 for imgui
                            ui.input_int("Row", &mut selected_row_i32).build();
                            if selected_row_i32 >= 0 {
                                textures[selected_index].selected_row = selected_row_i32 as usize; // Convert back to usize
                                textures[selected_index].set_selected_row(selected_row_i32 as usize);
                            }
                            ui.checkbox("Enable Animation", &mut textures[selected_index].animation); // Toggle animation for the selected image
                            if ui.input_int("Layer", &mut layer).build() {
                                textures[selected_index].layer = layer;
                            }
                        }

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

        /* render */
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        if state.canvas_present {
            // Inside the main loop, before rendering
            let (win_width, win_height) = canvas.window().size();
            
            textures.sort_by_key(|img| img.layer);
            for image in &mut textures {
                image.update();
                if image.selected_row != image.current_row { // Add this line to restrict animation to the selected row
                    image.current_row = image.selected_row;
                }
            }

            unsafe {
                gl::ClearColor(0.5, 0.5, 0.5, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                if state.enable_grid_view {
                    // Draw Grid
                    gl::UseProgram(grid_shader_program);
                    gl::BindVertexArray(grid_vao);
                    
                    let grid_u_color = CString::new("uColor").unwrap();
                    let color_location = gl::GetUniformLocation(grid_shader_program, grid_u_color.as_ptr());
                    gl::Uniform4f(color_location, grid.color[0], grid.color[1], grid.color[2], grid.color[3]);
                    
                    let grid_u_offset = CString::new("uOffset").unwrap();
                    let offset_location = gl::GetUniformLocation(grid_shader_program, grid_u_offset.as_ptr());
                    gl::Uniform2f(offset_location, viewport_state.offset_x / win_width as f32 * 2.0, viewport_state.offset_y / win_height as f32 * 2.0);
                    gl::DrawArrays(gl::LINES, 0, (grid_vertices.len() / 2) as i32);
                }
    
                gl::UseProgram(shader_program);
                gl::BindVertexArray(vao);
                for (index, image) in textures.iter().enumerate() {
                    gl::ActiveTexture(gl::TEXTURE0 + index as u32);
                    gl::BindTexture(gl::TEXTURE_2D, image.texture_id);
                    
                    let pos_name = CString::new("uImagePos").unwrap();
                    let pos_uniform = gl::GetUniformLocation(shader_program, pos_name.as_ptr());
                    let scale_name = CString::new("uScale").unwrap();
                    let scale_uniform = gl::GetUniformLocation(shader_program, scale_name.as_ptr());
                    let frame_name = CString::new("uCurrentFrame").unwrap();
                    let frame_uniform = gl::GetUniformLocation(shader_program, frame_name.as_ptr());
                    let frames_name = CString::new("uFrames").unwrap();
                    let frames_uniform = gl::GetUniformLocation(shader_program, frames_name.as_ptr());
                    
                    let row_name = CString::new("uCurrentRow").unwrap();
                    let row_uniform = gl::GetUniformLocation(shader_program, row_name.as_ptr());
                    let rows_name = CString::new("uRows").unwrap();
                    let rows_uniform = gl::GetUniformLocation(shader_program, rows_name.as_ptr());

                    let adjusted_pos_x = image.pos_x + viewport_state.offset_x;
                    let adjusted_pos_y = image.pos_y + viewport_state.offset_y;
                    let normalized_x = (adjusted_pos_x / win_width as f32) * 2.0 - 1.0;
                    let normalized_y = 1.0 - (adjusted_pos_y / win_height as f32) * 2.0;
                    gl::Uniform2f(pos_uniform, normalized_x, normalized_y);
                    gl::Uniform1f(scale_uniform, image.scale);
                    gl::Uniform1i(frame_uniform, image.current_frame as i32);
                    gl::Uniform1i(frames_uniform, image.frames as i32);

                    gl::Uniform1i(row_uniform, image.current_row as i32);
                    gl::Uniform1i(rows_uniform, image.rows as i32);

                    let texture_name = CString::new("textureSampler").unwrap();
                    let texture_uniform = gl::GetUniformLocation(shader_program, texture_name.as_ptr());
                    gl::Uniform1i(texture_uniform, index as i32);

                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                }
            }
        }

        platform.prepare_render(&ui, &canvas.window());
        renderer.render(&mut imgui);
        canvas.window().gl_swap_window();
    }
}
