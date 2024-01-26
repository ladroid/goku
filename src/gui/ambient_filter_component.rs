use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AmbientFilterComponent {
    pub intensity: f32,
    pub color: [f32; 4], // RGBA color
}