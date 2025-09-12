# World Foundry Core API Reference

## Overview

The World Foundry Core API provides comprehensive access to the fantasy world generation and management system. This API is implemented in Rust and exposed through Foreign Function Interface (FFI) bindings for integration with platform-specific applications on Windows, iOS, and Android.

## API Architecture

### Core Components

```
🔧 Core API Architecture
├── 🌍 World Management API
│   ├── World Creation and Loading
│   ├── World Serialization and Storage
│   ├── World Metadata Management
│   └── World Validation and Integrity
├── 🗺️ Geographic API
│   ├── Map Generation and Manipulation
│   ├── Terrain and Biome Management
│   ├── Climate and Weather Systems
│   └── Resource Distribution
├── 👥 Cultural API
│   ├── Culture Creation and Management
│   ├── Language and Namebase Systems
│   ├── Religion and Belief Systems
│   └── Historical Timeline Management
├── 🏛️ Political API
│   ├── State and Government Management
│   ├── Diplomatic Relations
│   ├── Territory and Border Management
│   └── Administrative Systems
├── 🏘️ Settlement API
│   ├── Settlement Creation and Management
│   ├── Population and Demographics
│   ├── Economic Systems
│   └── Infrastructure Management
├── 🎨 Rendering API
│   ├── Map Visualization
│   ├── Cultural Overlays
│   ├── Political Boundaries
│   └── Custom Styling
├── 📥 Import/Export API
│   ├── Azgaar Map Import
│   ├── Generic Data Import
│   ├── Multiple Export Formats
│   └── Batch Processing
└── 🔧 Utility API
    ├── Coordinate Systems
    ├── Mathematical Operations
    ├── Random Generation
    └── Performance Monitoring
```

## Core Data Types

### Basic Types

```rust
// Core identifier types
pub type WorldId = uuid::Uuid;
pub type CultureId = uuid::Uuid;
pub type ReligionId = uuid::Uuid;
pub type StateId = uuid::Uuid;
pub type SettlementId = uuid::Uuid;
pub type LanguageId = uuid::Uuid;

// Coordinate system
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}

// Color representation
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Error handling
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    Success = 0,
    InvalidInput = 1,
    NotFound = 2,
    AlreadyExists = 3,
    InternalError = 4,
    SerializationError = 5,
    ValidationError = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
}

// Result type for FFI
#[repr(C)]
pub struct ApiResult<T> {
    pub error_code: ApiError,
    pub data: Option<T>,
    pub error_message: *const c_char,
}
```

## World Management API

### Core World Functions

```rust
/// Initialize the World Foundry engine
/// Returns: Engine handle for subsequent operations
#[no_mangle]
pub extern "C" fn wf_engine_create() -> *mut c_void;

/// Destroy the engine and free resources
/// Parameters:
///   - engine: Engine handle from wf_engine_create
#[no_mangle]
pub extern "C" fn wf_engine_destroy(engine: *mut c_void);

/// Create a new world with specified configuration
/// Parameters:
///   - engine: Engine handle
///   - config: World configuration parameters
///   - world_id: Output parameter for new world ID
/// Returns: ApiError code
#[no_mangle]
pub extern "C" fn wf_world_create(
    engine: *mut c_void,
    config: *const WorldConfig,
    world_id: *mut WorldId,
) -> ApiError;

/// Load an existing world from storage
/// Parameters:
///   - engine: Engine handle
///   - world_id: ID of world to load
/// Returns: ApiError code
#[no_mangle]
pub extern "C" fn wf_world_load(
    engine: *mut c_void,
    world_id: WorldId,
) -> ApiError;

/// Save the current world to storage
/// Parameters:
///   - engine: Engine handle
/// Returns: ApiError code
#[no_mangle]
pub extern "C" fn wf_world_save(engine: *mut c_void) -> ApiError;
```

## Import/Export API

### Azgaar Import

```rust
/// Import a world from Azgaar's Fantasy Map Generator
/// Parameters:
///   - engine: Engine handle
///   - file_path: Path to the Azgaar .map or .json file
///   - import_options: Import configuration options
/// Returns: ApiError code
#[no_mangle]
pub extern "C" fn wf_import_azgaar(
    engine: *mut c_void,
    file_path: *const c_char,
    import_options: *const AzgaarImportOptions,
) -> ApiError;
```

## Usage Examples

### Basic World Creation

```c
// C example
#include "world_foundry.h"

int main() {
    // Create engine
    void* engine = wf_engine_create();
    if (!engine) {
        printf("Failed to create engine\n");
        return 1;
    }
    
    // Create world
    WorldId world_id;
    ApiError result = wf_world_create(engine, &config, &world_id);
    if (result != Success) {
        printf("Failed to create world: %d\n", result);
        wf_engine_destroy(engine);
        return 1;
    }
    
    // Save world
    result = wf_world_save(engine);
    if (result != Success) {
        printf("Failed to save world: %d\n", result);
    }
    
    // Cleanup
    wf_engine_destroy(engine);
    return 0;
}
```

---

**Last Updated**: March 2024  
**API Version**: 1.0.0  
**Core Engine Version**: 1.0.0  
**Compatibility**: World Foundry 1.0+