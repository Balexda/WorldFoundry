# World Foundry
## Cross-Platform Fantasy Map Generator - Project Proposal

### Executive Summary

This project aims to create a high-performance, cross-platform fantasy map generator that can import maps from Azgaar's Fantasy Map Generator while providing a superior user experience on Windows, iPad, and Android devices. The system will feature a shared world generation engine, device-agnostic rendering, and platform-specific UI/storage implementations.

### Problem Statement

Azgaar's Fantasy Map Generator, while feature-rich, suffers from:
- Poor performance on complex maps
- Web-based UI limitations
- Lack of native mobile support
- No offline capabilities
- Limited touch interface optimization

### Project Goals

1. **Performance**: Native performance significantly better than web-based solutions
2. **Cross-Platform**: Single codebase supporting Windows, iPad, and Android
3. **Compatibility**: Full import support for Azgaar .map and JSON formats
4. **User Experience**: Intuitive, touch-optimized interfaces for each platform
5. **Offline Capability**: Full functionality without internet connection
6. **Extensibility**: Modular architecture for future enhancements

### Technical Architecture

#### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────┬─────────────────┬─────────────────────────┤
│   Windows UI    │    iPad UI      │     Android UI          │
├─────────────────┼─────────────────┼─────────────────────────┤
│              Platform Abstraction Layer                    │
├─────────────────────────────────────────────────────────────┤
│                 Shared Core Engine                          │
│  ┌─────────────┬─────────────────┬─────────────────────────┐ │
│  │   World     │    Rendering    │    Import/Export        │ │
│  │ Generation  │     Engine      │       System            │ │
│  └─────────────┴─────────────────┴─────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

#### 1. Shared Core Engine

**World Generation Module**
- Heightmap generation and manipulation
- Biome assignment and climate simulation
- River and lake generation
- Political boundary creation
- Settlement placement and sizing
- Culture and religion generation
- Military unit placement

**Rendering Engine**
- Vector-based map rendering
- Multi-layer composition (terrain, political, labels, etc.)
- Zoom-level optimization
- Texture and pattern management
- Symbol and icon rendering
- Performance-optimized drawing pipelines

**Import/Export System**
- Azgaar .map file parser
- Azgaar JSON format support
- GeoJSON export capabilities
- Native format optimization
- Batch processing support

#### 2. Platform Abstraction Layer

**Storage Interface**
```typescript
interface IStorageProvider {
  saveMap(mapData: MapData, filename: string): Promise<void>;
  loadMap(filename: string): Promise<MapData>;
  listMaps(): Promise<string[]>;
  deleteMap(filename: string): Promise<void>;
  exportMap(mapData: MapData, format: ExportFormat): Promise<Blob>;
}
```

**UI Interface**
```typescript
interface IUIProvider {
  showDialog(dialog: DialogConfig): Promise<any>;
  showToast(message: string, type: ToastType): void;
  updateProgress(progress: number): void;
  requestFileAccess(): Promise<FileHandle>;
}
```

#### 3. Platform-Specific Implementations

**Windows (WinUI 3 + .NET)**
- Native file system access
- Windows-style menus and dialogs
- Keyboard shortcuts
- Multi-window support
- High-DPI awareness

**iPad (SwiftUI)**
- Touch-optimized gestures
- Apple Pencil support
- Files app integration
- Split-screen multitasking
- Context menus and toolbars

**Android (Jetpack Compose)**
- Material Design 3
- Scoped storage compliance
- Adaptive layouts for tablets
- Gesture navigation
- Share sheet integration

### Technology Stack

#### Core Engine (Shared)
- **Language**: Rust with C FFI bindings
- **Graphics**: Skia for cross-platform 2D rendering
- **Math**: nalgebra for linear algebra operations
- **Serialization**: serde for data handling
- **Async**: tokio for async operations

#### Platform Bindings
- **Windows**: Rust + WinRT bindings
- **iOS/iPad**: Swift with Rust static library
- **Android**: Kotlin with JNI bindings

