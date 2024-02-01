use serde::{Deserialize, Serialize};
use copypasta::ClipboardContext;
use crate::gui::component::Component;
use crate::gui::general_settings::GeneralSettings;
use crate::gui::texture_component::TextureComponent;
use crate::gui::terminal::Terminal;
use crate::gui::ambient_filter_component::AmbientFilterComponent;
use crate::gui::audio_player_component::AudioPlayerComponent;
use crate::gui::light_type::LightType;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub selected_component: Option<String>,
    pub components: Vec<Component>,
    #[serde(skip)]
    pub open_about: bool,
    pub open_text_editor: bool,
    pub canvas_present: bool,
    pub text_editor_content: String,
    pub textures: Vec<TextureComponent>,
    pub selected_texture_path: std::path::PathBuf,
    #[serde(skip)]
    pub open_preferences: bool,
    pub general_settings: GeneralSettings,
    pub gameobject_position: Option<(f32, f32)>,
    pub texture_path: Option<String>,
    #[serde(skip)]
    pub terminal: Terminal,
    #[serde(skip)]
    pub undo_stack: Vec<String>,
    #[serde(skip)]
    pub redo_stack: Vec<String>,
    #[serde(skip)]
    pub search_query: String,
    #[serde(skip)]
    pub search_result_index: usize,
    #[serde(skip)]
    pub search_results: Vec<usize>,
    #[serde(skip)]
    pub clipboard: Option<ClipboardContext>,
    #[serde(skip)]
    pub exit_requested: bool,
    #[serde(skip)]
    pub window_name: String,
    #[serde(skip)]
    pub translations: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    #[serde(skip)]
    pub show_save_dialog: bool,
    #[serde(skip)]
    pub show_save_dialog_file: bool,
    #[serde(skip)]
    pub project_dir: std::path::PathBuf,
    #[serde(skip)]
    pub ambient_filters: Vec<AmbientFilterComponent>,
    #[serde(skip)]
    pub audio_player: Option<AudioPlayerComponent>,
    #[serde(skip)]
    pub window_width: i32,
    #[serde(skip)]
    pub window_height: i32,
    #[serde(skip)]
    pub light_type: LightType,
    #[serde(skip)]
    pub light_color: [f32; 3],
    #[serde(skip)]
    pub light_png_path: String,
    #[serde(skip)]
    pub open_image_view: bool,
    #[serde(skip)]
    pub dynamic_texture_id: Option<u32>,
}

impl State {
    pub fn new() -> Self {
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
            light_type: LightType::None,
            light_color: [0.0, 0.0, 0.0],
            light_png_path: "".to_string(),
            open_image_view: false,
            dynamic_texture_id: None,
        };

        if let Err(e) = state.load_settings() {
            eprintln!("Error loading settings: {}", e);
            // Handle the error or provide default settings
        }
        
        state
    }

    pub fn save_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_json = serde_json::to_string(&self.general_settings)?;
        std::fs::write("settings.json", settings_json)?;
        Ok(())
    }

    pub fn load_settings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_json = std::fs::read_to_string("settings.json")?;
        self.general_settings = serde_json::from_str(&settings_json)?;
        Ok(())
    }

    pub fn load_translations(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        self.translations = serde_json::from_reader(file)?;
        Ok(())
    }

    pub fn translate(&self, text: &str) -> String {
        if let Some(language_map) = self.translations.get(&self.general_settings.language) {
            if let Some(translated) = language_map.get(text) {
                return translated.clone();
            }
        }
        text.to_string()
    }

    pub fn search_for_next(&mut self) {
        if !self.search_query.is_empty() {
            // Collect all indices of the search results
            self.search_results = self.text_editor_content.match_indices(&self.search_query).map(|(idx, _)| idx).collect();

            if !self.search_results.is_empty() {
                self.search_result_index = (self.search_result_index + 1) % self.search_results.len();
                // You can use this index to highlight or point to the occurrence in the UI.
            }
        }
    }

    pub fn search_for_previous(&mut self) {
        if !self.search_query.is_empty() && !self.search_results.is_empty() {
            if self.search_result_index == 0 {
                self.search_result_index = self.search_results.len() - 1;
            } else {
                self.search_result_index -= 1;
            }
            // Use this index to highlight or point to the occurrence in the UI.
        }
    }

    pub fn get_line_containing_index(&self, index: usize) -> Option<String> {
        let start = self.text_editor_content[..index].rfind('\n').map_or(0, |i| i + 1); // find previous newline or start of text
        
        let end_offset = self.text_editor_content[index..].find('\n').unwrap_or_else(|| self.text_editor_content[index..].len()); // find next newline or end of text in the slice
        let end = index + end_offset;

        Some(self.text_editor_content[start..end].to_string())
    }
}