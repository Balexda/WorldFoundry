//! Rendering system for world maps

use crate::{WorldMap, Result};
use skia_safe as skia;

/// Rendering configuration
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub style: RenderStyle,
    pub layers: Vec<LayerConfig>,
    pub zoom_level: f32,
    pub center_x: f32,
    pub center_y: f32,
}

/// Rendering style presets
#[derive(Debug, Clone)]
pub enum RenderStyle {
    Political,
    Physical,
    Cultural,
    Biome,
    Height,
    Temperature,
    Precipitation,
    Custom(CustomStyle),
}

#[derive(Debug, Clone)]
pub struct CustomStyle {
    pub background_color: skia::Color,
    pub water_color: skia::Color,
    pub land_colors: Vec<(f32, skia::Color)>, // height -> color mapping
    pub border_color: skia::Color,
    pub border_width: f32,
    pub text_color: skia::Color,
    pub text_size: f32,
}

/// Layer configuration for rendering
#[derive(Debug, Clone)]
pub struct LayerConfig {
    pub layer_type: LayerType,
    pub visible: bool,
    pub opacity: f32,
    pub z_order: i32,
}

#[derive(Debug, Clone)]
pub enum LayerType {
    Heightmap,
    Water,
    Biomes,
    Political,
    Cultural,
    Settlements,
    Rivers,
    Routes,
    Markers,
    Labels,
    Grid,
}

/// Main renderer for world maps
pub struct WorldRenderer {
    surface: Option<skia::Surface>,
    canvas: Option<skia::Canvas>,
}

impl WorldRenderer {
    pub fn new() -> Self {
        Self {
            surface: None,
            canvas: None,
        }
    }
    
    /// Initialize the renderer with given dimensions
    pub fn initialize(&mut self, width: i32, height: i32) -> Result<()> {
        let info = skia::ImageInfo::new_n32_premul((width, height), None);
        let surface = skia::surfaces::raster(&info, None, None)
            .ok_or_else(|| crate::WorldFoundryError::Rendering("Failed to create surface".to_string()))?;
            
        self.surface = Some(surface);
        Ok(())
    }
    
    /// Render a world map to the current surface
    pub fn render(&mut self, world_map: &WorldMap, config: &RenderConfig) -> Result<()> {
        if self.surface.is_none() {
            self.initialize(config.width as i32, config.height as i32)?;
        }
        
        let surface = self.surface.as_mut()
            .ok_or_else(|| crate::WorldFoundryError::Rendering("Surface not initialized".to_string()))?;
            
        let canvas = surface.canvas();
        
        // Clear canvas
        canvas.clear(skia::Color::WHITE);
        
        // Render layers in z-order
        let mut sorted_layers = config.layers.clone();
        sorted_layers.sort_by_key(|layer| layer.z_order);
        
        for layer in &sorted_layers {
            if layer.visible {
                Self::render_layer(canvas, world_map, layer, config)?;
            }
        }
        
        Ok(())
    }
    
    /// Get the rendered image as bytes
    pub fn get_image_bytes(&mut self) -> Result<Vec<u8>> {
        let surface = self.surface.as_mut()
            .ok_or_else(|| crate::WorldFoundryError::Rendering("Surface not initialized".to_string()))?;
            
        let image = surface.image_snapshot();
        let data = image.encode(None, skia::EncodedImageFormat::PNG, None)
            .ok_or_else(|| crate::WorldFoundryError::Rendering("Failed to encode image".to_string()))?;
            
        Ok(data.as_bytes().to_vec())
    }
    
    fn render_layer(canvas: &skia::Canvas, world_map: &WorldMap, layer: &LayerConfig, config: &RenderConfig) -> Result<()> {
        match layer.layer_type {
            LayerType::Heightmap => Self::render_heightmap(canvas, world_map, layer, config),
            LayerType::Water => Self::render_water(canvas, world_map, layer, config),
            LayerType::Biomes => Self::render_biomes(canvas, world_map, layer, config),
            LayerType::Political => Self::render_political(canvas, world_map, layer, config),
            LayerType::Cultural => Self::render_cultural(canvas, world_map, layer, config),
            LayerType::Settlements => Self::render_settlements(canvas, world_map, layer, config),
            LayerType::Rivers => Self::render_rivers(canvas, world_map, layer, config),
            LayerType::Routes => Self::render_routes(canvas, world_map, layer, config),
            LayerType::Markers => Self::render_markers(canvas, world_map, layer, config),
            LayerType::Labels => Self::render_labels(canvas, world_map, layer, config),
            LayerType::Grid => Self::render_grid(canvas, world_map, layer, config),
        }
    }
    
    fn render_heightmap(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement heightmap rendering
        Ok(())
    }
    
    fn render_water(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement water rendering
        Ok(())
    }
    
    fn render_biomes(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement biome rendering
        Ok(())
    }
    
    fn render_political(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement political boundaries rendering
        Ok(())
    }
    
    fn render_cultural(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement cultural regions rendering
        Ok(())
    }
    
    fn render_settlements(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement settlements rendering
        Ok(())
    }
    
    fn render_rivers(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement rivers rendering
        Ok(())
    }
    
    fn render_routes(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement routes rendering
        Ok(())
    }
    
    fn render_markers(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement markers rendering
        Ok(())
    }
    
    fn render_labels(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement labels rendering
        Ok(())
    }
    
    fn render_grid(_canvas: &skia::Canvas, _world_map: &WorldMap, _layer: &LayerConfig, _config: &RenderConfig) -> Result<()> {
        // TODO: Implement grid rendering
        Ok(())
    }
}

impl Default for WorldRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            style: RenderStyle::Political,
            layers: vec![
                LayerConfig { layer_type: LayerType::Heightmap, visible: true, opacity: 1.0, z_order: 0 },
                LayerConfig { layer_type: LayerType::Water, visible: true, opacity: 1.0, z_order: 1 },
                LayerConfig { layer_type: LayerType::Political, visible: true, opacity: 0.8, z_order: 2 },
                LayerConfig { layer_type: LayerType::Settlements, visible: true, opacity: 1.0, z_order: 3 },
                LayerConfig { layer_type: LayerType::Labels, visible: true, opacity: 1.0, z_order: 4 },
            ],
            zoom_level: 1.0,
            center_x: 0.5,
            center_y: 0.5,
        }
    }
}