use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
    event::Event,
    video::{GLProfile, Window}
};
use sdl2::image::LoadTexture;
use std::path::Path;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::io::Write;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::fmt::Display;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct Component {
    name: String,
    children: Vec<Component>,
}

#[derive(Serialize, Deserialize)]
struct GeneralSettings {
    enable_fullscreen: bool,
    enable_vsync: bool,
    language: String,
    enable_input_handler: bool,
    font_name: String,
    font_size: f32,
    font_change_requested: bool, // Add this
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextureComponent {
    pub path: std::path::PathBuf,
    pub tag_name: String,
    pub width: u32,
    pub height: u32,
}

impl Default for TextureComponent {
    fn default() -> Self {
        Self {
            path: std::path::PathBuf::new(),
            tag_name: String::new(),
            width: 0,
            height: 0,
        }
    }
}

#[derive(PartialEq)]
enum MessageType {
    Info,
    Error,
}

struct LogMessage {
    timestamp: DateTime<Utc>,
    message_type: MessageType,
    content: String,
}

impl LogMessage {
    fn new(message_type: MessageType, content: String) -> Self {
        let timestamp = Utc::now();  // Using chrono to get the current time

        Self {
            timestamp,
            message_type,
            content,
        }
    }

    fn to_string(&self) -> String {
        let type_str = match self.message_type {
            MessageType::Info => "[INFO]",
            MessageType::Error => "[ERROR]",
        };

        // Format the timestamp into a readable string
        let formatted_timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

        format!("{} - {} {}", formatted_timestamp, type_str, self.content)
    }
}

#[derive(Default)]
struct Terminal {
    content: Vec<LogMessage>,
    max_lines: usize,
}

impl Terminal {
    fn new(max_lines: usize) -> Self {
        Self {
            content: Vec::new(),
            max_lines,
        }
    }
    // Updated methods to handle LogMessage...
    fn log<T: Display>(&mut self, message: T) {
        self.add_message(MessageType::Info, format!("{}", message));
    }

    fn log_error<T: Display>(&mut self, message: T) {
        self.add_message(MessageType::Error, format!("{}", message));
    }

    fn add_message(&mut self, message_type: MessageType, content: String) {
        if self.max_lines > 0 && self.content.len() >= self.max_lines {
            self.content.remove(0);
        }
        self.content.push(LogMessage::new(message_type, content));
    }
    

