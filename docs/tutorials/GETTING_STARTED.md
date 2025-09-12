# Getting Started with World Foundry

Welcome to World Foundry! This tutorial will guide you through your first steps with our cross-platform fantasy map generator. By the end of this guide, you'll have imported your first map, explored the interface, and created your first custom content.

## 🎯 What You'll Learn

- How to install and set up World Foundry on your device
- How to import a map from Azgaar's Fantasy Map Generator
- How to navigate the World Foundry interface
- How to view and explore world data
- How to make your first modifications
- How to export your work

## 📋 Prerequisites

### System Requirements

**Windows:**
- Windows 10 version 1809 or later
- 4GB RAM minimum, 8GB recommended
- 2GB free storage space
- DirectX 11 compatible graphics

**iOS:**
- iOS 15.0 or later
- iPad recommended for best experience
- 2GB free storage space

**Android:**
- Android 7.0 (API level 24) or later
- 3GB RAM minimum, 4GB recommended
- 2GB free storage space

### Optional: Azgaar Map

If you want to follow along with the import tutorial, you'll need a map from [Azgaar's Fantasy Map Generator](https://azgaar.github.io/Fantasy-Map-Generator/):

1. Visit the Azgaar website
2. Generate or load a map you like
3. Go to `Tools` → `Save and Load` → `Download Map`
4. Save the `.map` file to your device

## 🚀 Installation

### Windows Installation

1. **Download World Foundry**
   - Visit the [releases page](https://github.com/Balexda/WorldFoundry/releases)
   - Download the latest Windows installer (`WorldFoundry-Setup.exe`)

2. **Install the Application**
   - Run the installer as administrator
   - Follow the installation wizard
   - Choose installation directory (default recommended)
   - Wait for installation to complete

3. **First Launch**
   - Launch World Foundry from the Start Menu
   - Allow Windows Defender/antivirus if prompted
   - The application will perform first-time setup

### iOS Installation

1. **Download from App Store**
   - Open the App Store on your iOS device
   - Search for "World Foundry"
   - Tap "Get" to download and install

2. **First Launch**
   - Tap the World Foundry icon to launch
   - Grant necessary permissions when prompted
   - The app will download initial resources

### Android Installation

1. **Download from Google Play**
   - Open Google Play Store
   - Search for "World Foundry"
   - Tap "Install" to download and install

2. **Alternative: Direct APK**
   - Download the APK from the releases page
   - Enable "Install from unknown sources" in Settings
   - Install the APK file

3. **First Launch**
   - Open World Foundry from your app drawer
   - Grant necessary permissions when prompted
   - Wait for initial setup to complete

## 🌍 Your First World

### Starting Fresh vs. Importing

When you first launch World Foundry, you'll see the welcome screen with two main options:

```
🏠 World Foundry Welcome
├── 🆕 Create New World
│   ├── Quick Start (Recommended for beginners)
│   ├── Custom Generation
│   └── Advanced Configuration
└── 📥 Import Existing World
    ├── Import from Azgaar
    ├── Import World Foundry File
    └── Import from Other Sources
```

For this tutorial, we'll start by importing an Azgaar map to give you immediate content to explore.

### Importing Your First Map

#### Step 1: Access Import Function

**Windows:**
1. Click `File` → `Import` → `From Azgaar`
2. Or use keyboard shortcut: `Ctrl+I`

**iOS:**
1. Tap the `+` button in the top-right corner
2. Select `Import from Azgaar`

**Android:**
1. Tap the menu button (☰) in the top-left
2. Select `Import` → `From Azgaar`

#### Step 2: Select Your Map File

```
📁 File Selection Dialog
├── 📂 Recent Files
│   └── (Previously imported maps appear here)
├── 📂 Browse Files
│   ├── Documents
│   ├── Downloads
│   └── Other Locations
└── 🔗 Import from URL
    └── (Paste Azgaar share link)
```

1. **Browse to your Azgaar file**
   - Navigate to where you saved the `.map` file
   - Select the file and click/tap "Open"

2. **Or import from URL**
   - If you have an Azgaar share link, paste it here
   - World Foundry will download and import automatically

#### Step 3: Configure Import Options

```
⚙️ Import Configuration
├── 📊 Data to Import
│   ├── ☑️ Geographic Data (Terrain, biomes, climate)
│   ├── ☑️ Cultural Data (Cultures, languages, religions)
│   ├── ☑️ Political Data (States, provinces, diplomacy)
│   ├── ☑️ Settlement Data (Cities, towns, villages)
│   └── ☑️ Economic Data (Trade routes, resources)
├── 🎯 Detail Level
│   ├── ○ Basic (Core data only)
│   ├── ● Standard (Recommended)
│   └── ○ Comprehensive (All available data)
├── 🔧 Processing Options
│   ├── ☑️ Generate missing data
│   ├── ☑️ Validate imported data
│   └── ☑️ Create backup of original
└── 📝 World Information
    ├── Name: [Auto-detected from file]
    ├── Description: [Optional custom description]
    └── Tags: [Optional tags for organization]
```

**Recommended Settings for First Import:**
- Keep all data types checked
- Use "Standard" detail level
- Enable all processing options
- Add a custom description if desired

#### Step 4: Start Import Process

1. Click/tap "Start Import"
2. Watch the progress indicator:

```
🔄 Import Progress
├── Stage: Reading Azgaar file... ████████████ 100%
├── Stage: Processing geography... ████████░░░░ 75%
├── Current: Analyzing biome distributions
├── Processed: 1,247 of 1,650 cells
└── Estimated time remaining: 2 minutes
```

3. Wait for completion (typically 2-5 minutes depending on map size)

#### Step 5: Import Complete

```
✅ Import Successful!
├── 📊 Import Summary
│   ├── Geographic cells: 1,650 processed
│   ├── Cultures: 12 imported
│   ├── Religions: 8 imported
│   ├── States: 15 imported
│   ├── Settlements: 89 imported
│   └── Trade routes: 23 imported
├── ⚠️ Warnings (2)
│   ├── Some culture names were duplicated and renamed
│   └── 3 settlements had missing population data (estimated)
└── 🎯 Next Steps
    ├── Explore your world
    ├── Customize imported data
    └── Export in new formats
```

Click/tap "Explore World" to continue.

## 🗺️ Exploring Your World

### The Main Interface

Once your world is loaded, you'll see the main World Foundry interface:

```
🖥️ World Foundry Interface
├── 🔝 Top Bar
│   ├── World name and status
│   ├── View controls (zoom, pan, layers)
│   └── Main menu and tools
├── 🗺️ Main Map View
│   ├── Interactive world map
│   ├── Layer overlays
│   └── Selection tools
├── 📊 Side Panel (Collapsible)
│   ├── World overview
│   ├── Selected item details
│   └── Quick actions
└── 🔽 Bottom Bar
    ├── Coordinate display
    ├── Scale indicator
    └── Status messages
```

### Navigation Basics

#### Map Navigation

**Windows:**
- **Pan**: Click and drag, or use arrow keys
- **Zoom**: Mouse wheel, or `+`/`-` keys
- **Fit to screen**: `F` key or View menu
- **Reset view**: `R` key

**iOS:**
- **Pan**: Touch and drag
- **Zoom**: Pinch gesture or double-tap
- **Fit to screen**: Double-tap with two fingers
- **Reset view**: Shake device or tap reset button

**Android:**
- **Pan**: Touch and drag
- **Zoom**: Pinch gesture or double-tap
- **Fit to screen**: Two-finger double-tap
- **Reset view**: Tap reset button in toolbar

#### Layer Controls

```
🎨 Map Layers
├── 🌍 Base Layers
│   ├── ☑️ Terrain (elevation, landforms)
│   ├── ☑️ Water Bodies (rivers, lakes, oceans)
│   ├── ☑️ Biomes (forests, deserts, grasslands)
│   └── ☑️ Climate (temperature, precipitation)
├── 👥 Cultural Layers
│   ├── ☑️ Culture Territories
│   ├── ☑️ Language Regions
│   ├── ☑️ Religious Influence
│   └── ☐ Historical Events
├── 🏛️ Political Layers
│   ├── ☑️ State Boundaries
│   ├── ☑️ Provincial Divisions
│   ├── ☐ Diplomatic Relations
│   └── ☐ Military Positions
├── 🏘️ Settlement Layers
│   ├── ☑️ Cities and Towns
│   ├── ☑️ Trade Routes
│   ├── ☐ Economic Zones
│   └── ☐ Population Density
└── 🎯 Overlay Options
    ├── ☑️ Labels and Names
    ├── ☐ Coordinate Grid
    ├── ☐ Scale Indicators
    └── ☐ Legend
```

**Try This:**
1. Toggle different layers on and off to see how they change the map
2. Try the "Culture Territories" layer to see cultural boundaries
3. Enable "Trade Routes" to see economic connections
4. Turn on "Labels and Names" for easier navigation

### Exploring World Data

#### Selecting and Viewing Items

**To select an item on the map:**
1. Click/tap on any settlement, culture territory, or geographic feature
2. The item will be highlighted
3. Details will appear in the side panel

**Example: Exploring a Settlement**

When you click on a city, you'll see:

```
🏘️ Settlement Details: Millbrook
├── 📊 Basic Information
│   ├── Type: Town
│   ├── Population: 1,247 residents
│   ├── Founded: Year 312, Age of Rivers
│   └── Culture: Riverlands Folk
├── 📍 Location
│   ├── Coordinates: (245.7, 156.3)
│   ├── Elevation: 125m above sea level
│   ├── Biome: Temperate Forest
│   └── Water Access: Swift Current River
├── 💼 Economy
│   ├── Primary Industry: Grain Milling
│   ├── Wealth Level: Comfortable
│   ├── Trade Connections: 8 routes
│   └── Market Day: Every Seventhday
├── 🏛️ Governance
│   ├── Government: Town Council
│   ├── Leader: Mayor Deepcurrent
│   ├── Laws: Regional + Local
│   └── Defenses: Town Guard (12 members)
└── 🎯 Quick Actions
    ├── Edit Settlement
    ├── View Trade Network
    ├── Explore Culture
    └── Generate NPCs
```

**Try This:**
1. Click on different settlements to compare them
2. Look for patterns in settlement sizes and locations
3. Notice how geography affects settlement development
4. Explore the trade connections between settlements

#### Understanding Cultures

Click on a culture territory to explore cultural data:

```
👥 Culture Details: Riverlands Folk
├── 🎭 Identity
│   ├── Name: Riverlands Folk
│   ├── Adjective: Riverlands
│   ├── People: Riverlanders
│   └── Population: 15,847 people
├── 🌍 Territory
│   ├── Core Region: Swift Current Valley
│   ├── Total Area: 2,847 km²
│   ├── Settlements: 23 (3 towns, 20 villages)
│   └── Expansion: Moderate, along waterways
├── 🎨 Characteristics
│   ├── Primary Colors: River Blue, Earth Brown
│   ├── Symbol: Flowing Water
│   ├── Values: Community, Adaptability, Trade
│   └── Traditions: Harvest Festival, River Blessing
├── 💼 Economy
│   ├── Focus: Agriculture and Trade
│   ├── Specialties: Grain, River Transport
│   ├── Trade Partners: Forest Dwellers, Mountain Clans
│   └── Currency: Regional Silver Coins
├── ⛪ Religion
│   ├── Primary Faith: The Flowing Faith (65%)
│   ├── Beliefs: Water as life source, seasonal cycles
│   ├── Practices: River ceremonies, community rituals
│   └── Sacred Sites: 3 major temples
└── 🗣️ Language
    ├── Primary Language: Riverspeak
    ├── Writing System: Flowing Script
    ├── Literacy Rate: 35%
    └── Naming Patterns: Water-themed names
```

### Using the Search Function

Find specific items quickly using the search feature:

**Windows:** `Ctrl+F` or click the search icon
**iOS/Android:** Tap the search icon in the toolbar

```
🔍 Search World Foundry
├── 🏘️ Settlements
│   ├── "Mill" → Millbrook, Miller's Rest, Old Mill
│   └── "Capital" → Goldbrook (Regional Capital)
├── 👥 Cultures
│   ├── "River" → Riverlands Folk
│   └── "Forest" → Forest Dwellers
├── ⛪ Religions
│   ├── "Flow" → The Flowing Faith
│   └── "Nature" → Nature Spirits
├── 🏛️ States
│   └── "Kingdom" → Northern River Kingdom
└── 🗺️ Geographic Features
    ├── "Swift" → Swift Current River
    └── "Mountain" → Ironhold Mountains
```

**Try This:**
1. Search for "river" to find all river-related items
2. Search for your favorite fantasy name patterns
3. Use search to quickly navigate large worlds

## ✏️ Making Your First Modifications

Now that you've explored your imported world, let's make some customizations.

### Editing Settlement Information

1. **Select a settlement** by clicking/tapping on it
2. **Click "Edit Settlement"** in the details panel
3. **Modify the information:**

```
✏️ Edit Settlement: Millbrook
├── 📝 Basic Information
│   ├── Name: [Millbrook] → [New Millbrook]
│   ├── Type: [Town] → [City] (if population supports it)
│   ├── Description: [Add custom description]
│   └── Nickname: [Add local nickname]
├── 👥 Population
│   ├── Current: [1,247] → [1,500] (simulate growth)
│   ├── Growth Rate: [2.1%] → [2.5%]
│   └── Demographics: [Adjust age distribution]
├── 💼 Economy
│   ├── Add Industry: [Select] → Textiles
│   ├── Wealth Level: [Comfortable] → [Wealthy]
│   └── Trade Routes: [Add new connection]
├── 🏛️ Governance
│   ├── Government Type: [Keep current]
│   ├── Leader Name: [Change if desired]
│   └── Special Laws: [Add local regulations]
└── 🎨 Appearance
    ├── Settlement Color: [Choose custom color]
    ├── Symbol: [Select from library]
    └── Architecture Style: [Regional variations]
```

4. **Save your changes** - they'll be immediately reflected on the map

### Adding a New Settlement

Let's add a small village to your world:

1. **Find a suitable location** on the map
   - Look for areas near water sources
   - Consider proximity to existing settlements
   - Check the terrain and biome suitability

2. **Right-click (or long-press) on the location**
3. **Select "Create New Settlement"**

```
🆕 New Settlement Creation
├── 📍 Location Confirmation
│   ├── Coordinates: (Auto-filled from click)
│   ├── Terrain: [Auto-detected] Gentle Hills
│   ├── Biome: [Auto-detected] Temperate Forest
│   └── Water Access: [Auto-detected] Small Stream
├── 🏷️ Basic Identity
│   ├── Name: [Generate] → Streamside
│   ├── Type: [Dropdown] → Village
│   ├── Population: [Suggested] → 245 residents
│   └── Founded: [Date picker] → Recent
├── 👥 Cultural Association
│   ├── Dominant Culture: [Dropdown] → Riverlands Folk
│   ├── Cultural Influence: [Slider] → 85%
│   ├── Language: [Auto-selected] → Riverspeak
│   └── Integration Level: [Dropdown] → Well-integrated
├── 💼 Economic Foundation
│   ├── Primary Industry: [Dropdown] → Agriculture
│   ├── Secondary Industry: [Dropdown] → Crafts
│   ├── Wealth Level: [Dropdown] → Modest
│   └── Trade Connections: [Multi-select] → Millbrook
└── 🏛️ Governance
    ├── Government: [Dropdown] → Village Elder
    ├── Autonomy Level: [Slider] → Moderate
    └── Regional Authority: [Auto-selected] → Same as culture
```

4. **Click "Create Settlement"**
5. **Your new village appears on the map!**

### Customizing Culture Information

1. **Select a culture territory** on the map
2. **Click "Edit Culture"** in the details panel
3. **Modify cultural characteristics:**

```
✏️ Edit Culture: Riverlands Folk
├── 🎨 Visual Identity
│   ├── Primary Color: [Color picker] → Deeper Blue
│   ├── Secondary Color: [Color picker] → Golden Brown
│   ├── Symbol: [Icon library] → Stylized River
│   └── Banner Design: [Template] → Flowing Patterns
├── 🎭 Cultural Traits
│   ├── Core Values: [Multi-select] → Add "Innovation"
│   ├── Social Structure: [Dropdown] → Keep current
│   ├── Traditions: [Text editor] → Add seasonal festivals
│   └── Taboos: [Text editor] → Add cultural restrictions
├── 💼 Economic Focus
│   ├── Primary: [Keep] → Agriculture and Trade
│   ├── Secondary: [Add] → River Transportation
│   ├── Specializations: [Multi-select] → Add "Boat Building"
│   └── Trade Preferences: [Adjust sliders]
├── ⛪ Religious Practices
│   ├── Primary Religion: [Keep] → The Flowing Faith
│   ├── Religious Devotion: [Slider] → Adjust level
│   ├── Sacred Practices: [Text editor] → Add rituals
│   └── Holy Sites: [Map selection] → Mark locations
└── 🗣️ Language and Names
    ├── Language Details: [Edit] → Add vocabulary
    ├── Naming Conventions: [Pattern editor] → Customize
    ├── Common Names: [List editor] → Add examples
    └── Place Naming: [Pattern editor] → Geographic terms
```

4. **Save changes** - the culture's territory will update its appearance

## 📤 Exporting Your Work

Once you've made modifications, you'll want to save and share your work.

### Saving Your World

**Auto-save:** World Foundry automatically saves your changes every few minutes.

**Manual save:**
- **Windows:** `Ctrl+S` or File → Save
- **iOS:** Changes save automatically, tap "Sync" to backup
- **Android:** Tap menu → Save, or changes save automatically

### Exporting Maps and Data

#### Export a Map Image

1. **Go to File → Export → Map Image** (or equivalent on mobile)
2. **Configure export settings:**

```
🖼️ Map Export Configuration
├── 📐 Image Settings
│   ├── Resolution: [Dropdown] → High (2048x1024)
│   ├── Format: [Dropdown] → PNG (recommended)
│   ├── Quality: [Slider] → High
│   └── DPI: [Input] → 300 (for printing)
├── 🎨 Visual Style
│   ├── Style: [Dropdown] → Realistic
│   ├── Color Scheme: [Dropdown] → Natural
│   ├── Lighting: [Dropdown] → Soft Shadows
│   └── Artistic Effects: [Checkboxes] → None
├── 🗺️ Map Layers
│   ├── ☑️ Terrain and Water
│   ├── ☑️ Political Boundaries
│   ├── ☑️ Settlements
│   ├── ☑️ Labels
│   └── ☐ Grid Lines
├── 📍 Map Region
│   ├── ○ Entire World
│   ├── ● Current View
│   └── ○ Custom Selection
└── 🎯 Purpose
    ├── ○ Digital Display
    ├── ● Print Production
    └── ○ Web Sharing
```

3. **Click "Export"** and choose save location
4. **Wait for processing** (may take a few minutes for high-resolution exports)

#### Export World Data

For sharing with other World Foundry users or backup:

1. **Go to File → Export → World Data**
2. **Choose format:**

```
📊 Data Export Options
├── 🌍 World Foundry Native
│   ├── Complete world data
│   ├── All customizations preserved
│   └── Best for sharing with other users
├── 📄 JSON Format
│   ├── Human-readable text format
│   ├── Good for data analysis
│   └── Compatible with many tools
├── 🗺️ GeoJSON
│   ├── Geographic data standard
│   ├── Compatible with GIS software
│   └── Good for mapping applications
└── 📋 CSV Data
    ├── Spreadsheet-compatible
    ├── Good for data analysis
    └── Settlement and culture lists
```

3. **Select what to include:**
   - Geographic data
   - Cultural information
   - Political structures
   - Settlement details
   - Custom modifications

4. **Export and save** to your desired location

## 🎉 Congratulations!

You've successfully:
- ✅ Installed World Foundry on your device
- ✅ Imported your first map from Azgaar
- ✅ Explored the interface and navigation
- ✅ Viewed detailed world information
- ✅ Made your first customizations
- ✅ Exported your work

## 🚀 Next Steps

Now that you're familiar with the basics, here are some suggestions for continuing your World Foundry journey:

### Immediate Next Steps
1. **Explore more of your world** - Click on different cultures, states, and geographic features
2. **Create more settlements** - Add villages, towns, or cities in strategic locations
3. **Customize cultures** - Develop unique cultural characteristics and traditions
4. **Build trade networks** - Connect settlements with trade routes

### Advanced Learning
1. **[Creating Your First World](FIRST_WORLD.md)** - Learn to generate worlds from scratch
2. **[Culture Creation Workflow](../workflows/creation/CULTURE_CREATION.md)** - Deep dive into culture design
3. **[Settlement Management](../workflows/editing/SETTLEMENT_MANAGEMENT.md)** - Advanced settlement features
4. **[Map Export Workflow](../workflows/export/MAP_EXPORT.md)** - Professional map production

### Community Engagement
1. **Share your creations** - Post your worlds in the community showcase
2. **Get feedback** - Ask for suggestions and improvements
3. **Learn from others** - Explore community-created worlds and techniques
4. **Contribute** - Help improve World Foundry by reporting bugs or suggesting features

## 🆘 Getting Help

If you encounter any issues or have questions:

### Documentation Resources
- **[User Workflows](../workflows/)** - Step-by-step guides for specific tasks
- **[FAQ](../community/SUPPORT.md)** - Common questions and answers
- **[Troubleshooting](../community/SUPPORT.md#troubleshooting)** - Solutions to common problems

### Community Support
- **GitHub Discussions** - Ask questions and share ideas
- **Discord Server** - Real-time chat with other users
- **Community Forums** - Detailed discussions and tutorials
- **Video Tutorials** - Visual learning resources

### Reporting Issues
If you find bugs or have feature requests:
1. Check existing issues on GitHub
2. Create a new issue with detailed information
3. Include screenshots and system information
4. Follow up on responses and testing requests

## 📚 Additional Resources

### Learning Materials
- **[World Building Basics](../tutorials/WORLDBUILDING_BASICS.md)** - Fundamental concepts
- **[Fantasy Geography](../tutorials/FANTASY_GEOGRAPHY.md)** - Creating believable worlds
- **[Cultural Design](../tutorials/CULTURAL_DESIGN.md)** - Developing rich cultures
- **[Political Systems](../tutorials/POLITICAL_SYSTEMS.md)** - Government and diplomacy

### Inspiration and Examples
- **Community World Gallery** - Explore user-created worlds
- **Historical Examples** - Real-world inspiration for fantasy settings
- **Literary Worlds** - Analysis of famous fantasy worlds
- **Gaming Applications** - Using World Foundry for tabletop RPGs

---

Welcome to the World Foundry community! We're excited to see what amazing worlds you'll create. Happy worldbuilding! 🌍✨

**Last Updated**: March 2024  
**Tutorial Version**: 1.0  
**Compatibility**: World Foundry 1.0+