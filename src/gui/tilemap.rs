use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;
use std::path::Path;
use std::str;
use crate::gui::shader::sdl_surface_to_gl_texture;

pub struct Tilemap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u32>, // Stores texture IDs or tile indices
}

impl Tilemap {
    pub fn new(width: usize, height: usize) -> Self {
        Tilemap {
            width,
            height,
            tiles: vec![0; width * height], // Initialize with zeros
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: u32) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = tile;
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x]
        } else {
            0
        }
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "{} {}", self.width, self.height)?;
        for tile in &self.tiles {
            writeln!(file, "{}", tile)?;
        }
        Ok(())
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        reader.read_line(&mut line)?;
        let dimensions: Vec<usize> = line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (width, height) = (dimensions[0], dimensions[1]);

        let mut tiles = vec![];
        for line in reader.lines() {
            tiles.push(line?.trim().parse().unwrap());
        }

        Ok(Tilemap { width, height, tiles })
    }
}
pub fn load_texture_from_path(path: &str) -> Result<(u32, u32, u32), String> {
    let surface = Surface::from_file(Path::new(path))?;
    let texture_id = sdl_surface_to_gl_texture(&surface)?;
    Ok((texture_id, surface.width(), surface.height()))
}