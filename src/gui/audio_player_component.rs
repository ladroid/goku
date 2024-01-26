use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AudioPlayerComponent {
    pub volume: i32,
    pub track_path: String,
    pub loop_count: i32,
}