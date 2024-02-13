use sdl2::pixels::Color as SdlColor;

pub struct Color {
    pub sdl_color: SdlColor,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            sdl_color: SdlColor::RGB(r, g, b),
        }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            sdl_color: SdlColor::RGBA(r, g, b, a),
        }
    }

    // Optionally, implement getters for the color components if needed
    pub fn r(&self) -> u8 {
        self.sdl_color.r
    }

    pub fn g(&self) -> u8 {
        self.sdl_color.g
    }

    pub fn b(&self) -> u8 {
        self.sdl_color.b
    }

    pub fn a(&self) -> u8 {
        self.sdl_color.a
    }

    // Helper method to get the underlying SDL2 Color for rendering purposes
    pub fn sdl_color(&self) -> SdlColor {
        self.sdl_color
    }
}