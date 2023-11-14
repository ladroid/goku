extern crate sdl2;
// extern  crate gl;

use crate::two_d::ui::RectWrapper;

use sdl2::render::Canvas;

use serde::Serialize;
use serde::ser::SerializeStruct;

pub struct DialogueTextBox<'a> {
    pub speaker: Option<String>,
    pub text: String,
    pub font: std::sync::Arc<sdl2::ttf::Font<'a, 'static>>,
    pub rect: sdl2::rect::Rect,
}

impl<'a> Serialize for DialogueTextBox<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("TextBox", 3)?;
        state.serialize_field("text", &self.text)?;
        state.serialize_field("rect", &RectWrapper(self.rect))?;
        // We cannot serialize the font, so we will skip it.
        state.end()
    }
}

#[allow(dead_code)]
impl<'a> DialogueTextBox<'a> {
    pub fn new(speaker: Option<String>, text: String, font: std::sync::Arc<sdl2::ttf::Font<'a, 'static>>, rect: sdl2::rect::Rect) -> Self {
        Self { speaker, text, font, rect }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        // Draw dialogue background
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 128));  // semi-transparent black
        canvas.fill_rect(self.rect)?;
        
        if let Some(ref speaker) = self.speaker {
            let surface = self.font.render(&speaker).blended(sdl2::pixels::Color::RGBA(255, 255, 255, 0)).map_err(|e| e.to_string())?;
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
            canvas.copy(&texture, None, sdl2::rect::Rect::new(self.rect.x, self.rect.y - 20, 120, 20))?; 
            // Offset by 20 pixels, adjust accordingly
        }
    
        let surface = self.font.render(&self.text).blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255)).map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, self.rect)?;
    
        Ok(())
    }    
}

#[allow(dead_code)]
pub struct DialogueOption<'a> {
    pub text: String,
    pub action: fn(),
    pub rect: sdl2::rect::Rect,  // New field
    pub font: std::sync::Arc<sdl2::ttf::Font<'a, 'static>>,  // New field
}

// struct DialogueOption {
//     pub text: String,
//     pub action: fn(),
// }

pub struct DialogueBox<'a> {
    text_boxes: Vec<std::rc::Rc<DialogueTextBox<'a>>>,
    options: Vec<DialogueOption<'a>>,
    pub is_active: bool,
}

#[allow(dead_code)]
impl<'a> DialogueBox<'a> {
    pub fn new() -> Self {
        Self {
            text_boxes: Vec::new(),
            options: Vec::new(),
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn add_text(&mut self, text_box: std::rc::Rc<DialogueTextBox<'a>>) {
        self.text_boxes.push(text_box);
    }

    fn add_option(&mut self, option: DialogueOption<'a>) {
        self.options.push(option);
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        if !self.is_active {
            return Ok(());
        }
    
        // Render each text box
        for text_box in &self.text_boxes {
            text_box.render(canvas)?;
        }
    
        let mut option_y_offset = 90;  // Start rendering options 90 pixels below the text
        for option in &self.options {
            let surface = option.font.render(&option.text).blended(sdl2::pixels::Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())?;
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
            canvas.copy(&texture, None, sdl2::rect::Rect::new(self.text_boxes[0].rect.x + 10, self.text_boxes[0].rect.y + option_y_offset, self.text_boxes[0].rect.width() - 20, 20))?;
            option_y_offset += 25;  // Adjust this value to change the space between options
        }
        
        Ok(())
    }
}