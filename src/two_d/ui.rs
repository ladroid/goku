extern crate sdl2;
// extern  crate gl;

use sdl2::render::Canvas;

use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;
use serde::ser::SerializeTuple;

// UI -> Customizable
    // Button +
    // Radio button
    // Checkbox +
    // Slider +
    // Text box +
// UI Layer
pub struct Layer<'a> {
    pub buttons: Vec<std::rc::Rc<Button<'a>>>,
    pub checkboxes: Vec<&'a mut Checkbox<'a>>,
    // Add other UI elements as needed
}

#[allow(dead_code)]
impl<'a> Layer<'a> {
    pub fn new() -> Self {
        Self { buttons: Vec::new(), checkboxes: Vec::new() }
    }

    pub fn add_button(&mut self, button: std::rc::Rc<Button<'a>>) {
        self.buttons.push(button);
    }

    pub fn add_checkbox(&mut self, checkbox: &'a mut Checkbox<'a>) {
        self.checkboxes.push(checkbox);
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>, color_text: sdl2::pixels::Color) -> Result<(), String> {
        for button in &self.buttons {
            button.render(canvas, color_text)?;
        }
        Ok(())
    }

    pub fn handle_mouse_click(&mut self, x: i32, y: i32) {
        for button in &self.buttons {
            if button.is_pressed(x, y) {
                button.on_click();
            }
        }

        for checkbox in &mut self.checkboxes {
            if checkbox.is_pressed(x, y) {
                checkbox.toggle();
            }
        }
    }
}

#[allow(dead_code)]
pub struct Button<'a> {
    text_box: std::rc::Rc<TextBox<'a>>,
    color: sdl2::pixels::Color,
    bg_rect: sdl2::rect::Rect, // Add this field
    pub center: (i32, i32),
    pub radius: i32,
    on_click_callback: Box<dyn Fn() + 'a>, // Add this field
}

struct RcTextBoxWrapper<'a>(std::rc::Rc<TextBox<'a>>);

impl<'a> Serialize for RcTextBoxWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        // Here we just dereference `Rc<TextBox>` to `TextBox`
        // and then call `serialize` on it.
        TextBox::serialize(&*self.0, serializer)
    }
}

#[derive(Debug)]
struct ColorWrapper(sdl2::pixels::Color);

impl Serialize for ColorWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (r, g, b) = (self.0.r, self.0.g, self.0.b);
        let mut state = serializer.serialize_struct("ColorWrapper", 3)?;
        state.serialize_field("r", &r)?;
        state.serialize_field("g", &g)?;
        state.serialize_field("b", &b)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ColorWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ColorFields {
            r: u8,
            g: u8,
            b: u8,
        }

        let fields = ColorFields::deserialize(deserializer)?;
        Ok(ColorWrapper(sdl2::pixels::Color::RGB(fields.r, fields.g, fields.b)))
    }
}

impl<'a> Serialize for Button<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Button", 3)?;
        state.serialize_field("text_box", &RcTextBoxWrapper(self.text_box.clone()))?;
        state.serialize_field("color", &ColorWrapper(self.color))?;
        state.serialize_field("bg_rect", &RectWrapper(self.bg_rect))?;
        // We cannot serialize the on_click_callback, so we will skip it.
        state.end()
    }
}

impl<'a> Button<'a> {
    pub fn new(text_box: std::rc::Rc<TextBox<'a>>, color: sdl2::pixels::Color, bg_rect: sdl2::rect::Rect, center: (i32, i32), radius: i32,on_click_callback: Box<dyn Fn() + 'a>,) -> Self {
        Self { text_box, color, bg_rect, center, radius, on_click_callback, }
    }

    pub fn is_pressed(&self, x: i32, y: i32) -> bool {
        self.bg_rect.contains_point(sdl2::rect::Point::new(x, y))
    }
    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>, color_text: sdl2::pixels::Color) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        // Draw the main rectangular body of the button
        canvas.fill_rect(sdl2::rect::Rect::new(
            self.bg_rect.x() + self.radius,
            self.bg_rect.y(),
            self.bg_rect.width() - 2 * self.radius as u32,
            self.bg_rect.height(),
        ))?;
        canvas.fill_rect(sdl2::rect::Rect::new(
            self.bg_rect.x(),
            self.bg_rect.y() + self.radius,
            self.bg_rect.width(),
            self.bg_rect.height() - 2 * self.radius as u32,
        ))?;

        // Draw the rounded corners using filled circles
        self.draw_filled_circle(canvas, self.bg_rect.x() + self.radius, self.bg_rect.y() + self.radius, self.radius)?; // top-left corner
        self.draw_filled_circle(canvas, self.bg_rect.x() + self.bg_rect.width() as i32 - self.radius, self.bg_rect.y() + self.radius, self.radius)?; // top-right corner
        self.draw_filled_circle(canvas, self.bg_rect.x() + self.radius, self.bg_rect.y() + self.bg_rect.height() as i32 - self.radius, self.radius)?; // bottom-left corner
        self.draw_filled_circle(canvas, self.bg_rect.x() + self.bg_rect.width() as i32 - self.radius, self.bg_rect.y() + self.bg_rect.height() as i32 - self.radius, self.radius)?; // bottom-right corner
        
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 0)); // Reset the draw color
        self.text_box.render(canvas, color_text)
    }

    fn draw_filled_circle(&self, canvas: &mut Canvas<sdl2::video::Window>, cx: i32, cy: i32, r: i32) -> Result<(), String> {
        for y in -r..=r {
            for x in -r..=r {
                if x * x + y * y <= r * r {
                    canvas.draw_point(sdl2::rect::Point::new(cx + x, cy + y))?;
                }
            }
        }
        Ok(())
    }

    pub fn on_click(&self) {
        (self.on_click_callback)();
    }
}

