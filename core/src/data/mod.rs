//! Core data structures for World Foundry

use serde::{Deserialize, Serialize};
use nalgebra::Point2;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Main world map data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub metadata: MapMetadata,
    pub heightmap: Grid<f32>,
    pub cells: Vec<Cell>,
    pub features: Vec<Feature>,
    pub cultures: Vec<Culture>,
    pub states: Vec<State>,
    pub burgs: Vec<Settlement>,
    pub rivers: Vec<River>,
    pub routes: Vec<Route>,
    pub markers: Vec<Marker>,
    pub zones: Vec<Zone>,
}

/// Map metadata and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMetadata {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    pub settings: MapSettings,
}

/// Map generation and display settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapSettings {
    pub distance_unit: String,
    pub distance_scale: f32,
    pub area_unit: String,
    pub height_unit: String,
    pub height_exponent: f32,
    pub temperature_scale: String,
    pub population_rate: f32,
    pub urbanization: f32,
    pub latitude: f32,
    pub longitude: f32,
}

/// Regular grid for heightmaps and other raster data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid<T> {
    pub width: u32,
    pub height: u32,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: u32, height: u32, default_value: T) -> Self 
    where 
        T: Clone 
    {
        Self {
            width,
            height,
            data: vec![default_value; (width * height) as usize],
        }
    }
    
    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        if x < self.width && y < self.height {
            self.data.get((y * self.width + x) as usize)
        } else {
            None
        }
    }
    
    pub fn set(&mut self, x: u32, y: u32, value: T) -> bool {
        if x < self.width && y < self.height {
            if let Some(cell) = self.data.get_mut((y * self.width + x) as usize) {
                *cell = value;
                return true;
            }
        }
        false
    }
}

/// Individual map cell (Voronoi cell or grid cell)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub id: u32,
    pub coordinates: Point2<f32>,
    pub height: f32,
    pub biome: BiomeType,
    pub temperature: f32,
    pub precipitation: f32,
    pub population: u32,
    pub culture: Option<u32>,
    pub state: Option<u32>,
    pub province: Option<u32>,
    pub religion: Option<u32>,
}

/// Biome types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BiomeType {
    Marine,
    Hot,
    Cold,
    Temperate,
    Dry,
    Frozen,
    Freshwater,
    // Add more biome types as needed
}

/// Geographic features (mountains, forests, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: u32,
    pub name: String,
    pub feature_type: FeatureType,
    pub cells: Vec<u32>,
    pub group: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureType {
    Mountain,
    Hill,
    Forest,
    Desert,
    Swamp,
    Lake,
    Island,
}

/// Cultural groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub base: u32,
    pub origins: Vec<u32>,
    pub shield: String,
    pub center: Point2<f32>,
    pub area: f32,
    pub rural: f32,
    pub urban: f32,
    pub expansionism: f32,
}

/// Political states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: u32,
    pub name: String,
    pub full_name: String,
    pub color: String,
    pub capital: u32,
    pub center: Point2<f32>,
    pub area: f32,
    pub population: u32,
    pub rural: f32,
    pub urban: f32,
    pub burgs: u32,
    pub culture: u32,
    pub type_: String,
    pub expansionism: f32,
    pub cells: Vec<u32>,
}

/// Settlements (cities, towns, villages)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub id: u32,
    pub name: String,
    pub cell: u32,
    pub x: f32,
    pub y: f32,
    pub state: u32,
    pub i: u32,
    pub culture: u32,
    pub feature: u32,
    pub capital: u32,
    pub port: u32,
    pub population: f32,
    pub type_: String,
}

/// Rivers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct River {
    pub id: u32,
    pub source: u32,
    pub mouth: u32,
    pub discharge: f32,
    pub length: f32,
    pub width: f32,
    pub cells: Vec<u32>,
    pub basin: u32,
    pub name: String,
    pub type_: String,
}

/// Trade routes and roads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: u32,
    pub group: u32,
    pub cells: Vec<u32>,
    pub feature: u32,
    pub length: f32,
}

/// Map markers and labels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub id: u32,
    pub icon: String,
    pub type_: String,
    pub dx: f32,
    pub dy: f32,
    pub x: f32,
    pub y: f32,
    pub cell: u32,
    pub i: u32,
    pub size: f32,
    pub fill: String,
    pub stroke: String,
    pub note: String,
}

/// Zones (climate, political, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub id: u32,
    pub name: String,
    pub cells: Vec<u32>,
    pub color: String,
}