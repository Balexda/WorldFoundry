# Azgaar Import Workflow

## Overview

The Azgaar import workflow allows users to import maps created in Azgaar's Fantasy Map Generator into World Foundry, preserving geographical, cultural, political, and religious data while adapting it to World Foundry's enhanced data model.

## Supported Import Formats

### Primary Format: JSON Export
- **File Extension**: `.json`
- **Source**: Azgaar's "Save → Export → JSON" option
- **Contains**: Complete world data including geography, cultures, religions, states, and metadata
- **Recommended**: ✅ Full feature support

### Secondary Format: Map File
- **File Extension**: `.map`
- **Source**: Azgaar's native save format
- **Contains**: Complete world data in compressed binary format
- **Status**: 🚧 Planned for future implementation

### Supplementary Formats
- **SVG Export**: For visual reference and layer extraction
- **GeoJSON**: For geographical data validation
- **CSV Data**: For culture/religion/state information

## Prerequisites

### Azgaar Map Requirements
- **Minimum Version**: Azgaar's Fantasy Map Generator v1.8+
- **Recommended Version**: v1.9+ for full compatibility
- **Map Completion**: At least basic geography and biomes defined
- **File Size**: Recommended maximum 50MB for optimal performance

### World Foundry Requirements
- **Application**: World Foundry installed and running
- **Storage Space**: 2-3x the source file size for processing
- **Memory**: Minimum 4GB RAM for large maps (>10,000 cells)

## Step-by-Step Import Process

### Phase 1: Preparation in Azgaar

#### 1.1 Verify Map Completeness
```
✅ Geography generated (elevation, temperature, precipitation)
✅ Biomes assigned to all land cells
✅ Cultures created and distributed
✅ Religions established
✅ States/nations defined
✅ Cities and towns placed
✅ Rivers and lakes generated
```

#### 1.2 Export from Azgaar
1. **Open your map** in Azgaar's Fantasy Map Generator
2. **Navigate to Menu** → `Options` → `Save and Load`
3. **Click "Export"** button
4. **Select "JSON"** format
5. **Choose export options**:
   - ✅ Include all layers
   - ✅ Include metadata
   - ✅ Include cultures data
   - ✅ Include religions data
   - ✅ Include states data
   - ✅ Include cities data
6. **Save the file** with a descriptive name (e.g., `MyWorld_v1.json`)

#### 1.3 Verify Export Quality
```bash
# Check file size (should be reasonable)
ls -lh MyWorld_v1.json

# Verify JSON structure (optional)
head -n 20 MyWorld_v1.json
```

### Phase 2: Import into World Foundry

#### 2.1 Launch Import Workflow

**Windows (WinUI 3):**
1. Open World Foundry
2. Click `File` → `Import` → `From Azgaar`
3. Or use keyboard shortcut: `Ctrl+Shift+I`
4. Or drag and drop the JSON file onto the main window

**iOS (SwiftUI):**
1. Open World Foundry
2. Tap the `+` button in the top navigation
3. Select `Import from Azgaar`
4. Use the file picker to select your JSON file

**Android (Jetpack Compose):**
1. Open World Foundry
2. Tap the floating action button (`+`)
3. Select `Import Azgaar Map`
4. Use the system file picker to locate your JSON file

#### 2.2 File Selection and Validation

```
📁 File Selection Dialog
├── 📂 Recent Files
│   ├── MyWorld_v1.json (2.3 MB)
│   └── TestMap.json (1.1 MB)
├── 📂 Browse Files
│   └── [System file picker]
└── 🔗 Import from URL (future feature)
```

**Validation Process:**
1. **File Format Check**: Verify JSON structure
2. **Version Compatibility**: Check Azgaar version compatibility
3. **Data Integrity**: Validate required fields
4. **Size Assessment**: Estimate import time and memory usage

#### 2.3 Import Configuration

```
⚙️ Import Settings
├── 📊 Data Selection
│   ├── ✅ Geography (elevation, biomes, climate)
│   ├── ✅ Cultures (names, colors, expansionism)
│   ├── ✅ Religions (names, colors, theology)
│   ├── ✅ States (names, colors, government)
│   ├── ✅ Cities and Towns
│   ├── ✅ Rivers and Lakes
│   └── ✅ Metadata (name, seed, settings)
├── 🎨 Visual Settings
│   ├── Color Scheme: [Azgaar Original] [World Foundry] [Custom]
│   ├── Scale Factor: [1.0x] [1.5x] [2.0x]
│   └── Layer Visibility: [Configure...]
├── 🔧 Processing Options
│   ├── Optimization Level: [Fast] [Balanced] [Quality]
│   ├── Memory Usage: [Conservative] [Normal] [Aggressive]
│   └── Background Processing: [✅ Enabled]
└── 📝 Naming
    ├── World Name: [Auto-detect] [Custom: "My Fantasy World"]
    └── Import Notes: [Optional description]
```