#[derive(Debug)]
pub struct RectWrapper(pub sdl2::rect::Rect);

impl Serialize for RectWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (x, y, w, h) = (self.0.x(), self.0.y(), self.0.width(), self.0.height());
        let mut tuple = serializer.serialize_tuple(4)?;
        tuple.serialize_element(&x)?;
        tuple.serialize_element(&y)?;
        tuple.serialize_element(&w)?;
        tuple.serialize_element(&h)?;
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for RectWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RectVisitor;

        impl<'de> serde::de::Visitor<'de> for RectVisitor {
            type Value = RectWrapper;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a tuple with 4 elements representing the SDL2 Rect")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<RectWrapper, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let x = seq.next_element()?.unwrap();
                let y = seq.next_element()?.unwrap();
                let w = seq.next_element()?.unwrap();
                let h = seq.next_element()?.unwrap();
                Ok(RectWrapper(sdl2::rect::Rect::new(x, y, w, h)))
            }
        }

        deserializer.deserialize_tuple(4, RectVisitor)
    }
}

pub struct TextBox<'a> {
    pub text: String,
    pub font: std::sync::Arc<sdl2::ttf::Font<'a, 'static>>,
    pub rect: sdl2::rect::Rect,
}

impl<'a> Serialize for TextBox<'a> {
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
impl<'a> TextBox<'a> {
    pub fn new(text: String, font: std::sync::Arc<sdl2::ttf::Font<'a, 'static>>, rect: sdl2::rect::Rect) -> Self {
        Self { text, font, rect }
    }

    // pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
    //     let surface = self.font.render(&self.text).blended(sdl2::pixels::Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string())?;
    //     let texture_creator = canvas.texture_creator();
    //     let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
    //     canvas.copy(&texture, None, self.rect)?;
    //     Ok(())
    // }

    // Add a method to set the text
    pub fn set_text(&mut self, new_text: String) {
        self.text = new_text;
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>, color_text: sdl2::pixels::Color) -> Result<(), String> {
        let surface = self.font.render(&self.text).blended(color_text).map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, self.rect)?;
    
        Ok(())
    }    
}

#[allow(dead_code)]
pub struct Checkbox<'a> {
    pub button: Button<'a>,
    pub checked: bool,
}

#[allow(dead_code)]
impl<'a> Checkbox<'a> {
    pub fn new(
        text_box: std::rc::Rc<TextBox<'a>>,
        color: sdl2::pixels::Color,
        bg_rect: sdl2::rect::Rect,
    ) -> Self {
        let button = Button::new(
            text_box,
            color,
            bg_rect,
            (0, 0),
            0,
            Box::new(|| {
                println!("Checkbox clicked!");
            }),
        );
        Self { button, checked: false }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>, color_text: sdl2::pixels::Color) -> Result<(), String> {
        self.button.render(canvas, color_text)?;

        if self.checked {
            let checkmark_rect = sdl2::rect::Rect::new(
                self.button.bg_rect.x() + self.button.bg_rect.width() as i32 / 4,
                self.button.bg_rect.y() + self.button.bg_rect.height() as i32 / 4,
                self.button.bg_rect.width() / 2,
                self.button.bg_rect.height() / 2,
            );
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            canvas.fill_rect(checkmark_rect)?;
        }

        Ok(())
    }

    pub fn is_pressed(&self, x: i32, y: i32) -> bool {
        self.button.is_pressed(x, y)
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
        (self.button.on_click_callback)();
    }
}

pub struct Slider<'a> {
    pub background_rect: sdl2::rect::Rect,
    pub slider_rect: sdl2::rect::Rect,
    pub slider_color: sdl2::pixels::Color,
    pub background_color: sdl2::pixels::Color,
    pub on_value_changed_callback: Box<dyn Fn(f32) + 'a>,
    pub value: f32,
}

#[allow(dead_code)]
impl<'a> Slider<'a> {
    pub fn new(
        background_rect: sdl2::rect::Rect,
        slider_rect: sdl2::rect::Rect,
        slider_color: sdl2::pixels::Color,
        background_color: sdl2::pixels::Color,
        on_value_changed_callback: Box<dyn Fn(f32) + 'a>,
    ) -> Self {
        Self {
            background_rect,
            slider_rect,
            slider_color,
            background_color,
            on_value_changed_callback,
            value: 0.0,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(self.background_color);
        canvas.fill_rect(self.background_rect)?;
        canvas.set_draw_color(self.slider_color);
        canvas.fill_rect(self.slider_rect)?;
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 0)); // Reset the draw color
        Ok(())
    }

    pub fn handle_mouse_click(&mut self, x: i32, y: i32) {
        if self.background_rect.contains_point(sdl2::rect::Point::new(x, y)) {
            let new_x = x - self.slider_rect.width() as i32 / 2;
            let max_x = self.background_rect.x() + self.background_rect.width() as i32 - self.slider_rect.width() as i32;
            let min_x = self.background_rect.x();
            self.slider_rect.set_x(new_x.clamp(min_x, max_x));
            self.update_value();
        }
    }

    pub fn update_value(&mut self) {
        let relative_x = self.slider_rect.x() - self.background_rect.x();
        let width_diff = self.background_rect.width() - self.slider_rect.width();
        self.value = relative_x as f32 / width_diff as f32;
        (self.on_value_changed_callback)(self.value);
    }
}