use crate::two_d;
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
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone)]
pub struct Component {
    name: String,
    children: Vec<Component>,
}

#[derive(Serialize, Deserialize)]
struct GeneralSettings {
    enable_fullscreen: bool,
    enable_vsync: bool,
    volume: f32,
    language: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextureComponent {
    pub path: std::path::PathBuf,
}

impl Default for TextureComponent {
    fn default() -> Self {
        Self {
            path: std::path::PathBuf::new(),
        }
    }
}

#[derive(Default)]
struct Terminal {
    content: Vec<String>,
    max_lines: usize,
}

impl Terminal {
    fn new(max_lines: usize) -> Self {
        Self {
            content: Vec::new(),
            max_lines,
        }
    }

    // Use a generic function to accept format arguments
    fn log<T: Display>(&mut self, message: T) {
        if self.content.len() >= self.max_lines {
            // Optional: implement logic to remove oldest lines
            // self.content.remove(0);
        }
        // Format the message and push it to the content
        self.content.push(format!("{}", message));
    }

    fn display(&self, ui: &imgui::Ui) {
        for line in &self.content {
            ui.text(line);
        }
    }
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
    tile: Option<two_d::Tile<'static>>,
    #[serde(skip)]
    tile_open: bool,
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
}

impl State {
    fn new() -> Self {
        Self {
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
                volume: 50.0,
                language: "English".to_string(),
            },
            gameobject_position: None,
            texture_path: None,
            terminal: Terminal::new(50),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            tile: None,
            tile_open: false,
            search_query: "".to_string(),
            search_result_index: 0,
            search_results: Vec::new(),
            clipboard: ClipboardContext::new().ok(),
            exit_requested: false,
            window_name: "".to_string(),
        }
    }

