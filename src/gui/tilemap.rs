// Tilemap
struct Tilemap {
    width: usize,
    height: usize,
    tiles: Vec<Option<(u32, usize)>>, // Store texture IDs and frame indices for each tile, None if no tile
    tile_width: u32,
    tile_height: u32,
    selected_tile: Option<(u32, usize)>, // (texture_id, frame_index)
}

impl Tilemap {
    fn new(width: usize, height: usize, tile_width: u32, tile_height: u32) -> Self {
        Tilemap {
            width,
            height,
            tiles: vec![None; width * height],
            tile_width,
            tile_height,
            selected_tile: None,
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, texture_id: u32, frame_index: usize) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = Some((texture_id, frame_index));
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<(u32, usize)> {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x]
        } else {
            None
        }
    }
}
