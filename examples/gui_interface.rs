// Cargo.toml
#epi = "0.17.0"
#egui_sdl2_gl = "0.16.0"
#egui_backend = "0.2.0"
#egui = "0.21.0"
#eframe = { version = "0.21.0", default-features = false, features = [
#    "accesskit",     # Make egui comptaible with screen readers. NOTE: adds a lot of dependencies.
#    "default_fonts", # Embed the default egui fonts.
#    "glow",          # Use the glow rendering backend. Alternative: "wgpu".
#    "persistence",   # Enable restoring app state when restarting the app.
#] }
#egui_sdl2_platform = "0.2.0"
#egui_glow = "0.21.0"


// use crate::two_d;

use std::fs::File;
use std::io::Write;
use std::collections::VecDeque;
use std::path::Path;

use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;
// Game Engine UI
    // egui
        // properties of components
        // game viewport (?)
        // scripting

lazy_static! {
    static ref TTF_CONTEXT: std::sync::Arc<sdl2::ttf::Sdl2TtfContext> = std::sync::Arc::new(sdl2::ttf::init().expect("Failed to initialize the TTF context"));
}

// macro log
macro_rules! logln {
    ($app:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        $app.log_messages.push(message);
    }};
}

// Define your UI components here
#[derive(Serialize, Deserialize, Default)]
pub struct Project {
    pub components: Vec<Component>,
    pub text: String, // Add this line
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Component {
    Scene { code: std::cell::RefCell<String> },
    GameObject,
    Texture,
    TileMap,
    Camera,
    Button,
    Slider,
    UILayer,
}

impl Component {
    pub fn name(&self) -> &str {
        match self {
            Component::Scene { .. } => "Scene",
            Component::GameObject => "GameObject",
            Component::Texture => "Texture",
            Component::TileMap => "TileMap",
            Component::Camera => "Camera",
            Component::Button => "Button",
            Component::Slider => "Slider",
            Component::UILayer => "UILayer",
        }
    }
}

pub struct AboutInfo {
    pub version: String,
    pub date: String,
    pub rust_version: String,
    pub games_api: String,
    pub os: String,
}

struct MyApp {
    pub project: Project,
    pub components: Vec<Component>,
    pub selected_component: Option<usize>,
    pub show_menu: bool,
    // selected_game_object_position: Option<&'a mut Vector2<i32>>,
    pub about_info: Option<AboutInfo>,
    pub show_about: bool,
    pub show_preferences: bool,
    pub text_history: VecDeque<String>,
    pub history_index: usize,
    pub clipboard: Option<String>,
    pub selected_range: Option<std::ops::Range<usize>>,
    pub selected_language: String,
    pub show_documentation_link: bool,
    pub log_messages: Vec<String>,
    pub selected_file: Option<std::path::PathBuf>,
    translations: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            project: Project::default(), /*file_system_root: ".".to_string(),*/ 
            components: Vec::new(), 
            selected_component: None, 
            show_menu: false,
            // selected_game_object_position: None,
            about_info: Some(AboutInfo {
                version: String::from("0.1.0"),
                date: String::from("03-25-2023"),
                rust_version: String::from("1.68"),
                games_api: String::from("SDL2"),
                os: std::env::consts::OS.to_string(),
            }),
            show_about: false,
            show_preferences: false,
            text_history: VecDeque::new(),
            history_index: 0,
            clipboard: None,
            selected_range: None,
            selected_language: "English".to_string(),
            show_documentation_link: false,
            log_messages: Vec::new(),
            selected_file: None,
            translations: std::collections::HashMap::new(),
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        // Self::default()
        if let Some(_saved_state) = cc.storage {
            // return eframe::get_value(_saved_state, eframe::APP_KEY).unwrap_or_default();
        }
        let mut app = Self::default();
        app.init_translations();
        app
        // Default::default()
    }

    fn name() -> String {
        return "goku engine".to_string();
    }