#### 2.4 Import Execution

**Progress Indicators:**
```
🔄 Importing "MyWorld_v1.json"
├── Phase 1: Parsing JSON data... ████████████████████ 100%
├── Phase 2: Processing geography... ███████████████░░░░░ 75%
├── Phase 3: Converting cultures... ████████░░░░░░░░░░░░ 40%
├── Phase 4: Processing religions... ░░░░░░░░░░░░░░░░░░░░ 0%
├── Phase 5: Converting states... ░░░░░░░░░░░░░░░░░░░░ 0%
├── Phase 6: Processing cities... ░░░░░░░░░░░░░░░░░░░░ 0%
└── Phase 7: Finalizing import... ░░░░░░░░░░░░░░░░░░░░ 0%

⏱️ Estimated time remaining: 2 minutes 34 seconds
💾 Memory usage: 1.2 GB / 4.0 GB available
🔄 Processing 15,847 cells...
```

**Detailed Phase Breakdown:**

**Phase 1: JSON Parsing**
- Parse JSON structure
- Validate data integrity
- Extract metadata
- Estimate processing requirements

**Phase 2: Geography Processing**
- Convert elevation data
- Map biome types to World Foundry system
- Process climate data (temperature, precipitation)
- Generate terrain features

**Phase 3: Culture Conversion**
- Map Azgaar cultures to World Foundry culture system
- Convert culture properties (expansionism, type, etc.)
- Process culture colors and visual styling
- Generate culture influence maps

**Phase 4: Religion Processing**
- Convert religious data structures
- Map theology types and characteristics
- Process religious influence and spread
- Handle religious conflicts and relationships

**Phase 5: State Conversion**
- Convert political entities
- Map government types
- Process diplomatic relationships
- Handle territorial boundaries

**Phase 6: Settlement Processing**
- Convert cities and towns
- Map settlement types and sizes
- Process economic and demographic data
- Generate trade routes (if available)

**Phase 7: Finalization**
- Optimize data structures
- Generate preview images
- Create backup of original data
- Prepare for rendering

### Phase 3: Import Review and Validation

#### 3.1 Import Summary

```
📋 Import Summary: "MyWorld_v1.json"
├── 📊 Statistics
│   ├── Total Cells: 15,847
│   ├── Land Cells: 8,234 (52%)
│   ├── Water Cells: 7,613 (48%)
│   ├── Cultures: 23
│   ├── Religions: 12
│   ├── States: 18
│   ├── Cities: 156
│   └── Towns: 423
├── ⚠️ Warnings
│   ├── 3 cultures had missing color data (assigned defaults)
│   ├── 1 religion had invalid theology type (converted to Folk)
│   └── 2 states had overlapping territories (boundaries adjusted)
├── ❌ Errors
│   └── None
└── ✅ Import Status: Successful
```

#### 3.2 Visual Preview

**Map Overview:**
- Thumbnail preview of the imported world
- Layer toggles for different data types
- Zoom and pan capabilities for detailed inspection

**Data Validation:**
- Culture distribution visualization
- Religious influence heatmap
- Political boundary verification
- Settlement placement review

#### 3.3 Data Verification Tools

**Geography Validation:**
```
🗺️ Geography Check
├── Elevation Range: -0.2 to 1.0 (✅ Valid)
├── Biome Distribution: 
│   ├── Ocean: 48.1% (✅ Reasonable)
│   ├── Forest: 23.4% (✅ Good)
│   ├── Grassland: 15.2% (✅ Good)
│   ├── Desert: 8.7% (✅ Reasonable)
│   ├── Mountain: 3.1% (✅ Good)
│   └── Tundra: 1.5% (✅ Good)
└── Climate Consistency: ✅ Passed
```

**Cultural Analysis:**
```
👥 Culture Analysis
├── Culture Count: 23 (✅ Diverse)
├── Average Territory: 358 cells (✅ Balanced)
├── Largest Culture: "Koreans" (1,247 cells)
├── Smallest Culture: "Highlanders" (89 cells)
├── Expansion Conflicts: 2 (⚠️ Minor)
└── Color Uniqueness: ✅ All distinct
```

### Phase 4: Post-Import Configuration

#### 4.1 World Settings Adjustment

```
⚙️ World Configuration
├── 📏 Scale and Units
│   ├── Distance Unit: [Miles] [Kilometers] [Custom]
│   ├── Scale Factor: 1 pixel = 2.5 km
│   └── World Circumference: ~40,000 km
├── 🕐 Time and Calendar
│   ├── Calendar System: [Earth-like] [Custom] [Fantasy]
│   ├── Year Length: 365 days
│   └── Current Date: Year 1, Day 1
├── 🎮 Gameplay Settings
│   ├── Simulation Speed: [Real-time] [Accelerated] [Turn-based]
│   ├── Random Events: [✅ Enabled]
│   └── AI Behavior: [Passive] [Active] [Aggressive]
└── 💾 Save Settings
    ├── Auto-save Interval: 5 minutes
    └── Backup Retention: 10 versions
```