    fn display(&self, ui: &imgui::Ui, only_errors: bool) {
        for message in &self.content {
            if only_errors && message.message_type != MessageType::Error {
                continue; // Skip non-error messages if only_errors is true
            }

            let text = message.to_string();
            let color = match message.message_type {
                MessageType::Info => [0.0, 1.0, 0.0, 1.0], // Green for info
                MessageType::Error => [1.0, 0.0, 0.0, 1.0], // Red for error
            };
            ui.text_colored(color, &text);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TranslationRequest {
    original_text: String,
    translated_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AmbientFilterComponent {
    pub intensity: f32,
    pub color: [f32; 4], // RGBA color
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AudioPlayerComponent {
    pub volume: i32,
    pub track_path: String,
    pub loop_count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    selected_component: Option<String>,
    components: Vec<Component>,
    #[serde(skip)]
    open_about: bool,
    open_text_editor: bool,
    canvas_present: bool,
    text_editor_content: String,
    textures: Vec<TextureComponent>,
    selected_texture_path: std::path::PathBuf,
    #[serde(skip)]
    open_preferences: bool,
    general_settings: GeneralSettings,
    gameobject_position: Option<(f32, f32)>,
    texture_path: Option<String>,
    #[serde(skip)]
    terminal: Terminal,
    #[serde(skip)]
    undo_stack: Vec<String>,
    #[serde(skip)]
    redo_stack: Vec<String>,
    #[serde(skip)]
    search_query: String,
    #[serde(skip)]
    search_result_index: usize,
    #[serde(skip)]
    search_results: Vec<usize>,
    #[serde(skip)]
    clipboard: Option<ClipboardContext>,
    #[serde(skip)]
    exit_requested: bool,
    #[serde(skip)]
    window_name: String,
    #[serde(skip)]
    translations: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    #[serde(skip)]
    show_save_dialog: bool,
    #[serde(skip)]
    show_save_dialog_file: bool,
    #[serde(skip)]
    project_dir: std::path::PathBuf,
    #[serde(skip)]
    ambient_filters: Vec<AmbientFilterComponent>,
    #[serde(skip)]
    audio_player: Option<AudioPlayerComponent>,
    #[serde(skip)]
    window_width: i32,
    #[serde(skip)]
    window_height: i32,
}

impl State {
    fn new() -> Self {
        let mut state = Self {
            components: Vec::new(),
            selected_component: None,
            open_about: false,
            open_text_editor: false,
            canvas_present: false,
            text_editor_content: String::with_capacity(10000),
            textures: Vec::new(),
            selected_texture_path: std::path::PathBuf::new(),
            open_preferences: false,
            general_settings: GeneralSettings {
                enable_fullscreen: false,
                enable_vsync: false,
                language: "English".to_string(),
                enable_input_handler: false,
                font_name: "ARIALUNI".to_string(),
                font_size: 18.0,
                font_change_requested: false,
            },
            gameobject_position: None,
            texture_path: None,
            terminal: Terminal::new(50),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            search_query: "".to_string(),
            search_result_index: 0,
            search_results: Vec::new(),
            clipboard: ClipboardContext::new().ok(),
            exit_requested: false,
            window_name: "".to_string(),
            translations: std::collections::HashMap::new(),
            show_save_dialog: false,
            show_save_dialog_file: false,
            project_dir: std::path::PathBuf::new(),
            ambient_filters: Vec::new(),
            audio_player: None,
            window_width: 0,
            window_height: 0,
        };

        if let Err(e) = state.load_settings() {
            eprintln!("Error loading settings: {}", e);
            // Handle the error or provide default settings
        }
        
        state
    }

    fn save_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_json = serde_json::to_string(&self.general_settings)?;
        std::fs::write("settings.json", settings_json)?;
        Ok(())
    }

    fn load_settings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_json = std::fs::read_to_string("settings.json")?;
        self.general_settings = serde_json::from_str(&settings_json)?;
        Ok(())
    }

    fn load_translations(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        self.translations = serde_json::from_reader(file)?;
        Ok(())
    }

    fn translate(&self, text: &str) -> String {
        if let Some(language_map) = self.translations.get(&self.general_settings.language) {
            if let Some(translated) = language_map.get(text) {
                return translated.clone();
            }
        }
        text.to_string()
    }

    fn search_for_next(&mut self) {
        if !self.search_query.is_empty() {
            // Collect all indices of the search results
            self.search_results = self.text_editor_content.match_indices(&self.search_query).map(|(idx, _)| idx).collect();

            if !self.search_results.is_empty() {
                self.search_result_index = (self.search_result_index + 1) % self.search_results.len();
                // You can use this index to highlight or point to the occurrence in the UI.
            }
        }
    }

    fn search_for_previous(&mut self) {
        if !self.search_query.is_empty() && !self.search_results.is_empty() {
            if self.search_result_index == 0 {
                self.search_result_index = self.search_results.len() - 1;
            } else {
                self.search_result_index -= 1;
            }
            // Use this index to highlight or point to the occurrence in the UI.
        }
    }

    fn get_line_containing_index(&self, index: usize) -> Option<String> {
        let start = self.text_editor_content[..index].rfind('\n').map_or(0, |i| i + 1); // find previous newline or start of text
        
        let end_offset = self.text_editor_content[index..].find('\n').unwrap_or_else(|| self.text_editor_content[index..].len()); // find next newline or end of text in the slice
        let end = index + end_offset;

        Some(self.text_editor_content[start..end].to_string())
    }
}

pub struct AboutInfo {
    pub version: String,
    pub date: String,
    pub rust_version: String,
    pub games_api: String,
    pub os: String,
}

struct DisplayComponentTree<'a> {
    ui: &'a imgui::Ui,
    state: &'a mut State,
}

impl<'a> DisplayComponentTree<'a> {
    fn display(&mut self, components: &[Component], level: usize) {
        for component in components {
            // Construct the label for the tree node, including the indentation
            let label = format!("{:indent$}{}", "", component.name, indent = level * 2);
    
            // Create a tree node for each component
            if let Some(node) = self.ui.tree_node(&imgui::ImString::new(label)) {
                // If the node is clicked, update the selected component
                if self.ui.is_item_clicked() {
                    self.state.selected_component = Some(component.name.clone());
                }
    
                // Recursively display child components, increasing the level for indentation
                self.display(&component.children, level + 1);
    
                // End the tree node
                node.end();
            }
        }
    }    
}

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    let gl = unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    };

    unsafe {
        gl.enable(glow::DEPTH_TEST);
        gl.depth_func(glow::LEQUAL);
    }

