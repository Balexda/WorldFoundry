# World Foundry Documentation

Welcome to the comprehensive documentation for World Foundry, a cross-platform fantasy map generator that imports Azgaar maps with native performance on Windows, iPad, and Android.

## 🚀 Quick Start

- **New Users**: Start with [Getting Started Tutorial](tutorials/GETTING_STARTED.md)
- **Developers**: Begin with [Development Setup](development/setup/DEVELOPMENT_SETUP.md)
- **Contributors**: Read the [Documentation Guide](DOCUMENTATION_GUIDE.md)

## 📋 Project Overview

- **[Project Status](PROJECT_STATUS.md)** - Current development status and roadmap
- **[Original Proposal](Proposal.md)** - Initial project vision and specifications
- **[Architecture Overview](architecture/ARCHITECTURE.md)** - System design and technical architecture

## 🏗️ Architecture & Technology

### System Architecture
- **[Core Architecture](architecture/ARCHITECTURE.md)** - Overall system design
- **[Data Model](architecture/DATA_MODEL.md)** - Data structures and relationships
- **[Platform Architecture](architecture/platforms/)** - Platform-specific implementations
  - [Windows Architecture](architecture/platforms/WINDOWS.md)
  - [iOS Architecture](architecture/platforms/IOS.md)
  - [Android Architecture](architecture/platforms/ANDROID.md)

### Technology Documentation
- **[Core Technologies](technology/core/)**
  - [Rust Core Engine](technology/core/RUST.md)
  - [Foreign Function Interface](technology/core/FFI.md)
  - [Performance Optimization](technology/core/PERFORMANCE.md)
- **[Platform Technologies](technology/platforms/)**
  - [WinUI 3 Guide](technology/platforms/WINUI.md)
  - [SwiftUI Guide](technology/platforms/SWIFTUI.md)
  - [Jetpack Compose Guide](technology/platforms/JETPACK_COMPOSE.md)

## 💻 Development

### Setup & Environment
- **[Development Setup](development/setup/DEVELOPMENT_SETUP.md)** - Cross-platform development environment
- **Platform-Specific Setup**:
  - [Windows Development](development/setup/WINDOWS_DEVELOPMENT.md)
  - [iOS Development](development/setup/IOS_DEVELOPMENT.md)
  - [Android Development](development/setup/ANDROID_DEVELOPMENT.md)

### Deployment & Testing
- **[Deployment Guides](development/deployment/)**
  - [Debug Deployment](development/deployment/DEBUG_DEPLOYMENT.md)
  - [Release Deployment](development/deployment/RELEASE_DEPLOYMENT.md)
  - [Platform Deployment](development/deployment/PLATFORM_DEPLOYMENT.md)
- **[Debugging](development/debugging/)**
  - [General Debugging](development/debugging/DEBUGGING_GUIDE.md)
  - [Windows Debugging](development/debugging/WINDOWS_DEBUGGING.md)
  - [iOS Debugging](development/debugging/IOS_DEBUGGING.md)
  - [Android Debugging](development/debugging/ANDROID_DEBUGGING.md)
- **[Testing](development/testing/)**
  - [Testing Strategy](development/testing/TESTING_STRATEGY.md)
  - [Unit Testing](development/testing/UNIT_TESTING.md)
  - [Integration Testing](development/testing/INTEGRATION_TESTING.md)

## 📖 User Workflows

### Import Workflows
- **[Azgaar Import](workflows/import/AZGAAR_IMPORT.md)** - Import maps from Azgaar's Fantasy Map Generator
- **[Generic Import](workflows/import/GENERIC_IMPORT.md)** - Import from other sources
- **[Batch Import](workflows/import/BATCH_IMPORT.md)** - Import multiple maps efficiently

### Content Creation
- **[Culture Creation](workflows/creation/CULTURE_CREATION.md)** - Design rich, detailed cultures
- **[Religion Creation](workflows/creation/RELIGION_CREATION.md)** - Create comprehensive belief systems
- **[Namebase Creation](workflows/creation/NAMEBASE_CREATION.md)** - Design naming systems and generators
- **[Language Creation](workflows/creation/LANGUAGE_CREATION.md)** - Develop constructed languages
- **[Character Creation](workflows/creation/CHARACTER_CREATION.md)** - Generate NPCs and characters

### Content Editing
- **[Map Editing](workflows/editing/MAP_EDITING.md)** - Modify and enhance maps
- **[Culture Editing](workflows/editing/CULTURE_EDITING.md)** - Update and refine cultures
- **[Settlement Management](workflows/editing/SETTLEMENT_MANAGEMENT.md)** - Manage cities and towns
- **[Trade Management](workflows/editing/TRADE_MANAGEMENT.md)** - Create and manage trade networks
- **[Diplomacy Management](workflows/editing/DIPLOMACY_MANAGEMENT.md)** - Handle diplomatic relations
- **[History Management](workflows/editing/HISTORY_MANAGEMENT.md)** - Track historical events and timelines

