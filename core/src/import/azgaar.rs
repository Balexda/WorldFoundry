//! Azgaar Fantasy Map Generator import functionality

use super::{MapImporter, MapPreview};
use crate::{WorldMap, Result, WorldFoundryError};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Importer for Azgaar Fantasy Map Generator files
pub struct AzgaarImporter;

impl AzgaarImporter {
    pub fn new() -> Self {
        Self
    }
    
    /// Import from file path (convenience method for C API)
    pub fn import_from_file(&self, file_path: &str) -> Result<WorldMap> {
        let path = Path::new(file_path);
        self.import(path)
    }
}

impl MapImporter for AzgaarImporter {
    fn can_import(&self, file_path: &Path) -> bool {
        if let Some(extension) = file_path.extension() {
            match extension.to_str() {
                Some("map") => true,  // Azgaar .map files
                Some("json") => {
                    // Check if it's an Azgaar JSON export
                    if let Ok(content) = fs::read_to_string(file_path) {
                        content.contains("Azgaar") || content.contains("Fantasy-Map-Generator")
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            false
        }
    }
    
    fn import(&self, file_path: &Path) -> Result<WorldMap> {
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| WorldFoundryError::Import("Invalid file extension".to_string()))?;
            
        match extension {
            "map" => self.import_map_file(file_path),
            "json" => self.import_json_file(file_path),
            _ => Err(WorldFoundryError::Import(
                format!("Unsupported file extension: {}", extension)
            )),
        }
    }
    
    fn get_preview(&self, file_path: &Path) -> Result<MapPreview> {
        let metadata = fs::metadata(file_path)
            .map_err(|e| WorldFoundryError::Import(format!("Cannot read file metadata: {}", e)))?;
            
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown");
            
        // For now, return basic preview info
        // TODO: Parse file headers for more detailed preview
        Ok(MapPreview {
            name: file_path.file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            width: 0,  // TODO: Parse from file
            height: 0, // TODO: Parse from file
            seed: None,
            version: None,
            created_at: None,
            file_size: metadata.len(),
            format: format!("Azgaar {}", extension.to_uppercase()),
        })
    }
}

impl AzgaarImporter {
    fn import_map_file(&self, file_path: &Path) -> Result<WorldMap> {
        // TODO: Implement .map file parsing
        // The .map format is a custom binary/text format used by Azgaar
        // We need to reverse engineer or use the existing JavaScript parser
        Err(WorldFoundryError::Import(
            "Azgaar .map file import not yet implemented".to_string()
        ))
    }
    
    fn import_json_file(&self, file_path: &Path) -> Result<WorldMap> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| WorldFoundryError::Import(format!("Cannot read JSON file: {}", e)))?;
            
        let azgaar_data: AzgaarJsonData = serde_json::from_str(&content)
            .map_err(|e| WorldFoundryError::Import(format!("Invalid Azgaar JSON format: {}", e)))?;
            
        self.convert_azgaar_to_world_map(azgaar_data)
    }
    
    fn convert_azgaar_to_world_map(&self, azgaar_data: AzgaarJsonData) -> Result<WorldMap> {
        use crate::data::*;
        use uuid::Uuid;
        use chrono::Utc;
        
        // Convert Azgaar data to World Foundry format
        let metadata = MapMetadata {
            id: Uuid::new_v4(),
            name: azgaar_data.info.map_name.unwrap_or_else(|| "Imported Map".to_string()),
            version: azgaar_data.info.version,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            width: azgaar_data.info.width,
            height: azgaar_data.info.height,
            seed: azgaar_data.info.seed,
            settings: MapSettings {
                distance_unit: azgaar_data.settings.distance_unit.unwrap_or_else(|| "km".to_string()),
                distance_scale: azgaar_data.settings.distance_scale.unwrap_or(1.0),
                area_unit: azgaar_data.settings.area_unit.unwrap_or_else(|| "km²".to_string()),
                height_unit: azgaar_data.settings.height_unit.unwrap_or_else(|| "m".to_string()),
                height_exponent: azgaar_data.settings.height_exponent.unwrap_or(1.8),
                temperature_scale: azgaar_data.settings.temperature_scale.unwrap_or_else(|| "°C".to_string()),
                population_rate: azgaar_data.settings.population_rate.unwrap_or(1.0),
                urbanization: azgaar_data.settings.urbanization.unwrap_or(1.0),
                latitude: azgaar_data.settings.latitude.unwrap_or(50.0),
                longitude: azgaar_data.settings.longitude.unwrap_or(0.0),
            },
        };
        
        // Create empty heightmap for now
        let heightmap = Grid::new(azgaar_data.info.width, azgaar_data.info.height, 0.0f32);
        
        // Convert cells if available
        let cells = if let Some(pack_cells) = azgaar_data.pack.and_then(|p| p.cells) {
            self.convert_pack_cells(pack_cells)?
        } else {
            Vec::new()
        };
        
        Ok(WorldMap {
            metadata,
            heightmap,
            cells,
            features: Vec::new(), // TODO: Convert features
            cultures: Vec::new(), // TODO: Convert cultures
            states: Vec::new(),   // TODO: Convert states
            burgs: Vec::new(),    // TODO: Convert burgs
            rivers: Vec::new(),   // TODO: Convert rivers
            routes: Vec::new(),   // TODO: Convert routes
            markers: Vec::new(),  // TODO: Convert markers
            zones: Vec::new(),    // TODO: Convert zones
        })
    }
    
