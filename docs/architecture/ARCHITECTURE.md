# World Foundry Architecture

## Overview

World Foundry is designed as a cross-platform fantasy map generator with a shared core engine written in Rust and platform-specific UI implementations. This architecture ensures consistent world generation and rendering across all supported platforms while providing native user experiences.

## Core Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Platform Layer                           │
├─────────────────┬─────────────────┬─────────────────────────┤
│   Windows UI    │    iOS UI       │    Android UI           │
│   (WinUI 3)     │   (SwiftUI)     │  (Jetpack Compose)      │
├─────────────────┼─────────────────┼─────────────────────────┤
│                 Platform Abstraction Layer                 │
├─────────────────────────────────────────────────────────────┤
│                    Core Engine (Rust)                      │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┐  │
│  │    Data     │ Generation  │  Rendering  │   Import/   │  │
│  │ Structures  │   Engine    │   Engine    │   Export    │  │
│  └─────────────┴─────────────┴─────────────┴─────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Core Engine Components

### 1. Data Structures (`core/src/data/`)

The core data module defines the fundamental structures for representing fantasy worlds:

- **WorldMap**: Main container for all world data
- **Grid<T>**: Generic grid structure for heightmaps and raster data
- **Cell**: Individual map cells (Voronoi or grid-based)
- **Feature**: Geographic features (mountains, forests, etc.)
- **Culture**: Cultural groups and their properties
- **State**: Political entities and boundaries
- **Settlement**: Cities, towns, and villages
- **River**: Water bodies and flow systems
- **Route**: Trade routes and roads
- **Marker**: Map annotations and labels

### 2. Generation Engine (`core/src/generation/`)

Procedural world generation system:

- **Heightmap Generation**: Noise-based terrain generation
- **Climate Simulation**: Temperature and precipitation modeling
- **Biome Assignment**: Climate-based ecosystem distribution
- **Cultural Development**: Civilization spread and interaction
- **Political Formation**: State boundaries and relationships
- **Settlement Placement**: City and town distribution
- **River Networks**: Realistic water flow systems

### 3. Rendering Engine (`core/src/rendering/`)

High-performance map visualization using Skia:

- **Layer System**: Composable rendering layers
- **Style Engine**: Configurable visual styles
- **Zoom/Pan Support**: Multi-scale rendering
- **Export Capabilities**: Multiple output formats
- **Performance Optimization**: GPU acceleration where available

### 4. Import/Export System (`core/src/import/`, `core/src/export/`)

Format compatibility and data exchange:

- **Azgaar Importer**: Native support for Azgaar Fantasy Map Generator
- **JSON Format**: World Foundry native format
- **Image Export**: PNG, SVG rendering
- **GeoJSON Export**: GIS software compatibility
- **Extensible Architecture**: Plugin system for additional formats

### 5. Platform Abstraction (`core/src/platform/`)

Device-specific functionality abstraction:

- **Storage Interface**: File system operations
- **UI Interface**: Native dialogs and interactions
- **Graphics Interface**: Hardware-accelerated rendering
- **Capabilities Detection**: Platform feature discovery

## Platform-Specific Implementations

### Windows (`platforms/windows/`)

- **Technology**: WinUI 3 with C++/WinRT
- **Graphics**: DirectX 11/12 or OpenGL
- **Storage**: Windows file system APIs
- **Distribution**: Microsoft Store or standalone installer

### iOS (`platforms/ios/`)

- **Technology**: SwiftUI with Swift
- **Graphics**: Metal rendering
- **Storage**: iOS sandboxed file system
- **Distribution**: App Store

### Android (`platforms/android/`)

- **Technology**: Jetpack Compose with Kotlin
- **Graphics**: Vulkan or OpenGL ES
- **Storage**: Android scoped storage
- **Distribution**: Google Play Store

## Data Flow

### Import Process

1. **File Detection**: Platform file picker selects input file
2. **Format Recognition**: Import registry identifies file type
3. **Data Parsing**: Format-specific importer processes file
4. **Data Conversion**: External format converted to WorldMap
5. **Validation**: Data integrity checks
6. **Storage**: Converted data saved in native format

### Generation Process

1. **Parameter Setup**: User configures generation parameters
2. **Heightmap Creation**: Noise-based terrain generation
3. **Climate Calculation**: Temperature/precipitation modeling
4. **Biome Assignment**: Climate-based ecosystem placement
5. **Cultural Simulation**: Civilization development
6. **Political Formation**: State and province boundaries
7. **Detail Generation**: Settlements, rivers, routes
8. **Finalization**: Complete WorldMap creation

### Rendering Process

1. **Configuration**: Render settings and style selection
2. **Layer Preparation**: Individual layer data preparation
3. **Graphics Context**: Platform-specific rendering setup
4. **Layer Composition**: Z-ordered layer rendering
5. **Post-Processing**: Effects and final adjustments
6. **Output**: Image generation or display

## Performance Considerations

### Memory Management

- **Streaming**: Large maps loaded in chunks
- **Caching**: Frequently accessed data cached
- **Compression**: Efficient data storage formats
- **Garbage Collection**: Minimal allocation patterns

### Rendering Optimization

- **Level of Detail**: Scale-appropriate rendering
- **Culling**: Off-screen content skipping
- **Batching**: Efficient draw call grouping
- **GPU Utilization**: Hardware acceleration

### Cross-Platform Performance

- **Native Compilation**: Rust's zero-cost abstractions
- **Platform APIs**: Direct system integration
- **Memory Efficiency**: Shared core engine
- **Threading**: Async operations where beneficial

## Security and Privacy

### Data Protection

- **Local Storage**: No cloud dependency required
- **Encryption**: Optional file encryption
- **Sandboxing**: Platform security compliance
- **Permissions**: Minimal required permissions

### Code Security

- **Memory Safety**: Rust's ownership system
- **Input Validation**: Robust parsing and validation
- **Error Handling**: Comprehensive error management
- **Dependency Auditing**: Regular security updates

## Extensibility

### Plugin Architecture

- **Import Plugins**: Additional file format support
- **Export Plugins**: New output formats
- **Generation Plugins**: Custom world generation algorithms
- **Rendering Plugins**: Additional visualization styles

### API Design

- **Stable Interfaces**: Backward compatibility
- **Documentation**: Comprehensive API docs
- **Examples**: Reference implementations
- **Testing**: Extensive test coverage

## Development Workflow

### Build System

- **Cargo**: Rust package management
- **Platform Tools**: Native build systems
- **CI/CD**: Automated testing and deployment
- **Cross-Compilation**: Multi-platform builds

### Testing Strategy

- **Unit Tests**: Core functionality testing
- **Integration Tests**: Cross-component testing
- **Platform Tests**: Device-specific testing
- **Performance Tests**: Benchmark validation

### Release Process

- **Versioning**: Semantic versioning
- **Changelog**: Detailed release notes
- **Distribution**: Platform-specific stores
- **Updates**: Automatic update mechanisms