#### Alternative Approach: Flutter
- **Framework**: Flutter 3.x
- **Language**: Dart with Rust FFI for core engine
- **Benefits**: Single UI codebase, excellent performance
- **Considerations**: Larger app size, platform integration limitations

### Data Structures

#### Core Map Data
```rust
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
```

#### Azgaar Import Format
```rust
pub struct AzgaarMap {
    pub version: String,
    pub info: MapInfo,
    pub settings: MapSettings,
    pub pack: PackData,
    pub grid: Option<GridData>,
    pub biomes_data: BiomesData,
    pub notes: Vec<Note>,
    pub name_bases: NameBases,
}
```

### Import System Design

#### Azgaar Format Support

**Supported Formats:**
1. `.map` files (custom binary format)
2. JSON exports (Full, Minimal, PackCells, GridCells)
3. GeoJSON cell data
4. PNG heightmaps with metadata

**Import Pipeline:**
```rust
pub trait MapImporter {
    fn can_import(&self, file_path: &Path) -> bool;
    fn import(&self, file_path: &Path) -> Result<WorldMap, ImportError>;
    fn get_preview(&self, file_path: &Path) -> Result<MapPreview, ImportError>;
}

pub struct AzgaarImporter;
impl MapImporter for AzgaarImporter {
    // Implementation details
}
```

**Data Conversion:**
- Voronoi cell data → Regular grid interpolation
- Azgaar coordinate system → Normalized coordinates
- Culture/religion mapping → Internal ID system
- SVG paths → Vector geometry

### Rendering Architecture

#### Multi-Layer Rendering
```rust
pub enum LayerType {
    Heightmap,
    Biomes,
    Temperature,
    Precipitation,
    Political,
    Cultures,
    Religions,
    Rivers,
    Routes,
    Settlements,
    Labels,
    Markers,
    Grid,
}

pub struct RenderLayer {
    pub layer_type: LayerType,
    pub visible: bool,
    pub opacity: f32,
    pub render_fn: Box<dyn Fn(&WorldMap, &RenderContext) -> Surface>,
}
```

#### Performance Optimizations
- **Level-of-Detail (LOD)**: Different rendering quality based on zoom level
- **Tile-based Rendering**: Divide map into tiles for efficient updates
- **Caching**: Cache rendered tiles and reuse when possible
- **Background Processing**: Generate tiles on background threads
- **GPU Acceleration**: Use platform-specific GPU APIs where available

### User Interface Design

#### Core UI Components
- **Map Viewport**: Pan, zoom, layer toggle
- **Tool Palette**: Selection, editing, measurement tools
- **Property Panels**: Context-sensitive editing panels
- **Layer Manager**: Show/hide and reorder layers
- **Import/Export Dialogs**: File management interfaces

#### Platform-Specific Adaptations

**Windows**
- Traditional menu bar and toolbars
- Dockable panels
- Keyboard shortcuts (Ctrl+O, Ctrl+S, etc.)
- Context menus
- Status bar with coordinates and zoom level

**iPad**
- Touch-optimized controls
- Gesture-based navigation (pinch to zoom, two-finger pan)
- Floating panels that can be dismissed
- Apple Pencil integration for precise editing
- Sidebar navigation

**Android**
- Material Design components
- Bottom navigation or navigation drawer
- Floating action buttons
- Adaptive layouts for different screen sizes
- Gesture navigation support

### Development Phases

#### Phase 1: Core Engine (8-10 weeks)
- [ ] Set up Rust project structure
- [ ] Implement basic data structures
- [ ] Create Azgaar import system
- [ ] Build basic rendering pipeline
- [ ] Add unit tests and documentation

#### Phase 2: Platform Foundations (6-8 weeks)
- [ ] Windows application shell
- [ ] iPad application shell
- [ ] Android application shell
- [ ] Platform abstraction layer
- [ ] Basic UI for each platform