    fn convert_pack_cells(&self, pack_cells: AzgaarPackCells) -> Result<Vec<crate::data::Cell>> {
        use crate::data::{Cell, BiomeType};
        use nalgebra::Point2;
        
        let mut cells = Vec::new();
        
        // Azgaar stores cell data in parallel arrays
        for i in 0..pack_cells.i.len() {
            let cell = Cell {
                id: pack_cells.i[i],
                coordinates: Point2::new(
                    pack_cells.p.get(i * 2).copied().unwrap_or(0.0),
                    pack_cells.p.get(i * 2 + 1).copied().unwrap_or(0.0),
                ),
                height: pack_cells.h.get(i).copied().unwrap_or(0.0),
                biome: BiomeType::Temperate, // TODO: Convert biome data
                temperature: pack_cells.temp.get(i).copied().unwrap_or(0.0),
                precipitation: pack_cells.prec.get(i).copied().unwrap_or(0.0),
                population: pack_cells.pop.get(i).copied().unwrap_or(0),
                culture: pack_cells.culture.get(i).and_then(|&x| x),
                state: pack_cells.state.get(i).and_then(|&x| x),
                province: pack_cells.province.get(i).and_then(|&x| x),
                religion: pack_cells.religion.get(i).and_then(|&x| x),
            };
            cells.push(cell);
        }
        
        Ok(cells)
    }
}

/// Azgaar JSON data structures
#[derive(Debug, Deserialize, Serialize)]
struct AzgaarJsonData {
    info: AzgaarInfo,
    settings: AzgaarSettings,
    pack: Option<AzgaarPack>,
    grid: Option<AzgaarGrid>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AzgaarInfo {
    version: String,
    #[serde(rename = "mapName")]
    map_name: Option<String>,
    width: u32,
    height: u32,
    seed: u64,
    #[serde(rename = "mapId")]
    map_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AzgaarSettings {
    #[serde(rename = "distanceUnit")]
    distance_unit: Option<String>,
    #[serde(rename = "distanceScale")]
    distance_scale: Option<f32>,
    #[serde(rename = "areaUnit")]
    area_unit: Option<String>,
    #[serde(rename = "heightUnit")]
    height_unit: Option<String>,
    #[serde(rename = "heightExponent")]
    height_exponent: Option<f32>,
    #[serde(rename = "temperatureScale")]
    temperature_scale: Option<String>,
    #[serde(rename = "populationRate")]
    population_rate: Option<f32>,
    urbanization: Option<f32>,
    latitude: Option<f32>,
    longitude: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AzgaarPack {
    cells: Option<AzgaarPackCells>,
    features: Option<serde_json::Value>,
    cultures: Option<serde_json::Value>,
    burgs: Option<serde_json::Value>,
    states: Option<serde_json::Value>,
    provinces: Option<serde_json::Value>,
    religions: Option<serde_json::Value>,
    rivers: Option<serde_json::Value>,
    markers: Option<serde_json::Value>,
    routes: Option<serde_json::Value>,
    zones: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AzgaarPackCells {
    i: Vec<u32>,        // cell ids
    p: Vec<f32>,        // coordinates (x,y pairs)
    h: Vec<f32>,        // heights
    temp: Vec<f32>,     // temperatures
    prec: Vec<f32>,     // precipitation
    pop: Vec<u32>,      // population
    culture: Vec<Option<u32>>,
    state: Vec<Option<u32>>,
    province: Vec<Option<u32>>,
    religion: Vec<Option<u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AzgaarGrid {
    // Grid cell data structure
    // TODO: Define based on Azgaar's grid format
}