use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Component {
    pub name: String,
    pub children: Vec<Component>,
}