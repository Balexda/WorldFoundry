use std::sync::Arc;
use std::collections::HashMap;
use anyhow::Result;

use crate::data::WorldMap;
use crate::rendering::color::ColorPalette;

/// Tile coordinate in the tile pyramid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoord {
    pub zoom: u32,
    pub x: u32,
    pub y: u32,
}

/// Tile size constants
pub const TILE_SIZE: u32 = 256;
pub const TILE_BYTES: usize = (TILE_SIZE * TILE_SIZE * 4) as usize; // RGBA

/// Tile renderer that generates 256x256 RGBA tiles
pub struct TileRenderer {
    world_data: Arc<WorldMap>,
    palette: ColorPalette,
    pyramid_cache: HashMap<u32, TilePyramid>,
}

/// Tile pyramid for a specific zoom level
struct TilePyramid {
    zoom: u32,
    tiles_x: u32,
    tiles_y: u32,
    world_scale: f64,
}

impl TileRenderer {
    /// Create a new tile renderer
    pub fn new(world_data: Arc<WorldMap>) -> Result<Self> {
        let palette = ColorPalette::default();
        let mut renderer = Self {
            world_data,
            palette,
            pyramid_cache: HashMap::new(),
        };
        
        // Pre-compute pyramid levels
        renderer.build_pyramid()?;
        
        Ok(renderer)
    }

    /// Build the tile pyramid for different zoom levels
    fn build_pyramid(&mut self) -> Result<()> {
        let world_width = self.world_data.metadata.width as f64;
        let world_height = self.world_data.metadata.height as f64;
        
        // Calculate zoom levels (0 = most zoomed out, higher = more zoomed in)
        for zoom in 0..=10 {
            let scale = 2.0_f64.powi(zoom as i32);
            let tiles_x = ((world_width * scale) / TILE_SIZE as f64).ceil() as u32;
            let tiles_y = ((world_height * scale) / TILE_SIZE as f64).ceil() as u32;
            
            let pyramid = TilePyramid {
                zoom: zoom as u32,
                tiles_x: tiles_x.max(1),
                tiles_y: tiles_y.max(1),
                world_scale: scale,
            };
            
            self.pyramid_cache.insert(zoom as u32, pyramid);
        }
        
        Ok(())
    }

    /// Render a single tile
    pub fn render_tile(&self, coord: &TileCoord) -> Result<Vec<u8>> {
        let pyramid = self.pyramid_cache.get(&coord.zoom)
            .ok_or_else(|| anyhow::anyhow!("Invalid zoom level: {}", coord.zoom))?;

        if coord.x >= pyramid.tiles_x || coord.y >= pyramid.tiles_y {
            return Err(anyhow::anyhow!(
                "Tile coordinates out of bounds: ({}, {}) >= ({}, {})",
                coord.x, coord.y, pyramid.tiles_x, pyramid.tiles_y
            ));
        }

        let mut tile_data = vec![0u8; TILE_BYTES];
        
        // Calculate world coordinates for this tile
        let world_x_start = (coord.x as f64 * TILE_SIZE as f64) / pyramid.world_scale;
        let world_y_start = (coord.y as f64 * TILE_SIZE as f64) / pyramid.world_scale;
        let world_x_end = ((coord.x + 1) as f64 * TILE_SIZE as f64) / pyramid.world_scale;
        let world_y_end = ((coord.y + 1) as f64 * TILE_SIZE as f64) / pyramid.world_scale;

        // Render each pixel in the tile
        for py in 0..TILE_SIZE {
            for px in 0..TILE_SIZE {
                let world_x = world_x_start + (px as f64 / TILE_SIZE as f64) * (world_x_end - world_x_start);
                let world_y = world_y_start + (py as f64 / TILE_SIZE as f64) * (world_y_end - world_y_start);
                
                let color = self.sample_world_color(world_x, world_y);
                
                let pixel_idx = ((py * TILE_SIZE + px) * 4) as usize;
                if pixel_idx + 3 < tile_data.len() {
                    tile_data[pixel_idx] = color.r;
                    tile_data[pixel_idx + 1] = color.g;
                    tile_data[pixel_idx + 2] = color.b;
                    tile_data[pixel_idx + 3] = color.a;
                }
            }
        }

        Ok(tile_data)
    }

    /// Sample the world color at a specific coordinate
    fn sample_world_color(&self, x: f64, y: f64) -> RgbaColor {
        // Clamp coordinates to world bounds
        let x = x.max(0.0).min(self.world_data.metadata.width as f64 - 1.0);
        let y = y.max(0.0).min(self.world_data.metadata.height as f64 - 1.0);
        
        let cell_x = x as usize;
        let cell_y = y as usize;
        
        // Get the cell index
        let cell_idx = cell_y * self.world_data.metadata.width as usize + cell_x;
        
        if cell_idx >= self.world_data.cells.len() {
            return RgbaColor { r: 0, g: 0, b: 0, a: 255 }; // Black for out of bounds
        }
        
        let cell = &self.world_data.cells[cell_idx];
        
        // Determine color based on cell properties
        if cell.height < 0.2 {
            // Water
            self.palette.water_color(cell.height as f64)
        } else if cell.temperature > 0.8 && cell.precipitation < 0.3 {
            // Desert
            self.palette.desert_color(cell.temperature as f64, cell.precipitation as f64)
        } else if cell.temperature < 0.3 {
            // Snow/Ice
            self.palette.snow_color(cell.temperature as f64)
        } else if cell.height > 0.8 {
            // Mountains
            self.palette.mountain_color(cell.height as f64)
        } else {
            // Grassland/Forest based on precipitation
            if cell.precipitation > 0.6 {
                self.palette.forest_color(cell.precipitation as f64)
            } else {
                self.palette.grassland_color(cell.precipitation as f64)
            }
        }
    }
}

/// RGBA color representation
#[derive(Debug, Clone, Copy)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}