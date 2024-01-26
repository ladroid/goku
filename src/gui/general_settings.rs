use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GeneralSettings {
    pub enable_fullscreen: bool,
    pub enable_vsync: bool,
    pub language: String,
    pub enable_input_handler: bool,
    pub font_name: String,
    pub font_size: f32,
    pub font_change_requested: bool, // Add this
}