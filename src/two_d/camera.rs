extern crate sdl2;
// extern  crate gl;

use sdl2::rect::Rect;

// use gl::types::*;
use nalgebra::Vector2;

use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;

// camera
pub struct Camera {
    pub position: Vector2<i32>,
    pub size: Vector2<u32>,
}

impl Serialize for Camera {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Camera", 2)?;
        state.serialize_field("position", &[self.position.x, self.position.y])?;
        state.serialize_field("size", &[self.size.x, self.size.y])?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Position, Size }

        struct CameraVisitor;

        impl<'de> serde::de::Visitor<'de> for CameraVisitor {
            type Value = Camera;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Camera")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Camera, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut position = None;
                let mut size = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Position => {
                            let coords: [i32; 2] = map.next_value()?;
                            position = Some(Vector2::new(coords[0], coords[1]));
                        }
                        Field::Size => {
                            let coords: [i32; 2] = map.next_value()?;
                            size = Some(Vector2::new(coords[0] as u32, coords[1] as u32));
                        }
                    }
                }
                let position = position.ok_or_else(|| serde::de::Error::missing_field("position"))?;
                let size = size.ok_or_else(|| serde::de::Error::missing_field("size"))?;
                Ok(Camera { position, size })
            }
        }

        const FIELDS: &'static [&'static str] = &["position", "size"];
        deserializer.deserialize_struct("Camera", FIELDS, CameraVisitor)
    }
}

#[allow(dead_code)]
impl Camera {
    pub fn new(position: Vector2<i32>, size: Vector2<u32>) -> Self {
        Self { position, size }
    }

    pub fn update(&mut self, target_position: Vector2<i32>) {
        self.position.x = target_position.x - self.size.x as i32 / 2;
        self.position.y = target_position.y - self.size.y as i32 / 2;
    }

    pub fn transform_rect(&self, rect: Rect) -> Rect {
        Rect::new(rect.x() - self.position.x, rect.y() - self.position.y, rect.width(), rect.height())
    }
}