### Export & Sharing
- **[Map Export](workflows/export/MAP_EXPORT.md)** - Export maps in various formats
- **[Data Export](workflows/export/DATA_EXPORT.md)** - Export world data for external use
- **[Print Production](workflows/export/PRINT_PRODUCTION.md)** - Prepare high-quality prints
- **[Web Publishing](workflows/export/WEB_PUBLISHING.md)** - Share worlds online

### Collaboration
- **[World Sharing](workflows/collaboration/WORLD_SHARING.md)** - Share worlds with others
- **[Collaborative Editing](workflows/collaboration/COLLABORATIVE_EDITING.md)** - Multi-user world editing
- **[Version Control](workflows/collaboration/VERSION_CONTROL.md)** - Manage world versions

## 🔧 API Reference

- **[Core API](api/CORE_API.md)** - Core engine API reference
- **[Platform APIs](api/PLATFORM_APIS.md)** - Platform-specific API documentation
- **[Plugin API](api/PLUGIN_API.md)** - Plugin development interface

## 📚 Tutorials

- **[Getting Started](tutorials/GETTING_STARTED.md)** - Your first steps with World Foundry
- **[Creating Your First World](tutorials/FIRST_WORLD.md)** - Complete world creation tutorial
- **[Advanced Techniques](tutorials/ADVANCED_TECHNIQUES.md)** - Power user features and tips
- **[Campaign Preparation](tutorials/CAMPAIGN_PREPARATION.md)** - Preparing worlds for tabletop campaigns

## 🤝 Community

- **[Contributing](community/CONTRIBUTING.md)** - How to contribute to World Foundry
- **[Code of Conduct](community/CODE_OF_CONDUCT.md)** - Community guidelines and standards
- **[Support](community/SUPPORT.md)** - Getting help and reporting issues
- **[Resources](community/RESOURCES.md)** - External resources and useful links

## 📊 Documentation Status

### ✅ Complete Documentation
- Core architecture and platform designs
- Rust core engine implementation
- Windows development setup and workflows
- Major user workflows (import, creation, export)
- Comprehensive workflow guides with UI mockups

### 🚧 In Progress
- iOS and Android development guides
- Platform-specific technology documentation
- API reference documentation
- Tutorial content

### 📋 Planned
- Advanced debugging guides
- Performance optimization guides
- Plugin development documentation
- Community contribution tools

## 🔍 Finding Information

### By Role
- **End Users**: Start with [Tutorials](tutorials/) and [Workflows](workflows/)
- **Developers**: Begin with [Development](development/) and [Architecture](architecture/)
- **Contributors**: Read [Community](community/) and [Documentation Guide](DOCUMENTATION_GUIDE.md)

### By Platform
- **Windows**: [Windows Architecture](architecture/platforms/WINDOWS.md) → [Windows Development](development/setup/WINDOWS_DEVELOPMENT.md)
- **iOS**: [iOS Architecture](architecture/platforms/IOS.md) → [iOS Development](development/setup/IOS_DEVELOPMENT.md)
- **Android**: [Android Architecture](architecture/platforms/ANDROID.md) → [Android Development](development/setup/ANDROID_DEVELOPMENT.md)

### By Task
- **Setting up development**: [Development Setup](development/setup/DEVELOPMENT_SETUP.md)
- **Importing maps**: [Azgaar Import](workflows/import/AZGAAR_IMPORT.md)
- **Creating content**: [Creation Workflows](workflows/creation/)
- **Exporting results**: [Export Workflows](workflows/export/)
- **Troubleshooting**: Check individual workflow troubleshooting sections

## 📝 Documentation Standards

This documentation follows the standards outlined in the [Documentation Guide](DOCUMENTATION_GUIDE.md). All contributors should review this guide before creating or updating documentation.

### Key Features
- **Comprehensive Coverage**: Every feature and workflow is documented
- **Visual Guidance**: UI mockups, decision trees, and diagrams throughout
- **Step-by-Step Instructions**: Detailed, actionable guidance
- **Cross-Platform Focus**: Consistent experience across Windows, iOS, and Android
- **Community-Driven**: Open to contributions and feedback

## 🆘 Getting Help

- **Documentation Issues**: Create an issue in the [GitHub repository](https://github.com/Balexda/WorldFoundry/issues)
- **Feature Questions**: Check [Workflows](workflows/) or [API Reference](api/)
- **Development Help**: See [Development](development/) section
- **Community Support**: Visit [Community Resources](community/SUPPORT.md)

---

**Last Updated**: March 2024  
**Documentation Version**: 1.0  
**Project Repository**: [github.com/Balexda/WorldFoundry](https://github.com/Balexda/WorldFoundry)

*World Foundry is an open-source project. Contributions to documentation are welcome and appreciated.*