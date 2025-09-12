//! World Foundry Core Engine
//! 
//! A high-performance, cross-platform fantasy map generator core engine
//! that provides world generation, rendering, and import/export capabilities.

pub mod data;
pub mod generation;
pub mod rendering;
pub mod import;
pub mod export;
pub mod platform;

pub use data::*;
pub use generation::*;
pub use rendering::*;
pub use import::*;
pub use export::*;
pub use platform::*;

/// Core result type for World Foundry operations
pub type Result<T> = std::result::Result<T, WorldFoundryError>;

/// Main error type for World Foundry operations
#[derive(thiserror::Error, Debug)]
pub enum WorldFoundryError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Import error: {0}")]
    Import(String),
    
    #[error("Export error: {0}")]
    Export(String),
    
    #[error("Rendering error: {0}")]
    Rendering(String),
    
    #[error("Generation error: {0}")]
    Generation(String),
    
    #[error("Platform error: {0}")]
    Platform(String),
}

/// Initialize the World Foundry core engine
pub fn initialize() -> Result<()> {
    // Initialize logging, graphics context, etc.
    Ok(())
}

/// Get the version of the World Foundry core engine
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}