    fn init_translations(&mut self) -> std::collections::HashMap<String, std::collections::HashMap<String, String>> {
        let translations = std::collections::HashMap::new();

        let mut en = std::collections::HashMap::new();
        en.insert("File".to_string(), "File".to_string());
        en.insert("New".to_string(), "New".to_string());
        // ... more English translations ...

        let mut de = std::collections::HashMap::new();
        de.insert("File".to_string(), "Datei".to_string());
        de.insert("New".to_string(), "Neu".to_string());
        de.insert("Open".to_string(), "Öffnen".to_string());
        de.insert("Save".to_string(), "Speichern".to_string());
        de.insert("Preferences".to_string(), "Einstellungen".to_string());
        de.insert("Exit".to_string(), "Beenden".to_string());
        de.insert("Edit".to_string(), "Bearbeiten".to_string());
        de.insert("Undo".to_string(), "Rückgängig".to_string());
        de.insert("Redo".to_string(), "Wiederholen".to_string());
        de.insert("Cut".to_string(), "Ausschneiden".to_string());
        de.insert("Copy".to_string(), "Kopieren".to_string());
        de.insert("Paste".to_string(), "Einfügen".to_string());
        de.insert("Find".to_string(), "Suchen".to_string());
        de.insert("View".to_string(), "Ansicht".to_string());
        de.insert("Appearance".to_string(), "Erscheinungsbild".to_string());
        de.insert("Light theme".to_string(), "Helles Thema".to_string());
        de.insert("Dark theme".to_string(), "Dunkles Thema".to_string());
        de.insert("Search".to_string(), "Suche".to_string());
        de.insert("Components".to_string(), "Komponenten".to_string());
        de.insert("Scene".to_string(), "Szene".to_string());
        de.insert("Inspector".to_string(), "Inspektor".to_string());
        de.insert("Choose .rs file".to_string(), ".rs Datei wählen".to_string());
        de.insert("Create new .rs file".to_string(), "Neue .rs Datei erstellen".to_string());
        de.insert("Tools".to_string(), "Werkzeuge".to_string());
        de.insert("Run".to_string(), "Ausführen".to_string());
        de.insert("Help".to_string(), "Hilfe".to_string());
        de.insert("Build".to_string(), "Bauen".to_string());
        de.insert("About".to_string(), "Über".to_string());
        de.insert("Documentation".to_string(), "Dokumentation".to_string());

        let mut fr = std::collections::HashMap::new();
        fr.insert("File".to_string(), "Fichier".to_string());
        fr.insert("New".to_string(), "Nouveau".to_string());
        fr.insert("Open".to_string(), "Ouvrir".to_string());
        fr.insert("Save".to_string(), "Enregistrer".to_string());
        fr.insert("Preferences".to_string(), "Préférences".to_string());
        fr.insert("Exit".to_string(), "Quitter".to_string());
        fr.insert("Edit".to_string(), "Éditer".to_string());
        fr.insert("Undo".to_string(), "Annuler".to_string());
        fr.insert("Redo".to_string(), "Refaire".to_string());
        fr.insert("Cut".to_string(), "Couper".to_string());
        fr.insert("Copy".to_string(), "Copier".to_string());
        fr.insert("Paste".to_string(), "Coller".to_string());
        fr.insert("Find".to_string(), "Rechercher".to_string());
        fr.insert("View".to_string(), "Affichage".to_string());
        fr.insert("Appearance".to_string(), "Apparence".to_string());
        fr.insert("Light theme".to_string(), "Thème clair".to_string());
        fr.insert("Dark theme".to_string(), "Thème sombre".to_string());
        fr.insert("Search".to_string(), "Recherche".to_string());
        fr.insert("Tools".to_string(), "Outils".to_string());
        fr.insert("Run".to_string(), "Exécuter".to_string());
        fr.insert("Help".to_string(), "Aide".to_string());
        fr.insert("Build".to_string(), "Construire".to_string());
        fr.insert("About".to_string(), "À propos".to_string());
        fr.insert("Documentation".to_string(), "Documentation".to_string());

        let mut es = std::collections::HashMap::new();
        es.insert("File".to_string(), "Archivo".to_string());
        es.insert("New".to_string(), "Nuevo".to_string());
        es.insert("Open".to_string(), "Abrir".to_string());
        es.insert("Save".to_string(), "Guardar".to_string());
        es.insert("Preferences".to_string(), "Preferencias".to_string());
        es.insert("Exit".to_string(), "Salir".to_string());
        es.insert("Edit".to_string(), "Editar".to_string());
        es.insert("Undo".to_string(), "Deshacer".to_string());
        es.insert("Redo".to_string(), "Rehacer".to_string());
        es.insert("Cut".to_string(), "Cortar".to_string());
        es.insert("Copy".to_string(), "Copiar".to_string());
        es.insert("Paste".to_string(), "Pegar".to_string());
        es.insert("Find".to_string(), "Buscar".to_string());
        es.insert("View".to_string(), "Ver".to_string());
        es.insert("Appearance".to_string(), "Apariencia".to_string());
        es.insert("Light theme".to_string(), "Tema claro".to_string());
        es.insert("Dark theme".to_string(), "Tema oscuro".to_string());
        es.insert("Search".to_string(), "Buscar".to_string());
        es.insert("Components".to_string(), "Componentes".to_string());
        es.insert("Scene".to_string(), "Escena".to_string());
        es.insert("Inspector".to_string(), "Inspector".to_string());
        es.insert("Choose .rs file".to_string(), "Elegir archivo .rs".to_string());
        es.insert("Create new .rs file".to_string(), "Crear nuevo archivo .rs".to_string());
        es.insert("Tools".to_string(), "Herramientas".to_string());
        es.insert("Run".to_string(), "Correr".to_string());
        es.insert("Help".to_string(), "Ayuda".to_string());
        es.insert("Build".to_string(), "Construir".to_string());
        es.insert("About".to_string(), "Acerca de".to_string());
        es.insert("Documentation".to_string(), "Documentación".to_string());

        let mut jp = std::collections::HashMap::new();
        jp.insert("File".to_string(), "ファイル".to_string());
        jp.insert("New".to_string(), "新規作成".to_string());
        jp.insert("Open".to_string(), "開く".to_string());
        jp.insert("Save".to_string(), "保存".to_string());
        jp.insert("Preferences".to_string(), "設定".to_string());
        jp.insert("Exit".to_string(), "終了".to_string());
        jp.insert("Edit".to_string(), "編集".to_string());
        jp.insert("Undo".to_string(), "元に戻す".to_string());
        jp.insert("Redo".to_string(), "やり直し".to_string());
        jp.insert("Cut".to_string(), "切り取り".to_string());
        jp.insert("Copy".to_string(), "コピー".to_string());
        jp.insert("Paste".to_string(), "貼り付け".to_string());
        jp.insert("Find".to_string(), "検索".to_string());
        jp.insert("View".to_string(), "表示".to_string());
        jp.insert("Appearance".to_string(), "外観".to_string());
        jp.insert("Light theme".to_string(), "ライトテーマ".to_string());
        jp.insert("Dark theme".to_string(), "ダークテーマ".to_string());
        jp.insert("Search".to_string(), "検索".to_string());
        jp.insert("Components".to_string(), "コンポーネント".to_string());
        jp.insert("Scene".to_string(), "シーン".to_string());
        jp.insert("Inspector".to_string(), "インスペクタ".to_string());
        jp.insert("Choose .rs file".to_string(), ".rsファイルを選択".to_string());
        jp.insert("Create new .rs file".to_string(), "新しい.rsファイルを作成".to_string());
        jp.insert("Tools".to_string(), "ツール".to_string());
        jp.insert("Run".to_string(), "実行".to_string());
        jp.insert("Help".to_string(), "ヘルプ".to_string());
        jp.insert("Build".to_string(), "ビルド".to_string());
        jp.insert("About".to_string(), "情報".to_string());
        jp.insert("Documentation".to_string(), "ドキュメンテーション".to_string());

        self.translations.insert("English".to_string(), en);
        self.translations.insert("Deutsch".to_string(), de);
        self.translations.insert("Español".to_string(), es);
        self.translations.insert("Français".to_string(), fr);
        self.translations.insert("日本語".to_string(), jp);

        translations
    }