#### 4.2 Layer Configuration

```
🎨 Layer Management
├── 📍 Base Layers
│   ├── ✅ Elevation (Opacity: 80%)
│   ├── ✅ Biomes (Opacity: 100%)
│   ├── ✅ Climate (Opacity: 60%)
│   └── ✅ Water Bodies (Opacity: 100%)
├── 👥 Cultural Layers
│   ├── ✅ Culture Territories (Opacity: 70%)
│   ├── ✅ Culture Names (Font: 12pt)
│   └── ☐ Culture Influence (Hidden)
├── ⛪ Religious Layers
│   ├── ✅ Religious Territories (Opacity: 50%)
│   ├── ☐ Religious Centers (Hidden)
│   └── ☐ Religious Conflicts (Hidden)
├── 🏛️ Political Layers
│   ├── ✅ State Boundaries (Width: 2px)
│   ├── ✅ State Names (Font: 14pt, Bold)
│   └── ☐ Diplomatic Relations (Hidden)
└── 🏙️ Settlement Layers
    ├── ✅ Cities (Size: Large icons)
    ├── ✅ Towns (Size: Medium icons)
    └── ✅ Settlement Names (Font: 10pt)
```

#### 4.3 Quality Assurance Checklist

```
✅ Post-Import QA Checklist
├── 🗺️ Visual Quality
│   ├── ✅ Map renders correctly at all zoom levels
│   ├── ✅ Colors are distinct and readable
│   ├── ✅ Text labels are properly positioned
│   └── ✅ No visual artifacts or corruption
├── 📊 Data Integrity
│   ├── ✅ All cultures have valid territories
│   ├── ✅ Religious data is consistent
│   ├── ✅ State boundaries are properly defined
│   └── ✅ Settlement data is complete
├── 🔧 Functionality
│   ├── ✅ Zoom and pan work smoothly
│   ├── ✅ Layer toggles function correctly
│   ├── ✅ Selection tools work properly
│   └── ✅ Export functions are available
└── 💾 Data Persistence
    ├── ✅ World saves correctly
    ├── ✅ Reload preserves all data
    └── ✅ Backup creation successful
```

## Advanced Import Options

### Batch Import Processing

For importing multiple Azgaar maps:

```
📦 Batch Import
├── 📁 Source Directory: /Maps/Azgaar/
├── 🎯 Target Collection: "Campaign Worlds"
├── ⚙️ Processing Options
│   ├── Import Settings: [Use Template] [Individual Config]
│   ├── Naming Convention: [Original] [Sequential] [Custom Pattern]
│   └── Error Handling: [Skip] [Prompt] [Auto-fix]
├── 📊 Progress Tracking
│   ├── Queue: 5 files pending
│   ├── Current: "Continent_East.json" (60% complete)
│   ├── Completed: 3 files
│   └── Failed: 0 files
└── 📝 Results Log
    ├── ✅ "MainWorld.json" - Imported successfully
    ├── ✅ "Islands.json" - Imported with 2 warnings
    ├── ✅ "Archipelago.json" - Imported successfully
    ├── 🔄 "Continent_East.json" - Processing...
    └── ⏳ "Northern_Lands.json" - Queued
```

### Custom Import Mappings

For advanced users who want to customize how Azgaar data maps to World Foundry:

```json
{
  "import_mapping": {
    "biomes": {
      "1": "ocean",
      "2": "forest_temperate",
      "3": "grassland_temperate",
      "4": "desert_hot",
      "5": "mountain_temperate",
      "custom_biome_12": "swamp_tropical"
    },
    "culture_types": {
      "Generic": "tribal",
      "River": "riverine",
      "Lake": "lacustrine",
      "Naval": "maritime",
      "Nomadic": "nomadic",
      "Hunting": "hunter_gatherer",
      "Highland": "highland"
    },
    "government_types": {
      "Monarchy": "absolute_monarchy",
      "Republic": "republic",
      "Theocracy": "theocracy",
      "Union": "federation"
    }
  }
}
```

## Troubleshooting Common Issues

### Import Failures

#### Issue: "Invalid JSON Format"
**Symptoms:**
- Import fails immediately
- Error message: "Failed to parse JSON data"

**Solutions:**
1. **Verify file integrity:**
   ```bash
   # Check if file is valid JSON
   python -m json.tool MyWorld.json > /dev/null
   ```
2. **Re-export from Azgaar** with latest version
3. **Check file encoding** (should be UTF-8)
4. **Reduce file size** by simplifying the map

