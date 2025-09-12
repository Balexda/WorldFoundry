//! Export functionality for various map formats

use crate::{WorldMap, Result};
use std::path::Path;

/// Trait for map exporters
pub trait MapExporter {
    /// Export a world map to the given file path
    fn export(&self, world_map: &WorldMap, file_path: &Path) -> Result<()>;
    
    /// Get the file extension this exporter produces
    fn file_extension(&self) -> &'static str;
    
    /// Get a human-readable name for this export format
    fn format_name(&self) -> &'static str;
}

/// JSON exporter for World Foundry native format
pub struct JsonExporter;

impl MapExporter for JsonExporter {
    fn export(&self, world_map: &WorldMap, file_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(world_map)?;
        std::fs::write(file_path, json)?;
        Ok(())
    }
    
    fn file_extension(&self) -> &'static str {
        "json"
    }
    
    fn format_name(&self) -> &'static str {
        "World Foundry JSON"
    }
}

/// PNG image exporter
pub struct PngExporter {
    width: u32,
    height: u32,
}

impl PngExporter {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl MapExporter for PngExporter {
    fn export(&self, world_map: &WorldMap, file_path: &Path) -> Result<()> {
        use crate::rendering::{WorldRenderer, RenderConfig};
        
        let mut renderer = WorldRenderer::new();
        let config = RenderConfig {
            width: self.width,
            height: self.height,
            ..Default::default()
        };
        
        renderer.render(world_map, &config)?;
        let image_bytes = renderer.get_image_bytes()?;
        
        std::fs::write(file_path, image_bytes)?;
        Ok(())
    }
    
    fn file_extension(&self) -> &'static str {
        "png"
    }
    
    fn format_name(&self) -> &'static str {
        "PNG Image"
    }
}

/// GeoJSON exporter for geographic data
pub struct GeoJsonExporter;

impl MapExporter for GeoJsonExporter {
    fn export(&self, world_map: &WorldMap, file_path: &Path) -> Result<()> {
        // TODO: Convert world map data to GeoJSON format
        // This would be useful for integration with GIS software
        Err(crate::WorldFoundryError::Export(
            "GeoJSON export not yet implemented".to_string()
        ))
    }
    
    fn file_extension(&self) -> &'static str {
        "geojson"
    }
    
    fn format_name(&self) -> &'static str {
        "GeoJSON"
    }
}

/// SVG vector exporter
pub struct SvgExporter {
    width: u32,
    height: u32,
}

impl SvgExporter {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl MapExporter for SvgExporter {
    fn export(&self, _world_map: &WorldMap, _file_path: &Path) -> Result<()> {
        // TODO: Implement SVG export for vector graphics
        Err(crate::WorldFoundryError::Export(
            "SVG export not yet implemented".to_string()
        ))
    }
    
    fn file_extension(&self) -> &'static str {
        "svg"
    }
    
    fn format_name(&self) -> &'static str {
        "SVG Vector"
    }
}

/// Registry of available exporters
pub struct ExporterRegistry {
    exporters: Vec<Box<dyn MapExporter>>,
}

impl ExporterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            exporters: Vec::new(),
        };
        
        // Register built-in exporters
        registry.register(Box::new(JsonExporter));
        registry.register(Box::new(PngExporter::new(2048, 1024)));
        registry.register(Box::new(GeoJsonExporter));
        registry.register(Box::new(SvgExporter::new(2048, 1024)));
        
        registry
    }
    
    pub fn register(&mut self, exporter: Box<dyn MapExporter>) {
        self.exporters.push(exporter);
    }
    
    pub fn find_exporter(&self, extension: &str) -> Option<&dyn MapExporter> {
        self.exporters
            .iter()
            .find(|exporter| exporter.file_extension() == extension)
            .map(|exporter| exporter.as_ref())
    }
    
    pub fn export(&self, world_map: &WorldMap, file_path: &Path) -> Result<()> {
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| crate::WorldFoundryError::Export("No file extension provided".to_string()))?;
            
        if let Some(exporter) = self.find_exporter(extension) {
            exporter.export(world_map, file_path)
        } else {
            Err(crate::WorldFoundryError::Export(
                format!("No exporter found for extension: {}", extension)
            ))
        }
    }
    
    pub fn list_formats(&self) -> Vec<(&'static str, &'static str)> {
        self.exporters
            .iter()
            .map(|exporter| (exporter.file_extension(), exporter.format_name()))
            .collect()
    }
}

impl Default for ExporterRegistry {
    fn default() -> Self {
        Self::new()
    }
}