    unsafe {
        gl.polygon_offset(1.0, 1.0); // Set the factors to your needs
        gl.enable(glow::POLYGON_OFFSET_FILL);
    }

    gl
}

#[allow(dead_code)]
pub fn cut_selected_text(text: &str, range: std::ops::Range<usize>) -> (String, String) {
    let selected_text = text[range.clone()].to_string();
    let remaining_text = format!("{}{}", &text[..range.start], &text[range.end..]);
    (selected_text, remaining_text)
}

pub fn execute_code(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = &state.project_dir; // Assuming `project_dir` is stored in state

    // Build the cargo project
    let build_status = std::process::Command::new("cargo")
        .arg("build")
        .current_dir(project_dir)
        .status()?;

    if !build_status.success() {
        state.terminal.log_error("Failed to compile the code");
        return Err("Failed to compile the code".into());
    }

    // Run the built cargo project
    let output = std::process::Command::new("cargo")
        .arg("run")
        .current_dir(project_dir)
        .output()?;

    if !output.status.success() {
        state.terminal.log_error("Failed to execute the code");
        return Err("Failed to execute the code".into());
    }

    // If everything was successful, print the output
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    state.terminal.log(format!("Output:\n{}", String::from_utf8_lossy(&output.stdout)));

    Ok(())
}

fn handle_two_d_module(temp_dir: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let two_d_src = Path::new("src/two_d.rs");
    let two_d_dest = temp_dir.join("src").join("two_d.rs");
    std::fs::copy(&two_d_src, &two_d_dest)?;

    let two_d_dir = temp_dir.join("src").join("two_d");
    std::fs::create_dir_all(&two_d_dir)?;

    let original_two_d_path = std::env::current_dir()?.join("src").join("two_d");
    for entry in std::fs::read_dir(original_two_d_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().ok_or("Failed to get file name")?.to_owned();
            std::fs::copy(&path, two_d_dir.join(filename))?;
        }
    }
    Ok(())
}