    // not optimized, not efficient (need JSON file or something like that)
    fn translate(&self, text: &str) -> String {
        if self.general_settings.language == "Français" {
            match text {
                "File" => "Fichier".to_string(),
                "Open" => "Ouvrir".to_string(),
                "Save" => "Sauvegarder".to_string(),
                "Save As..." => "Enregistrer sous...".to_string(),
                "Preferences" => "Préférences".to_string(),
                "Exit" => "Quitter".to_string(),
                "Edit" => "Modifier".to_string(),
                "Undo" => "Annuler".to_string(),
                "Redo" => "Rétablir".to_string(),
                "Cut" => "Couper".to_string(),
                "Copy" => "Copier".to_string(),
                "Paste" => "Coller".to_string(),
                "View" => "Affichage".to_string(),
                "Search" => "Rechercher".to_string(),
                "Text Editor" => "Éditeur de texte".to_string(),
                "Terminal" => "Terminal".to_string(),
                "Tools" => "Outils".to_string(),
                "Build" => "Construire".to_string(),
                "Run" => "Exécuter".to_string(),
                "Help" => "Aide".to_string(),
                "Documentation" => "Documentation".to_string(),
                "About" => "À propos".to_string(),
                "General settings..." => "Paramètres généraux...".to_string(),
                _ => text.to_string(), // Default to English if no translation is found
            }
        }
        else if self.general_settings.language == "Deutsch" {
            match text {
                "File" => "Datei".to_string(),
                "Open" => "Öffnen".to_string(),
                "Save" => "Speichern".to_string(),
                "Save As..." => "Speichern unter...".to_string(),
                "Preferences" => "Einstellungen".to_string(),
                "Exit" => "Beenden".to_string(),
                "Edit" => "Bearbeiten".to_string(),
                "Undo" => "Rückgängig".to_string(),
                "Redo" => "Wiederholen".to_string(),
                "Cut" => "Ausschneiden".to_string(),
                "Copy" => "Kopieren".to_string(),
                "Paste" => "Einfügen".to_string(),
                "View" => "Ansicht".to_string(),
                "Search" => "Suchen".to_string(),
                "Text Editor" => "Texteditor".to_string(),
                "Terminal" => "Terminal".to_string(),
                "Tools" => "Werkzeuge".to_string(),
                "Build" => "Erstellen".to_string(),
                "Run" => "Ausführen".to_string(),
                "Help" => "Hilfe".to_string(),
                "Documentation" => "Dokumentation".to_string(),
                "About" => "Über".to_string(),
                "General settings..." => "Allgemeine Einstellungen...".to_string(),
               _ => text.to_string(), 
            }
        } else if self.general_settings.language == "Español" {
            match text {
                "File" => "Archivo".to_string(),
                "Open" => "Abrir".to_string(),
                "Save" => "Guardar".to_string(),
                "Save As..." => "Guardar como...".to_string(),
                "Preferences" => "Preferencias".to_string(),
                "Exit" => "Salir".to_string(),
                "Edit" => "Editar".to_string(),
                "Undo" => "Deshacer".to_string(),
                "Redo" => "Rehacer".to_string(),
                "Cut" => "Cortar".to_string(),
                "Copy" => "Copiar".to_string(),
                "Paste" => "Pegar".to_string(),
                "View" => "Ver".to_string(),
                "Search" => "Buscar".to_string(),
                "Text Editor" => "Editor de texto".to_string(),
                "Terminal" => "Terminal".to_string(),
                "Tools" => "Herramientas".to_string(),
                "Build" => "Compilar".to_string(),
                "Run" => "Ejecutar".to_string(),
                "Help" => "Ayuda".to_string(),
                "Documentation" => "Documentación".to_string(),
                "About" => "Acerca de".to_string(),
                "General settings..." => "Configuración general...".to_string(),
                _ => text.to_string(),
            }
        } else if self.general_settings.language == "日本語" {
            match text {
                "File" => "ファイル".to_string(),
                "Open" => "開く".to_string(),
                "Save" => "保存".to_string(),
                "Save As..." => "名前を付けて保存...".to_string(),
                "Preferences" => "設定".to_string(),
                "Exit" => "終了".to_string(),
                "Edit" => "編集".to_string(),
                "Undo" => "元に戻す".to_string(),
                "Redo" => "やり直し".to_string(),
                "Cut" => "切り取り".to_string(),
                "Copy" => "コピー".to_string(),
                "Paste" => "貼り付け".to_string(),
                "View" => "表示".to_string(),
                "Search" => "検索".to_string(),
                "Text Editor" => "テキストエディタ".to_string(),
                "Terminal" => "ターミナル".to_string(),
                "Tools" => "ツール".to_string(),
                "Build" => "ビルド".to_string(),
                "Run" => "実行".to_string(),
                "Help" => "ヘルプ".to_string(),
                "Documentation" => "ドキュメント".to_string(),
                "About" => "このプログラムについて".to_string(),
                "General settings..." => "一般設定...".to_string(),
                _ => text.to_string(),
            }
        } else {
            text.to_string()
        }
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
            if imgui::CollapsingHeader::new(&imgui::ImString::new(format!("{:indent$}{}", "", component.name, indent = level * 2))).build(self.ui) {
                if self.ui.selectable(&format!("{:indent$}{}", "", component.name, indent = (level+1) * 2)) {
                    self.state.selected_component = Some(component.name.clone());
                }
                self.display(&component.children, level + 1);
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

pub fn execute_code(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = format!("{}/temp_cargo_project", std::env::temp_dir().to_string_lossy());
    
    // Create a new cargo project if it doesn't exist
    if !std::path::Path::new(&temp_dir).exists() {
        let output = std::process::Command::new("cargo")
            .arg("new")
            .arg("--bin")
            .arg(&temp_dir)
            .output()?;

        if !output.status.success() {
            return Err("Failed to create a new cargo project".into());
        }
    }

    // Write the provided code to the main.rs file in the new cargo project
    std::fs::write(format!("{}/src/main.rs", &temp_dir), code)?;
    
    // Create the two_d directory in the temporary project
    std::fs::copy("src/two_d.rs", format!("{}/src/two_d.rs", &temp_dir))?;
    let two_d_dir = format!("{}/src/two_d", &temp_dir);
    std::fs::create_dir_all(&two_d_dir)?;
    // Copy all files from the original two_d directory to the temporary project's two_d directory
    let original_two_d_path = std::env::current_dir()?.join("src").join("two_d");
    for entry in std::fs::read_dir(original_two_d_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().ok_or("Failed to get file name")?.to_owned();
            std::fs::copy(&path, format!("{}/{}", two_d_dir, filename.to_string_lossy()))?;
        }
    }

    // Append the required dependencies to Cargo.toml
    let mut cargo_toml = std::fs::read_to_string(format!("{}/Cargo.toml", &temp_dir))?;
    cargo_toml.push_str("\nnalgebra = \"0.32.2\"\nsdl2-sys = \"0.35.2\"\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"\nserde_derive = \"1.0.163\"\nrand = \"0.8.5\"\n");
    cargo_toml.push_str("[dependencies.sdl2]\nversion = \"0.35\"\ndefault-features = false\nfeatures = [\"image\", \"ttf\", \"mixer\"]\n"); 
    std::fs::write(format!("{}/Cargo.toml", &temp_dir), cargo_toml)?;

    // Build the new cargo project
    let output = std::process::Command::new("cargo")
        .arg("build")
        .current_dir(&temp_dir)
        .output()?;

    if !output.status.success() {
        return Err("Failed to compile the code".into());
    }

    // Run the new cargo project
    let output = std::process::Command::new("cargo")
        .arg("run")
        .current_dir(&temp_dir)
        .output()?;

    if !output.status.success() {
        return Err("Failed to execute the code".into());
    }

    // If everything was successful, print the output
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));