    // Translate a given key based on the selected language
    fn translate(translations: &std::collections::HashMap<String, std::collections::HashMap<String, String>>, selected_language: &str, key: &str) -> String {
        if let Some(language_map) = translations.get(selected_language) {
            if let Some(value) = language_map.get(key) {
                return value.clone();
            }
        }
        key.to_string() // Fallback to key if no translation is found
    }       
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            project, 
            components: _, 
            selected_component: _, 
            show_menu: _,
            // selected_game_object_position: _,
            about_info: _,
            show_about: _,
            show_preferences: _,
            text_history: _,
            history_index: _,
            clipboard: _,
            selected_range: _,
            selected_language: _,
            show_documentation_link: _,
            log_messages: _,
            selected_file: _,
            translations: _,
        } = self;

        configure_egui_fonts(ctx);

        // Check if the Delete key is pressed
        if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
            if let Some(selected_index) = self.selected_component {
                // Remove the selected component from the list
                self.components.remove(selected_index);
                println!("Deleted successfully {}", selected_index);
                logln!(self, "Deleted successfully {}", selected_index);
                // Deselect the removed component
                self.selected_component = None;
            }
        }

        // Before the closure, clone the data it needs.
        let file_text = MyApp::translate(&self.translations, &self.selected_language, "File").clone();
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(file_text, |ui| {
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "New")).clicked() {
                        println!("New button clicked");
                        logln!(self, "New button clicked");
                        *project = Project::default();
                    }
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Open")).clicked() {
                        if let Some(opened_project) = open_project() {
                            println!("Opened project successfully");
                            logln!(self, "Opened project successfully");
                            *project = opened_project;
                        } else {
                            eprintln!("Failed to open project");
                            logln!(self, "Failed to open project");
                        }
                    }
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Save")).clicked() {
                        if let Some(path) = save_project() {
                            if save_project_to_path(&path, project).is_ok() {
                                println!("Project saved to {:?}", path);
                                logln!(self, "Project saved to {:?}", path);
                            } else {
                                eprintln!("Failed to save project to {:?}", path);
                                logln!(self, "Failed to save project to {:?}", path);
                            }
                        }
                    }
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Preferences")).clicked() {
                        println!("Preferences button clicked");
                        self.show_preferences = !self.show_preferences; // Toggle the visibility of the Preferences window
                    }
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Exit")).clicked() {
                        println!("Exit button clicked");
                        logln!(self, "Exit button clicked");
                        eframe::Frame::close(_frame);
                    }
                });
            
                ui.menu_button(MyApp::translate(&self.translations, &self.selected_language, "Edit"), |ui| {
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Undo")).clicked() {
                        println!("Undo button clicked");
                        logln!(self, "Undo button clicked");
                        if self.history_index > 0 {
                            self.history_index -= 1;
                            project.text = self.text_history[self.history_index].clone();
                        }
                    }
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Redo")).clicked() {
                        println!("Redo button clicked");
                        logln!(self, "Redo button clicked");
                        if self.history_index + 1 < self.text_history.len() {
                            self.history_index += 1;
                            project.text = self.text_history[self.history_index].clone();
                        }
                    }
                    ui.separator();
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Cut")).clicked() {
                        println!("Cut button clicked");
                        logln!(self, "Cut button clicked");
                        if let Some(range) = self.selected_range.as_ref() {
                            let (selected_text, remaining_text) = cut_selected_text(&project.text, range.clone());
                            self.clipboard = Some(selected_text);
                            project.text = remaining_text;
                        }
                    }
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Copy")).clicked() {
                        if let Some(range) = self.selected_range.as_ref() {
                            self.clipboard = Some(project.text[range.clone()].to_string());
                        }
                    }
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Paste")).clicked() {
                        if let Some(clipboard_content) = self.clipboard.as_ref() {
                            if let Some(range) = self.selected_range.as_ref() {
                                let (before_selection, after_selection) = project.text.split_at(range.start);
                                project.text = format!("{}{}{}", before_selection, clipboard_content, after_selection);
                            }
                        }
                    }
                    ui.separator();
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Find")).clicked() {
                        println!("Find button clicked");
                        logln!(self, "Find button clicked");
                    }
                });
            
                ui.menu_button(MyApp::translate(&self.translations, &self.selected_language, "View"), |ui| {
                    ui.menu_button(MyApp::translate(&self.translations, &self.selected_language, "Appearance"), |ui| {
                        if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Light theme")).clicked() {
                            println!("Light theme button clicked");
                            logln!(self, "Light theme button clicked");
                            ctx.set_visuals(egui::Visuals::light());
                        }
                        if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Dark theme")).clicked() {
                            println!("Dark theme button clicked");
                            logln!(self, "Dark theme button clicked");
                            ctx.set_visuals(egui::Visuals::dark());
                        }
                    });
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Search")).clicked() {
                        println!("Search button clicked");
                        logln!(self, "Search button clicked");
                    }
                });
            
                ui.menu_button(MyApp::translate(&self.translations, &self.selected_language, "Tools"), |ui| {
                    ui.menu_button(MyApp::translate(&self.translations, &self.selected_language, "Build"), |ui| {
                        if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Web")).clicked() {
                            println!("Build button clicked");
                            if let Some(file_path) = &self.selected_file {
                                println!("{}",file_path.display());
                                // Save the changes from the TextEditor back to the .rs file
                                if std::fs::write(&file_path, &project.text).is_ok() {
                                    println!("Saved changes to {:?}", file_path);
                                    logln!(self, "Saved changes to {:?}", file_path);
                                    // Run the .rs file
                                    match execute_code_web(&project.text) {
                                        Ok(()) => {
                                            println!("Code executed successfully");
                                            logln!(self, "Code executed successfully");
                                        },
                                        Err(e) => {
                                            eprintln!("Failed to execute code: {}", e);
                                            logln!(self, "Failed to execute code: {}", e);
                                        },
                                    }
                                } else {
                                    eprintln!("Failed to save changes to {:?}", file_path);
                                    logln!(self, "Failed to save changes to {:?}", file_path);
                                }
                            } else {
                                println!("No .rs file is selected");
                                logln!(self, "No .rs file is selected");
                            }
                        }
                        if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Widnows")).clicked() {
                            println!("Windows build button clicked");
                        }
                        if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Mac OS")).clicked() {
                            println!("Mac build button clicked");
                        }
                        if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Linux")).clicked() {
                            println!("Linux build button clicked");
                        }
                        if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Android")).clicked() {
                            println!("Android build button clicked");
                        }
                    });
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Run")).clicked() {
                        if let Some(file_path) = &self.selected_file {
                            // Save the changes from the TextEditor back to the .rs file
                            if std::fs::write(&file_path, &project.text).is_ok() {
                                println!("Saved changes to {:?}", file_path);
                                logln!(self, "Saved changes to {:?}", file_path);
                                // Run the .rs file
                                match execute_code(&project.text) {
                                    Ok(()) => {
                                        println!("Code executed successfully");
                                        logln!(self, "Code executed successfully");
                                    },
                                    Err(e) => {
                                        eprintln!("Failed to execute code: {}", e);
                                        logln!(self, "Failed to execute code: {}", e);
                                    },
                                }
                            } else {
                                eprintln!("Failed to save changes to {:?}", file_path);
                                logln!(self, "Failed to save changes to {:?}", file_path);
                            }
                        } else {
                            println!("No .rs file is selected");
                            logln!(self, "No .rs file is selected");
                        }
                    }                    
                });
            
                ui.menu_button(MyApp::translate(&self.translations, &self.selected_language, "Help"), |ui| {
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "About")).clicked() {
                        println!("About button clicked");
                        logln!(self, "About button clicked");
                        self.show_about = !self.show_about; // Toggle the visibility of the About window
                    }
                    if ui.button(MyApp::translate(&self.translations, &self.selected_language, "Documentation")).clicked() {
                        println!("Documentation button clicked");
                        logln!(self, "Documentation button clicked");
                        self.show_documentation_link = !self.show_documentation_link;
                    }
                    if self.show_documentation_link {
                        ui.add(egui::Hyperlink::new("https://github.com/ladroid"));
                    }
                });
            });            
        });
        
        if self.show_preferences {
            egui::Window::new("Preferences")
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.heading("General");
                    ui.separator();
                    ui.heading("User interface");
                    ui.label("Appearance");
                    ui.label("Font");
                    ui.separator();
                    ui.heading("Language");
                    // Language selection
                    // Save the current language before the combo box
                    let previous_language = self.selected_language.clone();

                    let languages = ["English", "Deutsch", "Español", "Français", "日本語"];
                    egui::ComboBox::from_id_source("language_selector")
                        .selected_text(self.selected_language.clone())
                        .show_ui(ui, |ui| {
                            for language in languages {
                                ui.selectable_value(&mut self.selected_language, language.to_string(), language);
                            }
                        });

                    // If the language has changed, request a repaint
                    if previous_language != self.selected_language {
                        ctx.request_repaint();
                    }
                    ui.separator();
                    ui.heading("Editor Category");
                    let mut check = false;
                    ui.checkbox(&mut check, "Line numbers");
                    // Add a close button to the "Preferences" window
                    if ui.button("OK").clicked() {
                        self.show_preferences = false;
                    }
                });
        }
        
        if self.show_about {
            if let Some(about_info) = &self.about_info {
                egui::Window::new("About")
                    .default_width(200.0)
                    .show(ctx, |ui| {
                        ui.label(format!("Version: {}", about_info.version));
                        ui.label(format!("Date: {}", about_info.date));
                        ui.label(format!("Rust: {}", about_info.rust_version));
                        ui.label(format!("Games API: {}", about_info.games_api));
                        ui.label(format!("OS: {}", about_info.os));
                        // Add a close button to the "About" window
                        if ui.button("OK").clicked() {
                            self.show_about = false;
                        }
                    });
            }
        }        
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Components");
            let mut menu_pos = egui::pos2(0.0, 0.0);
            if ui.button("+").clicked() {
                // Remember the position of the "+" button
                menu_pos = ui.min_rect().max;
                ctx.request_repaint(); // Request repaint to show the dropdown menu
                self.show_menu = true;// Remember the position of the "+" button
            }

            if self.show_menu {
                // Show a dropdown menu with the list of components to add
                egui::Window::new("Add Component")
                    .id(egui::Id::new("add_component_menu"))
                    .fixed_pos(menu_pos)
                    .resizable(false)
                    .show(ctx, |ui| {
                        if ui.button("GameObject").clicked() {
                            self.components.push(Component::GameObject);
                            self.show_menu = false;
                        }
                        if ui.button("Texture").clicked() {
                            self.components.push(Component::Texture);
                            self.show_menu = false;
                        }
                        if ui.button("TileMap").clicked() {
                            self.components.push(Component::TileMap);
                            self.show_menu = false;
                        }
                        if ui.button("Camera").clicked() {
                            self.components.push(Component::Camera);
                            self.show_menu = false;
                        }
                        if ui.button("Button").clicked() {
                            self.components.push(Component::Button);
                            self.show_menu = false;
                        }
                        if ui.button("Slider").clicked() {
                            self.components.push(Component::Slider);
                            self.show_menu = false;
                        }
                        if ui.button("UILayer").clicked() {
                            self.components.push(Component::UILayer);
                            self.show_menu = false;
                        }
                        if ui.button("Scene").clicked() {
                            // self.components.push(Component::Scene);
                            // self.show_menu = false;
                            let dialog = rfd::FileDialog::new()
                                .add_filter("Rust source file", &["rs"]);
                            if let Some(path) = dialog.pick_file() {
                                // Read the contents of the selected .rs file
                                if let Ok(contents) = std::fs::read_to_string(&path) {
                                    // Assign the contents to the TextEditor
                                    project.text = contents;
                                    self.components.push(Component::Scene { code: path.to_string_lossy().into_owned().into() });
                                    self.show_menu = false;
                                }
                            }
                        }
                    });
            }

            ui.separator();

            // Display the added components under the separator
            for (index, component) in self.components.iter().enumerate() {
                if ui.selectable_label(self.selected_component == Some(index), component.name()).clicked() {
                    self.selected_component = Some(index);
                }
            }
        });
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Inspector");

            // Show the fields of the selected component
            if let Some(index) = self.selected_component {
                let component = &self.components[index];
                match component {
                    Component::GameObject => {
                        // Display GameObject fields
                        ui.label("GameObject properties");
                        ui.separator();
                        
                        // Position input field
                        ui.horizontal( |ui| {
                            ui.label("Position:");
                            let mut scalar_x = 1.0;
                            let mut scalar_y = 1.0;
                            ui.add(egui::DragValue::new(&mut scalar_x));
                            ui.add(egui::DragValue::new(&mut scalar_y));
                            // if let Some(position) = self.selected_game_object_position.as_mut() {
                            //     ui.add(egui::DragValue::new(&mut position.x));
                            //     ui.end_row();
                            //     ui.add(egui::DragValue::new(&mut position.y));
                            //     ui.end_row();
                            // } else {
                            //     ui.label("No GameObject selected");
                            //     ui.end_row();
                            // }
                        });

                        // Collider input fields
                        ui.label("Collider:");
                        ui.horizontal(|ui| {
                            ui.label("Position:");
                            //ui.add(egui::DragValue::new(&mut self.collider.x)).prefix("X:");
                            //ui.add(egui::DragValue::new(&mut self.collider.y)).prefix("Y:");
                        });
                        ui.horizontal(|ui| {
                            ui.label("Size:       ");
                            let mut scalar_x = 1.0;
                            let mut scalar_y = 1.0;
                            ui.add(egui::DragValue::new(&mut scalar_x).speed(1.0));
                            ui.add(egui::DragValue::new(&mut scalar_y).speed(1.0));
                        });

                        // RigidBody input field
                        ui.label("RigidBody:");
                        ui.horizontal(|ui| {
                            ui.label("Mass:");
                            let mut scalar_x = 1.0;
                            ui.add(egui::DragValue::new(&mut scalar_x).speed(1.0));
                        });
                    }
                    Component::Texture => {
                        // Display Texture fields
                        ui.label("Texture properties...");
                    }
                    Component::TileMap => {
                        // Display TileMap fields
                        ui.label("TileMap properties...");
                    }
                    Component::Camera => {
                        // Display Camera fields
                        ui.label("Camera properties...");
                    }
                    Component::Button => {
                        ui.label("Button properties");
                        ui.separator();
                        ui.label("Text:");
                        ui.add(egui::TextEdit::singleline(&mut "".to_string()).hint_text("Write something here"));
                        ui.end_row();
                        ui.label("Font size:");
                        let mut scalar_x = 1.0;
                        ui.add(egui::DragValue::new(&mut scalar_x).speed(1.0));
                        ui.end_row();

                    }
                    Component::Slider => {

                    }
                    Component::UILayer => {

                    }
                    Component::Scene { code } => {
                        // If the user wants to change the associated .rs file
                        if ui.button("Choose .rs file").clicked() {
                            let dialog = rfd::FileDialog::new()
                                .add_filter("Rust source file", &["rs"]);
                            if let Some(path) = dialog.pick_file() {
                                // Clone the path to avoid ownership issues
                                let path_clone = path.clone();
                                // Read the contents of the selected .rs file
                                if let Ok(contents) = std::fs::read_to_string(&path_clone) {
                                    // Assign the contents to the TextEditor
                                    project.text = contents;
                                    self.selected_file = Some(path);
                                    // Update the code of the Scene component
                                    let mut code_mut = code.borrow_mut();
                                    code_mut.clear();
                                    code_mut.push_str(&path_clone.to_str().unwrap().to_string());
                                }
                            }
                        }
                    
                        // If the user wants to create a new .rs file
                        if ui.button("Create new .rs file").clicked() {
                            let dialog = rfd::FileDialog::new()
                                .add_filter("Rust source file", &["rs"]);
                            if let Some(path) = dialog.save_file() {
                                // Clone the path to avoid ownership issues
                                let path_clone = path.clone();
                                // Create a new .rs file
                                let new_file_contents = "// Your Rust code here";
                                if std::fs::write(&path_clone, &new_file_contents).is_ok() {
                                    // Assign the new file contents to the TextEditor
                                    project.text = new_file_contents.to_string();
                                    self.selected_file = Some(path);
                                    // Update the code of the Scene component
                                    let mut code_mut = code.borrow_mut();
                                    code_mut.clear();
                                    code_mut.push_str(&path_clone.to_str().unwrap().to_string());
                                }
                            }
                        }
                    }
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_height(available_size.y);
                
                let previous_text = project.text.clone();
                let text_editor = egui::TextEdit::multiline(&mut project.text)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .lock_focus(true)
                    .desired_width(available_size.x)
                    .desired_rows(20);
                
                ui.add(text_editor);

                if previous_text != project.text {
                    self.text_history.truncate(self.history_index);
                    self.text_history.push_back(previous_text);
                    self.history_index += 1;
                }

                // Add a separator between the TextEdit and the debug console
                ui.separator();
                ui.heading("Debug console:");
                // Display the log messages in a ScrollArea
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    for message in &self.log_messages {
                        ui.label(message);
                    }
                });
            });
        });
    }
}

