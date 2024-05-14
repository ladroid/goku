pub struct Grid {
    pub spacing: f32,
    pub color: [f32; 4],
}

impl Grid {
    pub fn new(spacing: f32, color: [f32; 4]) -> Self {
        Grid { spacing, color }
    }
}