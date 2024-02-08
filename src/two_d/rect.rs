use sdl2::rect::Rect as SdlRect;

#[derive(PartialEq, Copy, Clone)]
pub struct Rect {
    sdl_rect: SdlRect,
}

#[allow(dead_code)]
impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Rect {
            sdl_rect: SdlRect::new(x, y, w, h),
        }
    }

    pub fn x(&self) -> i32 {
        self.sdl_rect.x()
    }

    pub fn y(&self) -> i32 {
        self.sdl_rect.y()
    }

    pub fn set_x(&mut self, x: i32) {
        self.sdl_rect.set_x(x);
    }

    pub fn set_y(&mut self, y: i32) {
        self.sdl_rect.set_y(y);
    }

    pub fn width(&self) -> u32 {
        self.sdl_rect.width()
    }

    pub fn height(&self) -> u32 {
        self.sdl_rect.height()
    }

    // Method to set width
    pub fn set_width(&mut self, w: u32) {
        self.sdl_rect.set_width(w);
    }

    // Method to set height
    pub fn set_height(&mut self, h: u32) {
        self.sdl_rect.set_height(h);
    }
    
    // Helper method to get the underlying SDL2 Rect for rendering purposes
    pub fn sdl_rect(&self) -> &SdlRect {
        &self.sdl_rect
    }

    pub fn to_sdl(&self) -> Option<sdl2::rect::Rect> {
        Some(self.sdl_rect.clone())
    }
}