pub fn execute_code_web(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("temp_cargo_project");

    // Ensure temp_dir exists
    if !temp_dir.exists() {
        // Create a new cargo project in the temp_dir
        let status = std::process::Command::new("cargo")
            .arg("new")
            .arg("--bin")
            .arg(&temp_dir)
            .status()?;

        if !status.success() {
            return Err("Failed to create a new cargo project".into());
        }
    }

    // Copy the Emscripten SDK directory to temp_dir
    let source_path = std::path::PathBuf::from("emsdk/");
    let destination_path = temp_dir.join("emsdk");

    // Copy directory contents
    copy_directory(&source_path, &destination_path)?;

    // Install Emscripten in the emsdk directory within temp_dir
    let emsdk_env_cmd_path = destination_path.join(if cfg!(target_os = "windows") { "emsdk_env.bat" } else { "emsdk_env.sh" });
    let emsdk_cmd_path = destination_path.join("emsdk");

    execute_command(if cfg!(target_os = "windows") { "cmd" } else { "sh" }, &emsdk_env_cmd_path)?;
    execute_command(if cfg!(target_os = "windows") { "cmd" } else { "sh" }, &emsdk_cmd_path.join("activate").join("latest"))?;

    // Write code to main.rs
    let main_rs_path = temp_dir.join("src").join("main.rs");
    std::fs::write(&main_rs_path, code)?;

    // Copy two_d.rs
    let two_d_path = Path::new("src/two_d.rs");
    let destination_two_d_path = temp_dir.join("src").join("two_d.rs");
    std::fs::copy(two_d_path, &destination_two_d_path)?;

    // Append dependencies to Cargo.toml
    let cargo_toml_path = temp_dir.join("Cargo.toml");
    append_dependencies_to_cargo_toml(&cargo_toml_path)?;

    // Create and write Web.toml
    let web_toml_path = temp_dir.join("Web.toml");
    create_web_toml(&web_toml_path)?;

    // Cleanup
    std::fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

pub fn build_code(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = &state.project_dir; // Assuming `project_dir` is stored in state

    // Build the cargo project
    let build_status = std::process::Command::new("cargo")
        .arg("build")
        .current_dir(project_dir)
        .status()?;

    if !build_status.success() {
        state.terminal.log_error("Failed to compile the code");
        return Err("Failed to compile the code".into());
    }

    Ok(())
}

fn append_dependencies_to_cargo_toml(cargo_toml_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut cargo_toml = std::fs::read_to_string(cargo_toml_path)?;
    cargo_toml.push_str("\nnalgebra = \"0.32.2\"\nsdl2-sys = \"0.35.2\"\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"\nserde_derive = \"1.0.163\"\nrand = \"0.8.5\"\n[dependencies.sdl2]\nversion = \"0.35\"\ndefault-features = false\nfeatures = [\"image\", \"ttf\", \"mixer\"]\n");
    std::fs::write(cargo_toml_path, cargo_toml)?;
    Ok(())
}

fn create_web_toml(web_toml_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let web_toml_content = r#"
default-target = "wasm32-unknown-emscripten"

[target.emscripten]
link-args = [
    "-s", "WASM=1",
    "-s", "USE_SDL=2",
]
"#;
    std::fs::write(web_toml_path, web_toml_content)?;
    Ok(())
}

fn execute_command(command: &str, args: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let status = std::process::Command::new(command)
        .arg(args)
        .status()?;

    if !status.success() {
        return Err(format!("Failed to execute {:?}", args).into());
    }
    Ok(())
}

fn copy_directory(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !src.is_dir() {
        return Err("Source is not a directory".into());
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            std::fs::create_dir_all(&dest_path)?;
            copy_directory(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

pub fn save_project() -> Option<(String, String)> {
    let dialog = rfd::FileDialog::new()
        .add_filter("SuperCool Project", &["sc"])
        .set_directory(".")
        .set_title("Save Project")
        .save_file(); // Use save_file() instead of save()

    match dialog {
        Some(result) => {
            let path_str = result.into_os_string().into_string().unwrap();
            let sc_path_str = path_str.clone() + ".sc";
            Some((path_str, sc_path_str))
        },
        None => {
            eprintln!("Failed to show save dialog");
            None
        }
    }
}

pub fn save_project_to_path<P: AsRef<Path>>(path: P, state: &State) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    let json = serde_json::to_string_pretty(state)?;
    write!(file, "{}", json)
}

pub fn open_state() -> std::io::Result<Option<State>> {
    let dialog = rfd::FileDialog::new()
        .add_filter("SuperCool Project", &["sc"])
        .set_directory(".")
        .set_title("Open State")
        .pick_file();

    match dialog {
        Some(path) => {
            let file = std::fs::read_to_string(path)?;
            match serde_json::from_str::<State>(&file) {
                Ok(state) => Ok(Some(state)),
                Err(e) => {
                    eprintln!("Failed to parse state file: {}", e);
                    Ok(None)
                },
            }
        },
        None => {
            eprintln!("Failed to show open dialog");
            Ok(None)
        }
    }
}

pub fn launcher() {
    /* initialize SDL and its video subsystem */
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    /* hint SDL to initialize an OpenGL 3.3 core profile context */
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_framebuffer_srgb_compatible(true);

    /* create a new window, be sure to call opengl method on the builder when using glow! */
    let window = video_subsystem
        .window("goku engine", 1280, 720)
        //.allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    /* create a new OpenGL context and make it current */
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    unsafe {
        sdl2::sys::SDL_GL_SetAttribute(sdl2_sys::SDL_GLattr::SDL_GL_DOUBLEBUFFER, 1);
        sdl2::sys::SDL_GL_SetAttribute(sdl2_sys::SDL_GLattr::SDL_GL_DEPTH_SIZE, 24);
        sdl2::sys::SDL_GL_SetAttribute(sdl2_sys::SDL_GLattr::SDL_GL_STENCIL_SIZE, 8);
        sdl2::sys::SDL_GL_SetAttribute(sdl2_sys::SDL_GLattr::SDL_GL_SHARE_WITH_CURRENT_CONTEXT, 1);
    }

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    /* enable vsync to cap framerate */
    canvas.window().subsystem().gl_set_swap_interval(1).unwrap();

    /* create new glow and imgui contexts */
    let gl = glow_context(&canvas.window());

    unsafe {
        gl.enable(glow::DEPTH_TEST);
        gl.depth_func(glow::LEQUAL);
    }

    unsafe {
        gl.polygon_offset(1.0, 1.0); // Set the factors to your needs
        gl.enable(glow::POLYGON_OFFSET_FILL);
    }
    
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
    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();

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

    'main: loop {
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
                    break 'main;
                }
                _ => {}
            }
            if state.exit_requested {
                break 'main;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        platform.prepare_frame(&mut imgui, &canvas.window(), &event_pump);

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
                let mut component_types = ["Scene", "Texture", "GameObject", "Ambient Filter", "Layer", "Audio Player", "Particle System", "Light", "Physics", "AI"];
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
        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();
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
use crate::two_d::{{Window, TextureManagerAnim, GameObject, Camera, InputHandler, AmbientFilter}};

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