#### Issue: "Memory Insufficient"
**Symptoms:**
- Import starts but fails during processing
- System becomes unresponsive
- Error: "Out of memory"

**Solutions:**
1. **Close other applications** to free memory
2. **Reduce import quality** settings
3. **Enable conservative memory mode**
4. **Split large maps** into smaller regions
5. **Increase system virtual memory**

#### Issue: "Corrupted Culture Data"
**Symptoms:**
- Import completes but cultures are missing
- Strange culture names or colors
- Territories not properly assigned

**Solutions:**
1. **Check Azgaar map completeness** before export
2. **Use "Regenerate Cultures"** in Azgaar before export
3. **Enable "Auto-fix culture data"** in import settings
4. **Manual culture assignment** post-import

### Performance Issues

#### Slow Import Processing
**Optimization strategies:**
1. **Enable background processing**
2. **Reduce visual quality during import**
3. **Close unnecessary applications**
4. **Use SSD storage** for temporary files
5. **Increase processing thread count**

#### Large File Handling
**For maps >20MB:**
1. **Enable streaming import mode**
2. **Use progressive loading**
3. **Implement data compression**
4. **Consider map subdivision**

## Integration with World Foundry Features

### Post-Import Enhancements

#### Automatic Feature Generation
After successful import, World Foundry can automatically generate:
- **Trade routes** between cities
- **Cultural migration patterns**
- **Religious pilgrimage sites**
- **Natural resource deposits**
- **Climate change projections**
- **Population growth models**

#### Data Enrichment
- **Historical timeline generation**
- **Language family trees**
- **Technological development levels**
- **Economic system modeling**
- **Diplomatic relationship networks**

### Export Compatibility

Imported Azgaar maps can be exported to:
- **World Foundry native format** (.wfmap)
- **Enhanced JSON** with World Foundry extensions
- **Campaign management tools** (Roll20, Foundry VTT)
- **3D visualization formats** (OBJ, glTF)
- **Geographic formats** (GeoJSON, KML)

## Best Practices

### Pre-Import Preparation
1. **Complete your Azgaar map** fully before export
2. **Use consistent naming conventions** for cultures, religions, states
3. **Verify data relationships** (culture-religion-state alignment)
4. **Test export/import** with a small test map first
5. **Document your world** with notes and descriptions

### Import Configuration
1. **Start with default settings** for first import
2. **Enable all data types** unless specifically unwanted
3. **Use balanced optimization** for best quality/speed ratio
4. **Keep original files** as backup
5. **Document any custom mappings** used

### Post-Import Workflow
1. **Review import summary** for warnings/errors
2. **Validate visual appearance** at multiple zoom levels
3. **Test core functionality** (zoom, pan, selection)
4. **Save immediately** after successful import
5. **Create backup** before making modifications

## Future Enhancements

### Planned Features
- **Real-time import progress** with detailed phase information
- **Selective data import** (choose specific cultures, regions)
- **Import diff comparison** (compare multiple versions)
- **Automated quality scoring** for imported maps
- **Cloud-based import processing** for large files
- **AI-powered data enhancement** post-import

### Community Contributions
- **Custom import templates** for specific campaign types
- **Community mapping standards** for consistent imports
- **Shared import configurations** for common use cases
- **Import quality metrics** and benchmarking
- **Collaborative import validation** tools

## Related Workflows

- [Creating New Cultures](../creation/CULTURE_CREATION.md) - Enhance imported cultures
- [Religion Management](../creation/RELIGION_CREATION.md) - Modify imported religions
- [State Administration](../editing/STATE_MANAGEMENT.md) - Manage political entities
- [Map Export](../export/MAP_EXPORT.md) - Export your enhanced world
- [Collaboration Setup](../collaboration/WORLD_SHARING.md) - Share imported worlds

## Support and Resources

### Documentation
- [Azgaar's Fantasy Map Generator Documentation](https://github.com/Azgaar/Fantasy-Map-Generator/wiki)
- [World Foundry Data Model Reference](../../architecture/DATA_MODEL.md)
- [Import API Documentation](../../api/IMPORT_API.md)

### Community
- [World Foundry Discord](https://discord.gg/worldfoundry) - #import-help channel
- [GitHub Discussions](https://github.com/Balexda/WorldFoundry/discussions) - Import category
- [Reddit Community](https://reddit.com/r/worldfoundry) - Import showcase

### Reporting Issues
- [Bug Reports](https://github.com/Balexda/WorldFoundry/issues/new?template=bug_report.yml)
- [Feature Requests](https://github.com/Balexda/WorldFoundry/issues/new?template=feature_request.yml)
- [Import-Specific Issues](https://github.com/Balexda/WorldFoundry/issues/new?template=import_issue.yml)