//! Import functionality for various map formats

pub mod azgaar;

use crate::{WorldMap, Result};
use std::path::Path;

/// Trait for map importers
pub trait MapImporter {
    /// Check if this importer can handle the given file
    fn can_import(&self, file_path: &Path) -> bool;
    
    /// Import a map from the given file
    fn import(&self, file_path: &Path) -> Result<WorldMap>;
    
    /// Get a preview of the map without full import
    fn get_preview(&self, file_path: &Path) -> Result<MapPreview>;
}

/// Preview information for a map file
#[derive(Debug, Clone)]
pub struct MapPreview {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub seed: Option<u64>,
    pub version: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub file_size: u64,
    pub format: String,
}

/// Registry of available importers
pub struct ImporterRegistry {
    importers: Vec<Box<dyn MapImporter>>,
}

impl ImporterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            importers: Vec::new(),
        };
        
        // Register built-in importers
        registry.register(Box::new(azgaar::AzgaarImporter::new()));
        
        registry
    }
    
    pub fn register(&mut self, importer: Box<dyn MapImporter>) {
        self.importers.push(importer);
    }
    
    pub fn find_importer(&self, file_path: &Path) -> Option<&dyn MapImporter> {
        self.importers
            .iter()
            .find(|importer| importer.can_import(file_path))
            .map(|importer| importer.as_ref())
    }
    
    pub fn import(&self, file_path: &Path) -> Result<WorldMap> {
        if let Some(importer) = self.find_importer(file_path) {
            importer.import(file_path)
        } else {
            Err(crate::WorldFoundryError::Import(
                format!("No importer found for file: {}", file_path.display())
            ))
        }
    }
    
    pub fn get_preview(&self, file_path: &Path) -> Result<MapPreview> {
        if let Some(importer) = self.find_importer(file_path) {
            importer.get_preview(file_path)
        } else {
            Err(crate::WorldFoundryError::Import(
                format!("No importer found for file: {}", file_path.display())
            ))
        }
    }
}

impl Default for ImporterRegistry {
    fn default() -> Self {
        Self::new()
    }
}