#### Phase 3: Feature Implementation (10-12 weeks)
- [ ] Map viewing and navigation
- [ ] Layer management
- [ ] Import/export functionality
- [ ] Basic editing tools
- [ ] Performance optimization

#### Phase 4: Polish and Testing (4-6 weeks)
- [ ] UI/UX refinements
- [ ] Cross-platform testing
- [ ] Performance profiling
- [ ] Bug fixes and stability
- [ ] Documentation and tutorials

### Technical Challenges and Solutions

#### Challenge 1: Cross-Platform Rendering
**Problem**: Consistent rendering across different platforms and graphics APIs
**Solution**: Use Skia as the common rendering backend with platform-specific surface creation

#### Challenge 2: Large Dataset Performance
**Problem**: Azgaar maps can have hundreds of thousands of cells
**Solution**: Implement spatial indexing, level-of-detail rendering, and background processing

#### Challenge 3: Touch Interface Design
**Problem**: Complex map editing on touch devices
**Solution**: Design touch-first interactions with gesture recognition and context-sensitive tools

#### Challenge 4: File Format Compatibility
**Problem**: Azgaar's format may change over time
**Solution**: Version-aware import system with backward compatibility and format migration

### Success Metrics

#### Performance Targets
- **Startup Time**: < 3 seconds on target devices
- **Import Time**: < 30 seconds for large Azgaar maps
- **Rendering**: 60 FPS at common zoom levels
- **Memory Usage**: < 500MB for typical maps

#### User Experience Goals
- **Learning Curve**: New users productive within 15 minutes
- **Feature Parity**: 90% of common Azgaar features supported
- **Stability**: < 1 crash per 100 hours of usage
- **Cross-Platform**: Consistent experience across all platforms

### Risk Assessment

#### High Risk
- **Azgaar Format Changes**: Mitigation through version detection and community engagement
- **Platform API Changes**: Regular updates and platform-specific testing
- **Performance on Mobile**: Early prototyping and optimization focus

#### Medium Risk
- **Development Complexity**: Phased approach and regular milestone reviews
- **User Adoption**: Beta testing program and community feedback
- **Maintenance Burden**: Automated testing and CI/CD pipelines

#### Low Risk
- **Technology Maturity**: Rust and chosen platforms are stable
- **Market Demand**: Clear user need demonstrated by Azgaar's popularity

### Resource Requirements

#### Development Team
- **1 Senior Rust Developer**: Core engine and architecture
- **1 Windows Developer**: WinUI implementation
- **1 iOS Developer**: SwiftUI implementation  
- **1 Android Developer**: Jetpack Compose implementation
- **1 UI/UX Designer**: Cross-platform design system
- **1 QA Engineer**: Testing and validation

#### Timeline
- **Total Duration**: 28-36 weeks
- **MVP Release**: 20 weeks
- **Full Feature Release**: 36 weeks

#### Budget Considerations
- Development tools and licenses
- Device testing hardware
- App store registration fees
- Continuous integration services

### Future Enhancements

#### Version 2.0 Features
- **Collaborative Editing**: Real-time multi-user editing
- **Cloud Sync**: Cross-device synchronization
- **Plugin System**: Third-party extensions
- **Advanced Generation**: AI-assisted world building
- **3D Visualization**: Height-based 3D rendering

#### Long-term Vision
- **World Building Suite**: Expand beyond maps to full campaign management
- **Community Features**: Map sharing and collaboration platform
- **Professional Tools**: Advanced features for commercial use
- **Educational Version**: Simplified interface for teaching geography

### Conclusion

This cross-platform fantasy map generator represents a significant improvement over existing web-based solutions. By leveraging native performance, touch-optimized interfaces, and a robust import system, we can create a tool that serves both casual users and professional world builders.

The phased development approach minimizes risk while ensuring early user feedback. The shared core engine architecture provides consistency while allowing platform-specific optimizations.

Success depends on careful attention to performance, user experience, and maintaining compatibility with the popular Azgaar format. With proper execution, this project can become the definitive tool for fantasy map creation and editing.