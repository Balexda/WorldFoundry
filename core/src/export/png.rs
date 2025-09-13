use std::sync::Arc;
use anyhow::Result;
use image::{ImageBuffer, Rgba, RgbaImage};

use crate::data::WorldMap;
use crate::rendering::tile::{TileRenderer, TileCoord, TILE_SIZE};

/// PNG exporter for world maps
pub struct PngExporter {
    // Configuration options can be added here
}

impl PngExporter {
    /// Create a new PNG exporter
    pub fn new() -> Self {
        Self {}
    }

    /// Export a world map to PNG
    pub fn export(
        &self,
        world_data: &Arc<WorldMap>,
        output_path: &str,
        width: u32,
        height: u32,
        style_json: Option<&str>,
    ) -> Result<()> {
        // Create a tile renderer for generating the image
        let renderer = TileRenderer::new(world_data.clone())?;
        
        // Parse style options (for future use)
        let _style_options = if let Some(style) = style_json {
            self.parse_style_json(style)?
        } else {
            StyleOptions::default()
        };

        // Calculate the zoom level that best fits the requested dimensions
        let zoom = self.calculate_optimal_zoom(world_data, width, height);
        
        // Create the output image
        let mut img: RgbaImage = ImageBuffer::new(width, height);
        
        // Calculate how many tiles we need
        let tiles_x = (width + TILE_SIZE - 1) / TILE_SIZE;
        let tiles_y = (height + TILE_SIZE - 1) / TILE_SIZE;
        
        // Render each tile and composite into the final image
        for ty in 0..tiles_y {
            for tx in 0..tiles_x {
                let tile_coord = TileCoord {
                    zoom,
                    x: tx,
                    y: ty,
                };
                
                // Render the tile
                let tile_data = renderer.render_tile(&tile_coord)?;
                
                // Copy tile data to the output image
                self.copy_tile_to_image(&mut img, &tile_data, tx, ty, width, height)?;
            }
        }
        
        // Save the image
        img.save(output_path)?;
        
        Ok(())
    }

    /// Calculate the optimal zoom level for the given dimensions
    fn calculate_optimal_zoom(&self, world_data: &WorldMap, width: u32, height: u32) -> u32 {
        let world_width = world_data.metadata.width as f64;
        let world_height = world_data.metadata.height as f64;
        
        // Find zoom level where world roughly fits in the requested dimensions
        let scale_x = width as f64 / world_width;
        let scale_y = height as f64 / world_height;
        let scale = scale_x.min(scale_y);
        
        // Convert scale to zoom level (zoom 0 = scale 1.0, zoom 1 = scale 2.0, etc.)
        let zoom = scale.log2().round() as i32;
        zoom.max(0).min(10) as u32 // Clamp to reasonable range
    }

    /// Copy tile data to the output image
    fn copy_tile_to_image(
        &self,
        img: &mut RgbaImage,
        tile_data: &[u8],
        tile_x: u32,
        tile_y: u32,
        img_width: u32,
        img_height: u32,
    ) -> Result<()> {
        let start_x = tile_x * TILE_SIZE;
        let start_y = tile_y * TILE_SIZE;
        
        for py in 0..TILE_SIZE {
            for px in 0..TILE_SIZE {
                let img_x = start_x + px;
                let img_y = start_y + py;
                
                // Skip pixels outside the image bounds
                if img_x >= img_width || img_y >= img_height {
                    continue;
                }
                
                let tile_idx = ((py * TILE_SIZE + px) * 4) as usize;
                if tile_idx + 3 < tile_data.len() {
                    let r = tile_data[tile_idx];
                    let g = tile_data[tile_idx + 1];
                    let b = tile_data[tile_idx + 2];
                    let a = tile_data[tile_idx + 3];
                    
                    img.put_pixel(img_x, img_y, Rgba([r, g, b, a]));
                }
            }
        }
        
        Ok(())
    }

    /// Parse style JSON options
    fn parse_style_json(&self, style_json: &str) -> Result<StyleOptions> {
        // For now, just return default options
        // In the future, this could parse JSON for customization
        let _parsed: serde_json::Value = serde_json::from_str(style_json)?;
        Ok(StyleOptions::default())
    }
}

/// Style options for PNG export
#[derive(Debug, Clone)]
struct StyleOptions {
    show_labels: bool,
    opacity: f64,
    // Add more style options as needed
}

impl Default for StyleOptions {
    fn default() -> Self {
        Self {
            show_labels: true,
            opacity: 1.0,
        }
    }
}