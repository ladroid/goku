use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TranslationRequest {
    pub original_text: String,
    pub translated_text: Option<String>,
}