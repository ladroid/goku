extern crate sdl2;
// extern  crate gl;

use sdl2::rect::Rect;

// use gl::types::*;
use nalgebra::Vector2;

use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;

// camera
pub struct Camera3D {
    pub position: Vector2<f64>, // Using f64 for more precise calculations
    pub direction: Vector2<f64>, // Viewing direction
    pub plane: Vector2<f64>, // Camera plane for field of view
    pub size: Vector2<u32>, // Size of the viewport
}

impl Serialize for Camera3D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Camera3D", 2)?;
        state.serialize_field("position", &[self.position.x, self.position.y])?;
        state.serialize_field("size", &[self.size.x, self.size.y])?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Camera3D {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Position, Size, Direction, Plane }

        struct CameraVisitor;

        impl<'de> serde::de::Visitor<'de> for CameraVisitor {
            type Value = Camera3D;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Camera3D")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Camera3D, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut position = None;
                let mut size = None;
                let mut direction = None;
                let mut plane = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Position => {
                            let coords: [f64; 2] = map.next_value()?; // Adjusted type to f64
                            position = Some(Vector2::new(coords[0], coords[1]));
                        },
                        Field::Size => {
                            let coords: [u32; 2] = map.next_value()?; // Adjusted type to u32
                            size = Some(Vector2::new(coords[0], coords[1]));
                        },
                        Field::Direction => {
                            let dir: [f64; 2] = map.next_value()?;
                            direction = Some(Vector2::new(dir[0], dir[1]));
                        },
                        Field::Plane => {
                            let pl: [f64; 2] = map.next_value()?;
                            plane = Some(Vector2::new(pl[0], pl[1]));
                        },
                    }
                }
                let position = position.ok_or_else(|| serde::de::Error::missing_field("position"))?;
                let size = size.ok_or_else(|| serde::de::Error::missing_field("size"))?;
                let direction = direction.ok_or_else(|| serde::de::Error::missing_field("direction"))?;
                let plane = plane.ok_or_else(|| serde::de::Error::missing_field("plane"))?;

                Ok(Camera3D { position, direction, plane, size })
            }
        }

        const FIELDS: &'static [&'static str] = &["position", "size", "direction", "plane"];
        deserializer.deserialize_struct("Camera3D", FIELDS, CameraVisitor)
    }
}

#[allow(dead_code)]
impl Camera3D {
    pub fn new(position: Vector2<f64>, direction: Vector2<f64>, plane: Vector2<f64>, size: Vector2<u32>) -> Self {
        Self { position, direction, plane, size }
    }

    pub fn update(&mut self, target_position: Vector2<f64>, target_direction: Vector2<f64>) {
        self.position = target_position;
        self.direction = target_direction;
    }
}