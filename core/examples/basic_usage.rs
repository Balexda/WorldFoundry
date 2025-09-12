//! Basic usage example for World Foundry

use world_foundry_core::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the World Foundry engine
    world_foundry_core::initialize()?;
    
    println!("World Foundry Core Engine v{}", world_foundry_core::version());
    
    // Example 1: Generate a new world
    println!("\n=== Generating New World ===");
    
    let generation_params = GenerationParams {
        seed: 42,
        width: 1024,
        height: 512,
        ..Default::default()
    };
    
    let generator = WorldGenerator::new(generation_params);
    
    // Note: This will fail until generation is implemented
    match generator.generate() {
        Ok(world_map) => {
            println!("Generated world: {}", world_map.metadata.name);
            println!("Dimensions: {}x{}", world_map.metadata.width, world_map.metadata.height);
            println!("Cells: {}", world_map.cells.len());
        }
        Err(e) => {
            println!("Generation not yet implemented: {}", e);
        }
    }
    
    // Example 2: Import from Azgaar
    println!("\n=== Importing from Azgaar ===");
    
    let import_registry = ImporterRegistry::new();
    
    // This would work with an actual Azgaar file
    let test_file = Path::new("test_map.json");
    if test_file.exists() {
        match import_registry.import(test_file) {
            Ok(world_map) => {
                println!("Imported world: {}", world_map.metadata.name);
                println!("Dimensions: {}x{}", world_map.metadata.width, world_map.metadata.height);
            }
            Err(e) => {
                println!("Import failed: {}", e);
            }
        }
    } else {
        println!("No test file found at: {}", test_file.display());
    }
    
    // Example 3: Create a simple world manually
    println!("\n=== Creating Simple World ===");
    
    let world_map = create_simple_world();
    println!("Created simple world: {}", world_map.metadata.name);
    
    // Example 4: Export to different formats
    println!("\n=== Exporting World ===");
    
    let export_registry = ExporterRegistry::new();
    
    // Export to JSON
    let json_path = Path::new("example_world.json");
    match export_registry.export(&world_map, json_path) {
        Ok(()) => println!("Exported to JSON: {}", json_path.display()),
        Err(e) => println!("JSON export failed: {}", e),
    }
    
    // Export to PNG
    let png_path = Path::new("example_world.png");
    match export_registry.export(&world_map, png_path) {
        Ok(()) => println!("Exported to PNG: {}", png_path.display()),
        Err(e) => println!("PNG export failed: {}", e),
    }
    
    // Example 5: Rendering
    println!("\n=== Rendering World ===");
    
    let render_config = RenderConfig {
        width: 800,
        height: 600,
        style: RenderStyle::Political,
        ..Default::default()
    };
    
    let mut renderer = WorldRenderer::new();
    match renderer.render(&world_map, &render_config) {
        Ok(()) => {
            match renderer.get_image_bytes() {
                Ok(image_data) => {
                    println!("Rendered world to {} bytes", image_data.len());
                    
                    // Save the rendered image
                    if let Err(e) = std::fs::write("rendered_world.png", image_data) {
                        println!("Failed to save rendered image: {}", e);
                    } else {
                        println!("Saved rendered image to: rendered_world.png");
                    }
                }
                Err(e) => println!("Failed to get image bytes: {}", e),
            }
        }
        Err(e) => println!("Rendering failed: {}", e),
    }
    
    // Example 6: Platform capabilities
    println!("\n=== Platform Information ===");
    
    match PlatformFactory::create() {
        Ok(platform) => {
            let capabilities = platform.capabilities();
            println!("Platform: {}", capabilities.platform_name);
            println!("Version: {}", capabilities.version);
            println!("Graphics APIs: {:?}", capabilities.graphics_apis);
            println!("Max texture size: {}", capabilities.max_texture_size);
        }
        Err(e) => {
            println!("Platform detection failed: {}", e);
        }
    }
    
    println!("\n=== Example Complete ===");
    Ok(())
}

/// Create a simple world for demonstration
fn create_simple_world() -> WorldMap {
    use nalgebra::Point2;
    use uuid::Uuid;
    use chrono::Utc;
    
    let metadata = MapMetadata {
        id: Uuid::new_v4(),
        name: "Example World".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        modified_at: Utc::now(),
        width: 100,
        height: 100,
        seed: 12345,
        settings: MapSettings {
            distance_unit: "km".to_string(),
            distance_scale: 1.0,
            area_unit: "km²".to_string(),
            height_unit: "m".to_string(),
            height_exponent: 1.8,
            temperature_scale: "°C".to_string(),
            population_rate: 1.0,
            urbanization: 1.0,
            latitude: 45.0,
            longitude: 0.0,
        },
    };
    
    // Create a simple heightmap
    let mut heightmap = Grid::new(100, 100, 0.0f32);
    
    // Add some simple terrain
    for y in 0..100 {
        for x in 0..100 {
            let height = ((x as f32 - 50.0).powi(2) + (y as f32 - 50.0).powi(2)).sqrt() / 50.0;
            let height = (1.0 - height).max(0.0);
            heightmap.set(x, y, height);
        }
    }
    
    // Create some sample cells
    let mut cells = Vec::new();
    for i in 0..10 {
        let cell = Cell {
            id: i,
            coordinates: Point2::new(i as f32 * 10.0, i as f32 * 10.0),
            height: 0.5,
            biome: BiomeType::Temperate,
            temperature: 15.0,
            precipitation: 800.0,
            population: 1000,
            culture: Some(0),
            state: Some(0),
            province: None,
            religion: None,
        };
        cells.push(cell);
    }
    
    // Create a sample culture
    let culture = Culture {
        id: 0,
        name: "Example Culture".to_string(),
        color: "#FF0000".to_string(),
        base: 0,
        origins: vec![0],
        shield: "default".to_string(),
        center: Point2::new(50.0, 50.0),
        area: 1000.0,
        rural: 0.8,
        urban: 0.2,
        expansionism: 1.0,
    };
    
    // Create a sample state
    let state = State {
        id: 0,
        name: "Example Kingdom".to_string(),
        full_name: "The Kingdom of Example".to_string(),
        color: "#0000FF".to_string(),
        capital: 0,
        center: Point2::new(50.0, 50.0),
        area: 1000.0,
        population: 10000,
        rural: 0.7,
        urban: 0.3,
        burgs: 1,
        culture: 0,
        type_: "Kingdom".to_string(),
        expansionism: 1.0,
        cells: (0..10).collect(),
    };
    
    // Create a sample settlement
    let settlement = Settlement {
        id: 0,
        name: "Example City".to_string(),
        cell: 0,
        x: 50.0,
        y: 50.0,
        state: 0,
        i: 0,
        culture: 0,
        feature: 0,
        capital: 1,
        port: 0,
        population: 5000.0,
        type_: "City".to_string(),
    };
    
    WorldMap {
        metadata,
        heightmap,
        cells,
        features: Vec::new(),
        cultures: vec![culture],
        states: vec![state],
        burgs: vec![settlement],
        rivers: Vec::new(),
        routes: Vec::new(),
        markers: Vec::new(),
        zones: Vec::new(),
    }
}