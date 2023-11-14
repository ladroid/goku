extern crate sdl2;
// extern  crate gl;

use crate::two_d::texture_manager::TextureManager;

use std::path::Path;
use sdl2::rect::Rect;

pub type TextureGrid<'a> = Vec<Vec<TextureManager<'a>>>;

pub struct Tile<'a> {
    pub textures: Vec<&'a TextureManager<'a>>,
    pub tile_map: Vec<Vec<u32>>,
    pub colliders: Vec<Rect>,
    pub texture_grid: Option<TextureGrid<'a>>,
}

#[allow(dead_code)]
impl<'a> Tile<'a> {
    pub fn new(tile_map_path: &Path, textures: Vec<& 'a TextureManager<'a>>, texture_grid: Option<TextureGrid<'a>>) -> Result<Self, Box<dyn std::error::Error>> {
        let tile_map_string = std::fs::read_to_string(tile_map_path).map_err(|e| e.to_string())?;
        let mut tile_map: Vec<Vec<u32>> = Vec::new();
        let mut colliders: Vec<Rect> = Vec::new();

        for (y, line) in tile_map_string.lines().enumerate() {
            let row: Vec<u32> = line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            for (x, &tile_type) in row.iter().enumerate() {
                if tile_type == 2 {
                    let collider = Rect::new((x * 82) as i32, (y * 82) as i32, 82, 82);
                    colliders.push(collider);
                }
            }
            tile_map.push(row);
        }
        Ok(Self { textures, tile_map, colliders, texture_grid })
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, tile_size: (u32, u32)) -> Result<(), String> {
        let (tile_width, tile_height) = tile_size;
        let rows = self.tile_map.len();
        let cols = self.tile_map[0].len();
        for y in 0..rows {
            for x in 0..cols {
                let tile_index = self.tile_map[y][x] as usize;
                let texture_manager = &self.textures[tile_index];
                let dest = sdl2::rect::Rect::new(
                    (x * tile_width as usize) as i32,
                    (y * tile_height as usize) as i32,
                    tile_width,
                    tile_height,
                );
                texture_manager.render_texture(canvas, dest)?;
            }
        }
        Ok(())
    }

    pub fn set_texture_grid(&mut self, texture_grid: TextureGrid<'a>) {
        self.texture_grid = Some(texture_grid);
    }

    pub fn from_generated_map(
        generated_map: Vec<Vec<u32>>,
        textures: Vec<&'a TextureManager<'a>>,
        texture_grid: Option<TextureGrid<'a>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut colliders: Vec<Rect> = Vec::new();

        for (y, row) in generated_map.iter().enumerate() {
            for (x, &tile_type) in row.iter().enumerate() {
                if tile_type == 2 {
                    let collider = Rect::new((x * 82) as i32, (y * 82) as i32, 82, 82);
                    colliders.push(collider);
                }
            }
        }
        Ok(Self {
            textures,
            tile_map: generated_map,
            colliders,
            texture_grid,
        })
    }
}