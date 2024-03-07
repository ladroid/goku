use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextureComponent {
    pub path: std::path::PathBuf,
    pub tag_name: String,
    pub width: u32,
    pub height: u32,
    pub frames: u32,
}

impl Default for TextureComponent {
    fn default() -> Self {
        Self {
            path: std::path::PathBuf::new(),
            tag_name: String::new(),
            width: 0,
            height: 0,
            frames: 0,
        }
    }
}