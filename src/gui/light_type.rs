use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub enum LightType {
    #[default] 
    None,
    Point,
    Spotlight,
}