    // Cleanup the temporary project directory after completion
    std::fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

pub fn execute_code_web(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("temp_cargo_project");

    // Ensure temp_dir exists
    if !temp_dir.exists() {
        // Create a new cargo project in the temp_dir
        let output = std::process::Command::new("cargo")
        .arg("new")
        .arg("--bin")
        .arg(&temp_dir)
        .output()?;

        if !output.status.success() {
            let err_msg = format!("Failed to create a new cargo project. Output: {}\nError: {}", 
                                    String::from_utf8_lossy(&output.stdout), 
                                    String::from_utf8_lossy(&output.stderr));
            return Err(err_msg.into());
        }
    }

    // 1) Copy the Emscripten SDK directory to temp_dir
    let source_path = std::path::PathBuf::from("emsdk/");
    let mut destination_path = temp_dir.clone();
    destination_path.push("emsdk");

    // Debug printing
    println!("Source Path: {:?}", source_path);
    println!("Destination Path: {:?}", destination_path);

    // Validate paths
    if !source_path.exists() {
        return Err(format!("Source path '{:?}' does not exist.", source_path).into());
    }

    let output = std::process::Command::new("xcopy")
        .arg(format!("{}\\", source_path.display())) // Using display to convert PathBuf to string
        .arg(format!("{}\\", destination_path.display()))
        .arg("/E")  // To copy directories and subdirectories, including empty ones
        .output()?;

    if !output.status.success() {
        let err_msg = format!("Failed to copy the Emscripten SDK directory. Error: {}",
                            String::from_utf8_lossy(&output.stderr));
        return Err(err_msg.into());
    } else {
        println!("Success");
    }

    // 2) and 3) Install Emscripten in the emsdk directory within temp_dir
    let emsdk_env_cmd_path = destination_path.join("emsdk_env.bat");
    if !emsdk_env_cmd_path.exists() {
        return Err(format!("File does not exist: {:?}", emsdk_env_cmd_path).into());
    }
    let output = std::process::Command::new("cmd")
        .arg("/C")
        .arg(&emsdk_env_cmd_path)
        .output()?;
    if !output.status.success() {
        return Err("Failed to execute emsdk_env.bat".into());
    }

    let emsdk_cmd_path = destination_path.join("emsdk");
    if !emsdk_cmd_path.exists() {
        return Err(format!("File does not exist: {:?}", emsdk_cmd_path).into());
    }
    let output = std::process::Command::new("cmd")
        .arg("/C")
        .arg(&emsdk_cmd_path)
        .arg("activate")
        .arg("latest")
        .output()?;
    if !output.status.success() {
        return Err("Failed to run Emscripten".into());
    }

    let main_rs_path = format!("{}/src/main.rs", &temp_dir.to_string_lossy());
    println!("Checking if path exists: {}", &main_rs_path);
    if !std::path::Path::new(&main_rs_path).exists() {
        println!("Path {} does not exist.", &main_rs_path);
    }

    let two_d_path = "/src/two_d.rs";
    println!("Checking if path exists: {}", &two_d_path);
    if !std::path::Path::new(two_d_path).exists() {
        println!("Path {} does not exist.", &two_d_path);
    }

    // Write the provided code to the main.rs file in the new cargo project
    match std::fs::write(&main_rs_path, code) {
        Ok(_) => println!("Successfully wrote to {}", &main_rs_path),
        Err(e) => println!("Failed to write to {}. Error: {:?}", &main_rs_path, e),
    }
    
    let destination_two_d_path = format!("{}/src/two_d.rs", &temp_dir.to_string_lossy());
    // Copy the global classes to the new cargo project
    match std::fs::copy(two_d_path, &destination_two_d_path) {
        Ok(_) => println!("Successfully copied to {}", &destination_two_d_path),
        Err(e) => println!("Failed to copy to {}. Error: {:?}", &destination_two_d_path, e),
    }

    // Append the required dependencies to Cargo.toml
    let mut cargo_toml = std::fs::read_to_string(format!("{}/Cargo.toml", &temp_dir.to_string_lossy()))?;
    cargo_toml.push_str("\nnalgebra = \"0.32.2\"\nsdl2-sys = \"0.35.2\"\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"\nserde_derive = \"1.0.163\"\nrand = \"0.8.5\"\n");
    cargo_toml.push_str("[dependencies.sdl2]\nversion = \"0.35\"\ndefault-features = false\nfeatures = [\"image\", \"ttf\", \"mixer\"]\n"); 
    std::fs::write(format!("{}/Cargo.toml", &temp_dir.to_string_lossy()), cargo_toml)?;

    // Create Web.toml content
    let web_toml_content = r#"
    default-target = "wasm32-unknown-emscripten"

    [target.emscripten]
    link-args = [
        "-s", "WASM=1",
        "-s", "USE_SDL=2",
    ]
    "#;

    // Write the content to Web.toml in temp_dir
    let web_toml_path = temp_dir.join("Web.toml");
    std::fs::write(&web_toml_path, web_toml_content)?;

    // If everything was successful, print the output
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));

    // Cleanup the temporary project directory after completion
    std::fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