pub fn configure_egui_fonts(ctx: &egui::Context) {
    let japanese_font_data = include_bytes!("NotoSansJP-VariableFont_wght.ttf");
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("my_font".to_owned(), egui::FontData::from_static(japanese_font_data));
    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    ctx.set_fonts(fonts);
}

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

// Not optimized
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

    let main_rs_path = format!("{}\\src\\main.rs", &temp_dir.to_string_lossy());
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
    
    let destination_two_d_path = format!("{}\\src\\two_d.rs", &temp_dir.to_string_lossy());
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

    // Build the new cargo project
    // let output = std::process::Command::new("cargo")
    //     .arg("web")
    //     .arg("build")
    //     .arg("--target")
    //     .arg("wasm32-unknown-emscripten")
    //     .current_dir(&temp_dir)
    //     .output()?;
    // println!("AAA {:?}", output);
    // if !output.status.success() {
    //     return Err("Failed to compile the code".into());
    // }

    // If everything was successful, print the output
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));

    // Cleanup the temporary project directory after completion
    // std::fs::remove_dir_all(&temp_dir)?;

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

pub fn save_project_to_path<P: AsRef<Path>>(path: P, project: &Project) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let json = serde_json::to_string_pretty(project)?;
    write!(file, "{}", json)
}

pub fn open_project() -> Option<Project> {
    let dialog = rfd::FileDialog::new()
        .add_filter("SuperCool Project", &["sc"])
        .set_directory(".")
        .set_title("Open Project")
        .pick_file();

    match dialog {
        Some(path) => {
            match load_project_from_path(&path) {
                Ok(mut project) => {
                    // Check if the project contains a Scene component
                    for component in &project.components {
                        if let Component::Scene { code } = component {
                            // Borrow the string inside the RefCell
                            let code_str = code.borrow();
                            // Read the contents of the .rs file
                            if let Ok(contents) = std::fs::read_to_string(&*code_str) {
                                project.text = contents;
                                break;
                            }
                        }
                    }
                    Some(project)
                },
                Err(e) => {
                    eprintln!("Failed to open project from {:?}: {}", path, e);
                    None
                }
            }
        }
        None => {
            eprintln!("Failed to show open dialog");
            None
        }
    }
}

pub fn load_project_from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Project> {
    let file = File::open(path)?;
    let project: Project = serde_json::from_reader(file)?;
    Ok(project)
}

pub fn launcher() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(&MyApp::name(), native_options, Box::new(|cc| Box::new(MyApp::new(cc))))?;
    Ok(())
}

fn main() -> Result<(), String> {
    // run()?;
    if let Err(err) = launcher() {
        eprintln!("Error: {}", err);
    };
    Ok(())
}