pub fn save_project() -> Option<String> {
    let dialog = rfd::FileDialog::new()
        .add_filter("SuperCool Project", &["sc"])
        .set_directory(".")
        .set_title("Save Project")
        .save_file(); // Use save_file() instead of save()

    match dialog {
        Some(result) => Some(result.into_os_string().into_string().unwrap()),
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

fn display_tile_editor(ui: &imgui::Ui, state: &mut State) {
    ui.window("Tile Editor")
        .size([300.0, 600.0], imgui::Condition::FirstUseEver)
        .position([650.0, 10.0], imgui::Condition::FirstUseEver)
        .opened(&mut state.tile_open)
        .build(|| {
            if let Some(tile) = &mut state.tile {
                // Edit tile_map:
                for (row_index, row) in tile.tile_map.iter_mut().enumerate() {
                    for (col_index, tile_type) in row.iter_mut().enumerate() {
                        let old_tile_type_i32 = *tile_type as i32;
                        let mut tile_type_i32 = old_tile_type_i32;
                        let _ = ui.input_int(&format!("Tile[{},{}]", row_index, col_index), &mut tile_type_i32);
                        if old_tile_type_i32 != tile_type_i32 {
                            *tile_type = tile_type_i32.max(0) as u32;  // Ensure non-negative
                        }
                    }
                }

                // Edit colliders:
                for (index, collider) in tile.colliders.iter_mut().enumerate() {
                    let mut x = collider.x();
                    let mut y = collider.y();
                    let mut w = collider.width() as i32;
                    let mut h = collider.height() as i32;

                    let _ = ui.input_int(&format!("Collider[{}] X", index), &mut x);
                    let _ = ui.input_int(&format!("Collider[{}] Y", index), &mut y);
                    let _ = ui.input_int(&format!("Collider[{}] Width", index), &mut w);
                    let _ = ui.input_int(&format!("Collider[{}] Height", index), &mut h);

                    *collider = sdl2::rect::Rect::new(x, y, w.max(0) as u32, h.max(0) as u32);  // Ensure non-negative dimensions
                }

                // For textures:
                for (index, _texture) in tile.textures.iter_mut().enumerate() {
                    // You'll need to adjust how you display and edit the texture.
                    // This is just a placeholder to get you started.
                    let mut texture_name = String::new(); // This is a placeholder. You should initialize with the current texture's name or id.
                    let _ = ui.input_text(&format!("Texture[{}]", index), &mut texture_name);
                    // If texture_name has changed, load/update the texture or whatever logic you have in place.
                }
            }
        });
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

    /* setup platform and renderer, and fonts to imgui */
    let fonts = imgui.fonts();
    let font_data = include_bytes!("ARIALUNI.TTF");
    let font_size = 18.0; // You can adjust the size as needed
    fonts.add_font(&[imgui::FontSource::TtfData {
        data: font_data,
        size_pixels: font_size,
        config: Some(imgui::FontConfig {
            size_pixels: 18.0,
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

    let mut state = State::new();

    /* start main loop */
    let mut event_pump = sdl.event_pump().unwrap();

    let mut is_light_theme = false; // Boolean to track the current theme (You can manage this in your state if required)

    let about_info = AboutInfo {
        version: String::from("0.1.0"),
        date: String::from("03-25-2023"),
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
                    if let Some(path) = save_project() {
                        if save_project_to_path(&path, &state).is_ok() {
                            println!("Project saved to {:?}", path);
                            state.terminal.log("Project saved");
                        } else {
                            eprintln!("Failed to save project to {:?}", path);
                            state.terminal.log(format!("Failed to save project to {:?}", path));
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
                if ui.menu_item(state.translate("Console")) {
                    println!("Console");
                    state.terminal.log("Console");
                }
                if ui.menu_item("Tile Editor") {
                    println!("Tile Editor");
                    state.tile_open = true;
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
                                state.terminal.log(format!("Execution failed: {}", e));
                            },
                        }
                    }
                    // if ui.menu_item("Windows") {
                    // }
                    // if ui.menu_item("Mac OS") {
                    // }
                    // if ui.menu_item("Linux") {
                    // }
                    // if ui.menu_item("Android") {
                    // }
                });
                if ui.menu_item(state.translate("Run")) {
                    match execute_code(&state.text_editor_content) {
                        Ok(_) => {
                            println!("Execution successful!");
                            state.terminal.log("Execution successful!");
                        },
                        Err(e) => {
                            println!("Execution failed: {}", e);
                            state.terminal.log(format!("Execution failed: {}", e));
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
        .size([300.0, window_height as f32 - 200.0], imgui::Condition::Always)
        .position(control_panel_position, imgui::Condition::Always)
        .build(|| {
            match &state.selected_component {
                Some(component) if component == "Scene" => {
                    ui.input_text("Window Name", &mut state.window_name).build();
                    if ui.button("OK") {
                        println!("Window name set to: {}", state.window_name);
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
                        generate_template(&mut state);
                    }
                },
                Some(component) if component == "Texture" => {
                    if ui.button("Add Texture...") {
                        let file = FileDialog::new()
                            .add_filter("PNG Image", &["png"])
                            .pick_file();

                        if let Some(file_path) = file {
                            state.textures.push(TextureComponent { path: file_path });
                        } else {
                            println!("No file chosen");
                            state.terminal.log("No file chosen");
                        }
                    }
                    
                    for (idx, texture) in state.textures.iter().enumerate() {
                        ui.text(format!("Texture {} path: {:?}", idx + 1, texture.path));
                        let p = texture.path.clone();
                        if ui.button(&format!("Load Texture {}", idx + 1)) {
                            let tex = texture_creator.load_texture(&p).unwrap();
                            textures.push(tex);
                            texture_pos.push((301.676, 22.346)); // Add a new position for the new texture
                        }
                        state.texture_path = Some(texture.path.to_str().unwrap().to_string());
                    }                    
                },                               
                Some(component) => ui.text(component),
                None => ui.text("No component selected"),
            }
        });       

        let (_, window_height) = canvas.window().size();
        ui.window("Components")
        .flags(imgui::WindowFlags::NO_COLLAPSE | imgui::WindowFlags::NO_RESIZE)
        .size([300.0, window_height as f32 - 200.0], imgui::Condition::Always)
        .position([0.0, 24.0], imgui::Condition::Always)
        .build(|| {
            ui.text("Add component:");
            if ui.small_button("+") {
                ui.open_popup("Add Component");
            }

            ui.popup("Add Component", || {
                let component_types = ["Scene", "Texture", "GameObject"];

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
          .size([window_width as f32, window_height as f32], imgui::Condition::Always)
          .position(terminal_position, imgui::Condition::Always)
          .build(|| {
              if let Some(tab_bar) = ui.tab_bar("##tabbar") {
                  if ui.tab_item("Log").is_some() {
                      // Terminal UI code
                      state.terminal.display(&ui);
                      // Add more logic as needed for the terminal content
                  }
                  if ui.tab_item("Problems").is_some() {
                    // Terminal UI code
                    // Add more logic as needed for the terminal content
                }
                  tab_bar.end();
              }
            });

        if state.tile_open {
            display_tile_editor(&ui, &mut state);
        }

        if state.open_preferences {
            ui.window(state.translate("Preferences"))
                .size([400.0, 300.0], imgui::Condition::FirstUseEver)
                .opened(&mut state.open_preferences) // Allows you to close the window
                .build(|| {
                    ui.text("Preferences Categories:");
                    ui.separator();
                    ui.menu("General", || {
                        ui.text("General settings...");
                        ui.separator();
                        // You can add sliders, checkboxes, and other controls for various general settings
                        ui.checkbox("Enable Fullscreen", &mut state.general_settings.enable_fullscreen);
                        ui.checkbox("Enable canvas present", &mut state.canvas_present);
                        ui.checkbox("Enable VSync", &mut state.general_settings.enable_vsync);
                        ui.slider("Volume", 0.0, 100.0, &mut state.general_settings.volume);
                        // Add controls for general settings here
                    });
                    ui.menu("Appearance", || {
                        ui.text("Appearance settings...");
                        // Add controls for appearance settings here
                    });
                    ui.menu("Input", || {
                        ui.text("Input settings...");
                        // Add controls for input settings here
                    });
                    let languages = ["English", "Deutsch", "Español", "Français", "日本語"];
                    
                    let mut current_item = languages
                        .iter()
                        .position(|language| language == &state.general_settings.language)
                        .unwrap_or(0);

                    if ui.combo(
                        "Language",
                        &mut current_item,
                        &languages,
                        |language| language.to_string().into(),
                    ) {
                        state.general_settings.language = languages[current_item].to_string();
                    }
                    // Add more categories as needed
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

fn generate_template(state: &mut State) {
    let window_title = &state.window_name;
    let mut content = format!(r#"
mod two_d;
use nalgebra::Vector2;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let mut window = Window::new("{}", 800, 600)?;

    let last_frame_time = unsafe {{ sdl2::sys::SDL_GetTicks() }};
    let mut current_frame_time;
    let mut delta_time;

    let texture_creator = window.canvas.texture_creator();
"#, window_title);

    // Create a counter for game objects and their textures
    let mut game_object_counter = 1;

    for component in &state.components {
        if component.name == "GameObject" {
            let game_object_var_name = format!("game_object{}", game_object_counter);
            content.push_str(&format!(r#"
    let texture_manager{} = TextureManagerAnim::new(&texture_creator);

    let mut {} = GameObject::new(texture_manager{}, Vector2::new(50, 50));
{}."#, game_object_counter, game_object_var_name, game_object_counter, game_object_var_name));

            for texture in &state.textures {
                content.push_str(&format!(r#"
    {}.load_texture(Path::new("{}"), 30, 30, 150)?;
"#, game_object_var_name, texture.path.display()));
            }
            game_object_counter += 1;
        }
    }
    content.push_str(&format!(r#"
Ok(())
}}"#));
    state.text_editor_content = content;
}